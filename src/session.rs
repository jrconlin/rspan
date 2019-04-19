use google::spanner::v1::spanner::{DeleteSessionRequest, GetSessionRequest, CreateSessionRequest};
use google::spanner::v1::spanner;

use database::Database;
use transaction::Transaction;
use batch::Batch;
use snapshot::Snapshot;

use chrono::{DateTime, Utc};
use regex::Regex;
use std::fmt;

/// Representation of a Cloud Spanner Session.
///
/// We can use `Session` to:
/// * `create` the session.
/// * Use method `exists` to check for the existence of the session
/// * Method `drop` the session
pub struct Session<'a> {
    database: &'a Database<'a>,
    id: Option<String>,
}

impl<'a> Session<'a> {
    /// Initializes a new session owned by a given `database`
    ///
    /// # Arguments
    ///
    /// * `database` - The database that owns the session.
    ///
    /// # Return value
    ///
    /// A `Session` owned by database.
    pub fn new(database: &'a Database<'a>) -> Self {
        Session {
            database: database,
            id: None,
        }
    }

    /// Getter of session id, set by the back-end during method [`create`].
    ///
    /// # Return value
    ///
    /// A `Option<&String>` of the session id.
    ///
    /// [`create`]: #method.create
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Session name used in requests.
    ///
    /// The session name is of the form:
    ///
    /// `projects/../instances/../databases/../sessions/{session_id}`
    ///
    /// # Return value
    ///
    /// A `String` representing the session name.
    ///
    /// # Panics
    /// 
    /// If session is not yet created.
    pub fn name(&self) -> String {
        if self.id.is_none() {
            panic!("No session ID set by back end.");
        }
        self.database.name() + "/sessions/" + self.id.as_ref().unwrap()
    }

    /// Helper for transaction-related API calls.    
    pub fn database(&'a self) -> &'a Database<'a> {
        self.database
    }

    /// Create this session, bound to its database.
    ///
    /// See [RPC docs]
    ///
    /// # Panics 
    ///
    /// If `session_id` is already set.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/reference/rpc/google.spanner.v1#google.spanner.v1.Spanner.CreateSession
    pub fn create(&mut self) {
        if self.id.is_some() {
            panic!("Session ID already set by back-end.");
        }

        let api = self.database.spanner_api();
        let mut req = CreateSessionRequest::new();
        req.set_database(self.database.name());
        let session_pb = api.create_session(&req).unwrap();
        let splits: Vec<&str> = session_pb.get_name().split("/").collect();
        self.id = Some(splits[7].to_string());
    }

    /// Test for the existence of this session.
    ///
    /// See [RPC docs]
    ///
    /// # Return value
    ///
    /// True if the session exists on the back-end, else False.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/reference/rpc/google.spanner.v1#google.spanner.v1.Spanner.GetSession
    pub fn exists(&self) -> bool {
        if self.id.is_none() {
            return false
        }

        let api = self.database.spanner_api();;
        let mut req = GetSessionRequest::new();
        req.set_name(self.name());
        let res = api.get_session(&req);
        res.is_ok()
    }

    /// Delete this session.
    ///
    /// See [RPC docs]
    ///
    /// # Return value
    ///
    /// True if the session was deleted, else False.
    ///
    /// # Panics 
    ///
    /// If `session_id` is already set.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.v1#google.spanner.v1.Spanner.DeleteSession
    pub fn delete(&mut self) -> bool {
        if self.id.is_none() {
            panic!("Session ID not set by back end.");
        }

        let api = self.database.spanner_api();
        let mut req = DeleteSessionRequest::new();
        req.set_name(self.name());
        let res = api.delete_session(&req);
        res.is_ok()
    }

    /// Create a snapshot to perform a set of reads with shared staleness.
    ///
    /// See [RPC docs]
    ///
    /// # Arguments
    ///
    /// * `read_timestamp` - (Optional) Execute all read at the given timestamp.
    ///
    /// * `min_read_timestamp` - (Optional) Execute all reads at a timestamp >= `min_read_timestamp`.
    ///
    /// * `max_staleness` - (Optional) Read data at a timestamp >= NOW - `max_staleness` seconds.
    ///
    /// * `exact_staleness` - (Optional) Execute all reads at a timestamp that is `exact_staleness` old.
    ///
    /// * `multi_use` - If true, multiple method [`read`]/[`execute_sql`] calls can be performed
    ///                 with the snapshot in the context of a read-only transaction, used to ensure
    ///                 isolation/consistency. Incompatible with `max_staleness` and `min_read_timestamp`. 
    ///
    /// # Return value
    ///
    /// A [`Snapshot`] bound to this session.
    ///
    /// # Panics
    ///
    /// If session has not been created
    ///
    /// [`Snapshot`]: ../snapshot/struct.Snapshot.html
    /// [`read`]: ../snapshot/struct.Snapshot.html#method.read
    /// [`execute_sql`]: ../snapshot/struct.Snapshot.html#method.execute_sql
    /// [RPC docs]: https://cloud.google.com/spanner/reference/rpc/google.spanner.v1#google.spanner.v1.TransactionOptions.ReadOnly
    pub fn snapshot(&'a self, read_timestamp: Option<DateTime<Utc>>, min_read_timestamp: Option<DateTime<Utc>>, max_staleness: Option<DateTime<Utc>>, exact_staleness: Option<DateTime<Utc>>, multi_use: bool) -> Snapshot {
        if self.id.is_none() {
            panic!("Session has not been created.");
        }

        Snapshot::new(self, read_timestamp, min_read_timestamp, max_staleness, exact_staleness, multi_use)
    }

