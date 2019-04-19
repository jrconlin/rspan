use google::spanner::v1::mutation::{Mutation, Mutation_Delete};
use google::spanner::v1::transaction::{TransactionOptions, TransactionOptions_ReadWrite};
use google::spanner::v1::spanner::{CommitRequest, BeginTransactionRequest, RollbackRequest};

use helpers::{pb_timestamp_to_datetime, make_write_pb};
use keyset::KeySet;
use session::Session;

use chrono::{DateTime, Utc};
use protobuf::well_known_types::Value;
use protobuf::RepeatedField;

/// Implement read-write transaction semantics for a session
pub struct Transaction<'a> {
    session: &'a Session<'a>,
    mutations: Vec<Mutation>,
    commited: Option<Result<DateTime<Utc>, String>>,
    rolled_back: bool,
    transaction_id: Option<Vec<u8>>
}

impl<'a> Transaction<'a> {
    /// Initializes a new Transaction owned by a given `session`.
    ///
    /// # Arguments
    ///
    /// * `session` - The session used to perform the commit.
    ///
    /// # Return value
    ///
    /// A `Transaction` owned by session.
    pub fn new(session: &'a Session<'a>) -> Transaction {
        Transaction {
            session: session,
            commited: None,
            rolled_back: false,
            mutations: Vec::new(),
            transaction_id: None
        }
    }

    /// Getter of transaction id.
    ///
    /// # Return value
    ///
    /// A `Option<&Vec<u8>>` of the transaction id.
    pub fn transaction_id(&self) -> Option<&Vec<u8>> {
        self.transaction_id.as_ref()
    }

    /// Getter of commited.
    ///
    /// # Return value
    ///
    /// A `Option<&Result<DateTime<Utc>, String>>`, if `None` transaction hasn't commited,
    /// if `Some` and `Ok` it'll return the timestamp of commit else an error message.
    pub fn commited(&self) -> Option<&Result<DateTime<Utc>, String>>{
        self.commited.as_ref()
    }

    /// Helper for method [`commit`] et al
    ///
    /// # Panics
    ///
    /// If the object's state is invalid for making API requests.
    ///
    /// [`commit`]: #method.commit
    fn check_state(&self) {
        if self.transaction_id.is_none() {
            panic!("Transaction is not begun");
        }

        if self.commited.is_some() {
            panic!("Transaction is already committed");
        }

        if self.rolled_back {
            panic!("Transaction is already rolled back");
        }
    }

    /// Begin a transaction on the database
    ///
    /// # Return value
    ///
    /// The ID for the newly-begun transaction.
    ///
    /// # Panics
    ///
    /// If the transaction is already begun, committed, or rolled back.
    pub fn begin(mut self) -> Transaction<'a> {
        if self.transaction_id.is_some() {
            panic!("Transaction already begun");
        }

        if self.commited.is_some() {
            panic!("Transaction is already committed");
        }

        if self.rolled_back {
            panic!("Transaction is already rolled back");
        }

        let api = self.session.database().spanner_api();
        let mut req = BeginTransactionRequest::new();
        req.set_session(self.session.name());
        let mut opts = TransactionOptions::new();
        opts.set_read_write(TransactionOptions_ReadWrite::new());
        req.set_options(opts);
        let res = api.begin_transaction(&req).unwrap();
        self.transaction_id = Some(res.get_id().to_vec());
        self
    }

    /// Roll back a transaction on the database
    pub fn rollback(mut self) {
        self.check_state();
        {
            let api = self.session.database().spanner_api();
            let mut req = RollbackRequest::new();
            req.set_session(self.session.name());
            match self.transaction_id {
                Some(ref id) => { req.set_transaction_id(id.to_vec()); },
                None => { }
            };
            api.rollback(&req).unwrap();
        }
        self.rolled_back = true;
    }

    /// Insert one or more new table rows.
    ///
    /// # Arguments
    ///
    /// * `table` - Name of table to be modified.
    ///
    /// * `columns` - Name of table columns to be modified.
    ///
    /// * `values` - Values to be modified.
    pub fn insert(&mut self, table: String, columns: Vec<String>, values: Vec<Vec<Value>>) {
        let mut m = Mutation::new();
        m.set_insert(make_write_pb(table, columns, values));
        self.mutations.push(m);
    } 

    /// Update one or more existing table rows.
    ///
    /// # Arguments
    ///
    /// * `table` - Name of table to be modified.
    ///
    /// * `columns` - Name of table columns to be modified.
    ///
    /// * `values` - Values to be modified.
    pub fn update(&mut self, table: String, columns: Vec<String>, values: Vec<Vec<Value>>) {
        let mut m = Mutation::new();
        m.set_update(make_write_pb(table, columns, values));
        self.mutations.push(m);
    } 

    /// Insert/update one or more table rows.
    ///
    /// # Arguments
    ///
    /// * `table` - Name of table to be modified.
    ///
    /// * `columns` - Name of table columns to be modified.
    ///
    /// * `values` - Values to be modified.
    pub fn upsert(&mut self, table: String, columns: Vec<String>, values: Vec<Vec<Value>>) {
        let mut m = Mutation::new();
        m.set_insert_or_update(make_write_pb(table, columns, values));
        self.mutations.push(m);
    } 

    /// Replace one or more table rows.
    ///
    /// # Arguments
    ///
    /// * `table` - Name of table to be modified.
    ///
    /// * `columns` - Name of table columns to be modified.
    ///
    /// * `values` - Values to be modified.
    pub fn replace(&mut self, table: String, columns: Vec<String>, values: Vec<Vec<Value>>) {
        let mut m = Mutation::new();
        m.set_replace(make_write_pb(table, columns, values));
        self.mutations.push(m);;
    } 

    /// Delete one or more table rows.
    ///
    /// # Arguments
    ///
    /// * `table` - Name of table to be modified.
    ///
    /// * `keyset` - Keys/ranges identifying rows to delete.
    pub fn delete(&mut self, table: String, keyset: KeySet) {
        let mut delete = Mutation_Delete::new();
        delete.set_table(table);
        delete.set_key_set(keyset.to_pb());
        let mut m = Mutation::new();
        m.set_delete(delete);
        self.mutations.push(m);
    }

    /// Commit mutations to the database.
    ///
    /// # Panics
    ///
    /// If there are no mutation to commit.
    pub fn commit(&mut self) {
        self.check_state();
        if self.mutations.is_empty() {
            panic!("No mutation to commit");
        }
        let api = self.session.database().spanner_api();
        let mut req = CommitRequest::new();
        req.set_session(self.session.name());
        req.set_mutations(RepeatedField::from_vec(self.mutations.clone()));
        match self.transaction_id {
            Some(ref id) => { req.set_transaction_id(id.to_vec()); },
            None => { }
        }
        let mut response = api.commit(&req).unwrap();
        if response.has_commit_timestamp() {
            self.commited = Some(Ok(pb_timestamp_to_datetime(response.take_commit_timestamp())));
        } else {
            self.commited = Some(Err("Couldn't commit transaction.".to_string()));
        }
    }
}