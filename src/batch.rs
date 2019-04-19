use ::google::spanner::v1::transaction::{TransactionOptions, TransactionOptions_ReadWrite};
use ::google::spanner::v1::mutation::{Mutation, Mutation_Delete};
use ::google::spanner::v1::spanner::{CommitRequest};

use helpers::{pb_timestamp_to_datetime, make_write_pb};
use session::Session;
use keyset::KeySet;

use protobuf::well_known_types::Value;
use protobuf::{RepeatedField};
use chrono::{Utc, DateTime};

/// Accumulate mutations for transmission during method [`commit`].
///
/// [`commit`]: #method.commit
pub struct Batch<'a> {
    session: &'a Session<'a>,
    mutations: Vec<Mutation>,
    commited: Option<DateTime<Utc>>
}

impl<'a> Batch<'a> {
    /// Initializes a new Batch owned by a given `session`.
    ///
    /// # Arguments
    ///
    /// * `session` - The session used to perform the commit.
    ///
    /// # Return value
    ///
    /// A `Batch` owned by session.
    pub fn new(session: &'a Session<'a>) -> Self {
        Batch {
            session: session,
            mutations: Vec::new(),
            commited: None
        }
    }

    /// Helper for method [`commit`] et al.
    ///
    /// # Panics
    ///
    /// If batch already commited
    ///
    /// [`commit`]: #method.commit
    fn check_state(&self) {
        if self.commited.is_none() {
            panic!("Batch already commited");
        }
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

    /// Commi mutations to the database.
    ///
    /// # Return value
    ///
    /// A `DateTime<Utc>` representing the timesamp of the committed changes.
    pub fn commit(&mut self) -> DateTime<Utc> {
        self.check_state();
        let database = self.session.database();
        let api = database.spanner_api();
        let mut req = CommitRequest::new();
        req.set_session(self.session.name());
        req.set_mutations(RepeatedField::from_vec(self.mutations.clone()));
        let mut opts = TransactionOptions::new();
        opts.set_read_write(TransactionOptions_ReadWrite::new());
        req.set_single_use_transaction(opts);
        let mut response = api.commit(&req).unwrap();
        self.commited = Some(pb_timestamp_to_datetime(response.take_commit_timestamp()));
        self.commited.unwrap()
    }
}