    /// Factory to create a batch for this session.
    ///
    /// # Return value
    ///
    /// A [`Batch`] bound to this session.
    ///
    /// # Panics
    ///
    /// If the session has not yet been created.
    ///
    /// [`Batch`]: ../batch/struct.Batch.html
    pub fn batch(&'a self) -> Batch {
        if self.id.is_none() {
            panic!("Session has not been created");
        }

        Batch::new(self)
    }

    /// Create a transaction to perform a set of read with shared staleness
    ///
    /// # Return value
    ///
    /// A [`Transaction`] bound to this session.
    ///
    /// # Panics 
    ///
    /// If the session has not been created.
    ///
    /// [`Transaction`]: ../transaction/struct.Transaction.html
    fn transaction(&'a self) -> Transaction {
        if self.id().is_none() {
            panic!("Session has not been created");
        };

        Transaction::new(self)
    }

    /// Perform a unit of work in a transaction.
    ///
    /// # Arguments
    ///
    /// * `func` - A clousure takes a required argument, the transaction, and
    ///             returns the same transaction after applying mutations.
    ///
    /// # Return value
    ///
    /// A new `Result` where if the transaction was succesfull will return a 
    /// `DateTime<Utc>` corrsponding to the timestamp of committed transaction
    /// else it'll return a `String` on error.
    pub fn run_in_transaction<F>(&'a self, func: F) -> Result<DateTime<Utc>, String> 
        where F: for<'b> Fn(Transaction<'b>) -> Transaction<'b>
    {        
        let mut txn = self.transaction().begin();
        txn = func(txn);
        let mut result = Err(String::from("Couldn't get result"));
        match txn.commited() {
            None => { },
            Some(r) => {
                match r {
                    Err(m) => {
                        result = Err(m.to_string());
                    },
                    Ok(v) => {
                        result = Ok(v.clone());
                    }
                }
            }
        }
        if result.is_err() {
            txn.rollback();
        }
        result
    }

    /// Creates an instance of this struct from a protobuf
    ///
    /// # Arguments
    ///
    /// * `session_pb` - A session protobuf object.
    ///
    /// * `database` - The database that owns the session.
    ///
    /// # Return value
    ///
    /// The `Session` parsed from the protobuf response.
    ///
    /// # Panics
    ///
    /// * If the session name does not match expected format
    /// * If the parsed project ID does not match the project ID
    ///   on the instance's client.
    /// * If the parsed instance ID does not match the instance ID
    ///   on the database's instance.
    /// * If the parsed database ID does not match the database's ID.
    pub fn from_pb(session_pb: &spanner::Session, database: &'a Database<'a>) -> Session<'a> {
        let re = Regex::new(r"^projects/(?P<project>[^/]+)/instances/(?P<instance_id>[a-z][-a-z0-9]*)/databases/(?P<database_id>[^/]+)/sessions/(?P<id>[^/]+)$").unwrap();
        let caps = re.captures(&session_pb.name);
        if caps.is_none() {
            panic!("Session protobuf name was not in the expected format: {}", session_pb.name);
        }
        let c = caps.unwrap();
        if &c["project"] != database.instance().client().id() {
            panic!("Project ID on the database does not match the project ID on the instance's client");
        }
        if &c["instance_id"] != database.instance().id() {
            panic!("Instance ID does not match the Instance ID on the database's instance object");
        }
        if &c["database_id"] != database.id() {
            panic!("Database ID does not match the Database ID on the database object");
        }
        let session_id = &c["id"];
        Session {
            id: Some(session_id.to_string()),
            database: database
        }
    }
}

impl<'a> fmt::Debug for Session<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Session {{ id: {:?}, database: {:?}}}", self.id, self.database)
    }
}

/// Helper struct used when listing sessions.
pub struct ListSessions<'a> {
    pub sessions: Vec<Session<'a>>,
    pub next_page_token: String
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::Client;
    use instance::Instance;
    use keyset::KeySet;
    use protobuf::well_known_types::Value;

    #[test]
    fn init() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = Instance::new(String::from("archived"), &client, None, None, None);
        let mut db = Database::new(String::from("testdata"), &instance, None);
        let mut session = db.session();
        assert_eq!(session.exists(), false);
        session.create();
        assert_eq!(session.exists(), true);
        assert_eq!(session.name(), String::from("projects/rusty-206403/instances/archived/databases/testdata/sessions/") + session.id().unwrap());
        assert_eq!(session.delete(), true);
        assert_eq!(session.exists(), false);
    }

    #[test]
    fn transaction() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = Instance::new(String::from("archived"), &client, None, None, None);
        let mut db = Database::new(String::from("new_db"), &instance, None);
        let mut session = db.session();
        session.create();
        let result = session.run_in_transaction(|mut txn| {
            let table = String::from("main_table");
            let mut key = Value::new();
            key.set_string_value(String::from("test"));
            let mut val = Value::new();
            val.set_string_value(String::from("27"));
            let columns = vec![String::from("key"), String::from("value")];
            txn.insert(table.clone(), columns.clone(), vec![vec![key.clone(), val.clone()]]);
            val.set_string_value(String::from("28"));
            txn.update(table.clone(), columns.clone(), vec![vec![key.clone(), val.clone()]]);
            key.set_string_value(String::from("testd"));
            val.set_string_value(String::from("11"));
            txn.upsert(table.clone(), columns.clone(), vec![vec![key.clone(), val.clone()]]);
            let keyset = KeySet::new(Some(vec![String::from("testd")]), None, None);
            txn.delete(table.clone(), keyset);
            txn.commit();
            txn
        });
        session.delete();
        assert_eq!(result.is_ok(), true);
    }
}