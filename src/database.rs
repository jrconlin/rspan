use instance::Instance;
use keyset::KeySet;
use operation::{Operation};
use snapshot::Snapshot;
use session::{Session, ListSessions};
use streamed::StreamedResultSet;
use transaction::Transaction;
use operation::OperationResult;
use helpers::{QueryMode, Type};

use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Arc;
use std::fmt;
use chrono::{DateTime, Utc};
use grpcio::{ChannelBuilder, EnvBuilder, ChannelCredentials};
use protobuf::RepeatedField;
use protobuf::well_known_types::Value;
use regex::Regex;

use google::spanner::admin::database::v1::spanner_database_admin::{DropDatabaseRequest, UpdateDatabaseDdlRequest, GetDatabaseDdlRequest, CreateDatabaseRequest};
use google::spanner::admin::database::v1::spanner_database_admin;
use google::spanner::v1::spanner::{ListSessionsRequest};
use google::spanner::v1::spanner_grpc::SpannerClient;

thread_local! {
    /// Local thread used to check if a transaction is running or not.
    pub static TRANSACTION_RUNNING: RefCell<bool>
        = RefCell::new(false);
}

/// Representation of a Cloud Spanner Database.
///
/// We can use `Database` to:
/// * `create` itself
/// * `reload` itself
/// * `update` itself
/// * `drop` itself
pub struct Database<'a> {
    id: String,
    instance: &'a Instance<'a>,
    ddl_statements: Option<Vec<String>>,
    spanner_api: SpannerClient
}

impl<'a> Database<'a> {
    /// Initializes a new Database owned by a given `Instance`
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the database.
    ///
    /// * `instance` - The instance that owns the database
    ///
    /// * `ddl_statements` - (Optional) DDL statements, excluding the CREATE DATABASE statement.
    ///
    /// # Return value
    ///
    /// A `Database` owned by instance.
    pub fn new(id: String, instance: &'a Instance<'a>, ddl_statements: Option<Vec<String>>) -> Self {
        let credentials = ChannelCredentials::google_default_credentials().unwrap();

        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).secure_connect("spanner.googleapis.com", credentials);

        Database {
            id: id,
            instance: instance,
            ddl_statements: ddl_statements,
            spanner_api: SpannerClient::new(ch)
        }
    }

    /// Database name used in requests.
    ///
    /// The database name is of the form:
    ///
    /// `projects/../instances/../databases/{database_id}`
    ///
    /// # Return value
    ///
    /// A `String` representing the database name
    pub fn name(&self) -> String {
        self.instance.name() + "/databases/" + &self.id
    }

    /// Getter of database id.
    ///
    /// # Return value
    ///
    /// A `&String` of the database id.
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Helper for session-related API calls.
    pub fn spanner_api(&self) -> &SpannerClient {
        &self.spanner_api
    }

    /// Helper for session-related API calls.
    pub fn instance(&self) -> &Instance {
        self.instance
    }

    /// DDL Statements used to define database schema
    ///
    /// See [Data Definition Language]
    ///
    /// # Return value
    ///
    /// The statements in `&Option<Vec<String>>`.
    ///
    /// [Data Definition Language]: https://cloud.google.com/spanner/docs/data-definition-language
    pub fn ddl_statements(&self) -> &Option<Vec<String>> {
        &self.ddl_statements
    }

    /// Create this database within its instance.
    ///
    /// Includes any configured schema assigned to `ddl_statements`.
    ///
    /// See [RPC docs]
    ///
    /// # Return value
    ///
    /// An [`Operation`] used to poll the status of the create request.
    ///
    /// # Panics
    ///
    /// * If the database already exists.
    /// * If the instance owning the database does not exist.
    ///
    /// [`Operation`]: ../operation/struct.Operation.html
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.database.v1#google.spanner.admin.database.v1.DatabaseAdmin.CreateDatabase
    pub fn create(&self) -> Operation {
        let api = self.instance.client().database_admin_api();
        let mut db_name = self.id.clone();
        if db_name.contains("-") {
            db_name = format!(" {} ", db_name);
        }
        
        let mut req = CreateDatabaseRequest::new();
        req.set_parent(self.instance.name());
        req.set_create_statement(format!("CREATE DATABASE {}", db_name));
        match self.ddl_statements {
            Some(ref st) => { req.set_extra_statements(RepeatedField::from_vec(st.to_vec())); },
            None => { }
        }
        let res = api.create_database(&req);
        match res {
            Err(e) => {
                Operation {
                    name: String::from("Error"),
                    done: true,
                    result: OperationResult::Error(e)
                }
            },
            Ok(mut op) => {
                Operation {
                    name: op.take_name(),
                    done: op.get_done(),
                    result: OperationResult::Response
                }
            }
        }
    }

    /// Test whether this database exists.
    ///
    /// See [RPC docs]
    ///
    /// # Return value
    ///
    /// True if the database exists, else false.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.database.v1#google.spanner.admin.database.v1.DatabaseAdmin.GetDatabase
    pub fn exists(&self) -> bool {
        let api = self.instance.client().database_admin_api();
        let mut req = GetDatabaseDdlRequest::new();
        req.set_database(self.name());

        let res = api.get_database_ddl(&req);
        match res {
            Ok(_) => { true },
            _ => { false }
        }
    }

    /// Reload this database.
    ///
    /// Refresh any configured schema into `ddl_statements`.
    ///
    /// See [RPC docs]
    ///
    /// # Panics
    ///
    /// If the database does not exist
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.database.v1#google.spanner.admin.database.v1.DatabaseAdmin.GetDatabase
    pub fn reload(&mut self) {
        let api = self.instance.client().database_admin_api();
        let mut req = GetDatabaseDdlRequest::new();
        req.set_database(self.name());

        let mut res = api.get_database_ddl(&req).unwrap();
        self.ddl_statements = Some(res.take_statements().into_vec());
    }

    /// Update DDL for this database.
    ///
    /// Apply any configured schema from `ddl_statements`.
    ///
    /// See [RPC docs].
    ///
    /// # Arguments
    ///
    /// * `ddl_statements` - A vector of DDL statements to use on this database.
    ///
    /// # Panics
    ///
    /// * If the database not exist.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.database.v1#google.spanner.admin.database.v1.DatabaseAdmin.UpdateDatabaseDdl
    pub fn update_ddl(&self, ddl_statements: Vec<String>) -> Operation {
        let api = self.instance.client().database_admin_api();
        let mut req = UpdateDatabaseDdlRequest::new();
        req.set_database(self.name());
        req.set_statements(RepeatedField::from_vec(ddl_statements));

        let res = api.update_database_ddl(&req);
        match res {
            Err(e) => {
                Operation {
                    name: String::from("Error"),
                    done: true,
                    result: OperationResult::Error(e)
                }
            },
            Ok(mut op) => {
                Operation {
                    name: op.take_name(),
                    done: op.get_done(),
                    result: OperationResult::Response
                }
            }
        }
    }

    /// Drop this database.
    ///
    /// See [RPC docs].
    ///
    /// [RPC docs: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.database.v1#google.spanner.admin.database.v1.DatabaseAdmin.DropDatabase]
    pub fn drop(&self) {
        let api = self.instance.client().database_admin_api();
        let mut req = DropDatabaseRequest::new();
        req.set_database(self.name());

        api.drop_database(&req).unwrap();
    }

    /// Factory to create a session for this instance.
    ///
    /// #Arguments
    ///
    /// A [`Session`] bound to this database.
    ///
    /// [`Session`]: ../session/struct.Session.html
    pub fn session(&'a self) -> Session {
        Session::new(self)
    }

    /// Return and object which wraps a batch read/query.
    ///
    /// # Arguments
    ///
    /// * `read_timestamp` - Execute all reads at the given timestamp.
    ///
    /// * `exact_staleness` - Execute all reads at a timestamp that is `exact_staleness` old.
    ///
    /// # Return value
    ///
    /// A new [`BatchSnapshot`] wrapper.
    ///
    /// [`BatchSnapshot`]: ../database/struct.BatchSnapshot.html
    pub fn batch_snapshot(&'a mut self, read_timestamp: Option<DateTime<Utc>>, exact_staleness: Option<DateTime<Utc>>) -> BatchSnapshot {
        BatchSnapshot::new(self, read_timestamp, exact_staleness)
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
    pub fn run_in_transaction<F>(&'a mut self, func: F) -> Result<DateTime<Utc>, String> 
        where F: for<'b> Fn(Transaction<'b>) -> Transaction<'b>{
        TRANSACTION_RUNNING.with(|running| {
            if *running.borrow() {
                panic!("Spanner does not suppot nested transactions.");
            } else {
                *running.borrow_mut() = true;
            }
        });

        let result;
        {
            let mut session = self.session();
            result = session.run_in_transaction(func);
            session.delete();
        }

        TRANSACTION_RUNNING.with(|running| {
            *running.borrow_mut() = false;
        });

        result
    }

    /// Creates an instance of this struct from a protobuf
    ///
    /// # Arguments
    ///
    /// * `database_pb` - A database protobuf object.
    ///
    /// * `instance` - The instance that owns the database.
    ///
    /// # Return value
    ///
    /// The `Database` parsed from the protobuf response.
    ///
    /// # Panics
    ///
    /// * If the database name does not match expected format
    /// * If the parsed project ID does not match the project ID
    ///   on the instance's client.
    /// * If the parsed instance ID does not match the instance's ID.
    pub fn from_pb(database_pb: &spanner_database_admin::Database, instance: &'a Instance<'a>) -> Database<'a> {
        let re = Regex::new(r"^projects/(?P<project>[^/]+)/instances/(?P<instance_id>[a-z][-a-z0-9]*)/databases/(?P<id>[^/]+)$").unwrap();
        let caps = re.captures(&database_pb.name);
        if caps.is_none() {
            panic!("Database protobuf name was not in the expected format: {}", database_pb.name);
        }
        let c = caps.unwrap();
        if &c["project"] != instance.client().id() {
            panic!("Project ID on the database does not match the project ID on the instance's client");
        }
        if &c["instance_id"] != instance.id() {
            panic!("Instance ID does not match the Instance ID on the instance object");
        }
        let id = &c["id"];
        Database::new(id.to_string(), instance, None)
    }

    /// List sessions for the database.
    ///
    /// See [RPC docs]
    ///
    /// # Arguments
    ///
    /// * `filter` - (Optional) Filter to select sessions listed. See the `ListSessionRequest` docs above for examples.
    ///
    /// * `page_size` - (Optional) Maximum number of results to return.
    ///
    /// * `page_token` - (Optional) Token for fetching next page of results.
    ///
    /// # Return value
    ///
    /// A [`ListSessions`] struct with the result.
    ///
    /// [`ListSessions`]: ../session/struct.ListSessions.html
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.v1#google.spanner.v1.Spanner.ListSessions
    pub fn list_sessions(&self, filter: Option<String>, page_size: Option<i32>, page_token: Option<String>) -> ListSessions {
        let mut req = ListSessionsRequest::new();
        req.set_database(self.name());
        match filter {
            Some(f) => { req.set_filter(f); },
            None => { }
        }
        match page_size {
            Some(p) => { req.set_page_size(p); },
            None => { }
        }
        match page_token {
            Some(p) => { req.set_page_token(p); },
            None => { }
        }
        let mut res = self.spanner_api.list_sessions(&req).unwrap();
        let mut sessions = Vec::new();
        for x in res.take_sessions().into_vec().iter() {
            sessions.push(Session::from_pb(x, self));
        }
        ListSessions {
            sessions: sessions,
            next_page_token: res.take_next_page_token()
        }
    }
}

impl<'a> fmt::Debug for Database<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database {{ id: {}, instance: {:?} , ddl_statements: {:?}}}", self.id, self.instance, self.ddl_statements)
    }
}

/// Helper struct used when listing databases.
pub struct ListDatabases<'a> {
    pub databases: Vec<Database<'a>>,
    pub next_page_token: String
}

/// Wrapper for generating and processing read/query batches.
pub struct BatchSnapshot<'a> {
    database: &'a mut Database<'a>,
    session: Option<Session<'a>>,
    snapshot: Option<Snapshot<'a>>,
    read_timestamp: Option<DateTime<Utc>>,
    exact_staleness: Option<DateTime<Utc>>
}

impl<'a> BatchSnapshot<'a> {
    /// Initializes a new `BatchSnapshot`
    ///
    /// # Arguments
    ///
    /// * `database` - Database to use.
    ///
    /// * `read_timestamp` - (Optional) Execute all reads at the given timestamp.
    ///
    /// * `exact_staleness` - (Optional) Execute all reads at a timestamp that is `exact_staleness` old.
    ///
    /// # Return value
    ///
    /// A `BatchSnapshot`.
    pub fn new(database: &'a mut Database<'a>, read_timestamp: Option<DateTime<Utc>>, exact_staleness: Option<DateTime<Utc>>) -> Self {
        BatchSnapshot {
            database: database,
            session: None,
            snapshot: None,
            read_timestamp: read_timestamp,
            exact_staleness: exact_staleness
        }
    }

    /// Create snapshot if needed
    ///
    /// Caller is responsible for cleaning up the session after all.
    fn get_snapshot(&'a mut self) -> &'a mut Snapshot<'a> {
        match self.snapshot {
            Some(ref mut s) => { s },
            None => {
                let session = match self.session {
                    Some(ref s) => { s },
                    None => {
                        self.session = Some(self.database.session());
                        match self.session {
                            Some(ref mut s) => { 
                                if s.id().is_none() {
                                    s.create();
                                }
                            }
                            None => { }
                        }
                        self.session.as_ref().unwrap()
                    }
                };
                self.snapshot = Some(session.snapshot(self.read_timestamp.clone(), None, None, self.exact_staleness.clone(), true));
                self.snapshot.as_mut().unwrap().begin();
                self.snapshot.as_mut().unwrap()
            }
        }
    }

    /// Convenience method: perform read operation via snapshot.
    ///
    /// See method [`read`]
    ///
    /// [`read`]: ../snapshot/struct.Snapshot.html#method.read
    pub fn read(&'a mut self, table: String, columns: Vec<String>, keyset: KeySet, index: Option<String>, limit: Option<i64>, partition: Option<Vec<u8>>) -> StreamedResultSet {
        self.get_snapshot().read(table, columns, keyset, index, limit, partition)
    }

    /// Convenience method: perform query operation via snapshot.
    ///
    /// See method [`execute_sql`]
    ///
    /// [`execute_sql`]: ../snapshot/struct.Snapshot.html#method.execute_sql
    pub fn execute_sql(&'a mut self, sql: String, params: Option<HashMap<String, Value>>, param_types: Option<HashMap<String, Type>>, query_mode: Option<QueryMode>, partition: Option<Vec<u8>>) -> StreamedResultSet {
        self.get_snapshot().execute_sql(sql, params, param_types, query_mode, partition)
    }

    /// Start a partitioned batch read operation.
    ///
    /// Uses the `PartitionRead` API request to initiate the partitioned
    /// read. Returns a list of batch information needed to perform the
    /// actual read.
    ///
    /// # Arguments
    ///
    /// * `table` - Name of the table from which to fetch data.
    ///
    /// * `columns` - Names of columns to be retrieved.
    ///
    /// * `keyset` - A [`KeySet`] containing keys/ranges identifying rows
    ///              to be retrieved.
    ///
    /// * `index` - (Optional) Name of index to use, rather than the table's
    ///             primary key.
    ///
    /// * `partition_size_bytes` - (Optional) Desired size for each partition generated.
    ///                            The service uses this as a hint, th actual partition size
    ///                            may differ.
    ///
    /// * `max_partitions` - (Optional) Desired maximum number of partitions generated. The
    ///                      service uses this as a hint, the actual number of partitions may
    ///                      differ.
    ///
    /// # Return value
    ///
    /// A vector of [`PartitionRead`].
    ///
    /// [`PartitionRead`]: ../database/struct.PartitionRead.html
    /// [`KeySet`]: ../keyset/struct.KeySet.html
    pub fn generate_read_batches(&'a mut self, table: String, columns: Vec<String>, keyset: KeySet, index: Option<String>, partition_size_bytes: Option<i64>, max_partitions: Option<i64>) -> Vec<BatchType> {
        let partitions = self.get_snapshot().partition_read(table.clone(), columns.clone(), keyset.clone(), index.clone(), partition_size_bytes.clone(), max_partitions.clone());
        let mut result = Vec::new();
        for p in partitions {
            result.push(BatchType::Read(PartitionRead {
                partition: p,
                table: table.clone(),
                columns: columns.clone(),
                keyset: keyset.clone(),
                index: index.clone().unwrap_or(String::from(""))
            }));
        }
        result
    }

    /// Process a single, partitioned read.
    ///
    /// # Arguments
    ///
    /// * `batch` - One of the [`PartitionRead`] returned from an earlier
    ///             call to method [`generate_read_batches`]
    ///
    /// # Return value
    ///
    /// A [`StreamedResultSet`] instance which can be used to consume rows.
    ///
    /// [`PartitionRead`]: ../database/struct.PartitionRead.html
    /// [`generate_read_batches`]: #method.generate_read_batches
    /// [`StreamedResultSet`]: ../streamed/struct.StreamedResultSet.html
    pub fn process_read_batch(&'a mut self, batch: PartitionRead) -> StreamedResultSet {
        self.get_snapshot().read(batch.table, batch.columns, batch.keyset, Some(batch.index), None, Some(batch.partition))
    }

    /// Start a partitioned batch query operation.
    ///
    /// Uses the `PartitionQuery` API request to start a partitioned
    /// query operation. Returns a list of batch information needed to perform the
    /// perform the actual queries.
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL query statement.
    ///
    /// * `params` - Values for parameter replacement. Keys must match
    ///              the names used in `sql`.
    ///
    /// * `param_types` - (Optional) Maps explicit types for one or more param
    ///                   values; required if parameters are passed.
    ///
    /// * `partition_size_bytes` - (Optional) Desired size for each partition generated.
    ///                            The service uses this as a hint, th actual partition size
    ///                            may differ.
    ///
    /// * `max_partitions` - (Optional) Desired maximum number of partitions generated. The
    ///                      service uses this as a hint, the actual number of partitions may
    ///                      differ.
    ///
    /// # Return value
    ///
    /// A vector of [`PartitionQuery`].
    ///
    /// [`PartitionQuery`]: ../database/struct.PartitionQuery.html
    pub fn generate_query_batches(&'a mut self, sql: String, params: Option<HashMap<String, Value>>, param_types: Option<HashMap<String, Type>>, partition_size_bytes: Option<i64>, max_partitions: Option<i64>) -> Vec<BatchType> {
        let partitions = self.get_snapshot().partition_query(sql.clone(), params.clone(), param_types.clone(), partition_size_bytes.clone(), max_partitions.clone());
        let mut result = Vec::new();
        for p in partitions {
            result.push(BatchType::Query(PartitionQuery {
                partition: p,
                sql: sql.clone(),
                params: params.clone(),
                param_types: param_types.clone()
            }));
        }
        result
    }

    /// Process a single, partitioned query.
    ///
    /// # Arguments
    ///
    /// * `batch` - One of the [`PartitionQuery`] returned from an earlier
    ///             call to method [`generate_query_batches`]
    ///
    /// # Return value
    ///
    /// A [`StreamedResultSet`] instance which can be used to consume rows.
    ///
    /// [`PartitionQuery`]: ../database/struct.PartitionQuery.html
    /// [`generate_query_batches`]: #method.generate_query_batches
    /// [`StreamedResultSet`]: ../streamed/struct.StreamedResultSet.html
    pub fn process_query_batch(&'a mut self, batch: PartitionQuery) -> StreamedResultSet {
        self.get_snapshot().execute_sql(batch.sql, batch.params, batch.param_types, None, Some(batch.partition))
    }

    /// Process a single, partitioned query or read.
    ///
    /// # Arguments
    ///
    /// * `batch` - One of the [`BatchType`] returned from an earlier call
    ///             to method [`generate_query_batches`] or method [`generate_read_batches`].
    ///
    /// # Return value
    ///
    /// A [`StreamedResultSet`] instance which can be used consume rows.
    ///
    /// [`BatchType`]: ../database/enum.BatchType.html
    /// [`generate_query_batches`]: #method.generate_query_batches
    /// [`generate_read_batches`]: #method.generate_read_batches
    /// [`StreamedResultSet`]: ../streamed/struct.StreamedResultSet.html
    pub fn process(&'a mut self, batch: BatchType) -> StreamedResultSet {
        match batch {
            BatchType::Query(b) => { self.process_query_batch(b) },
            BatchType::Read(b) => { self.process_read_batch(b) }
        }
    }

    /// Clean up underlying session.
    ///
    /// If the transaction has been shared across multiples machines,
    /// calling this on any machine would invalidate the transaction
    /// everywhere. Ideally this would be called when data has been
    /// read from all partitions.
    pub fn close(&'a mut self) {
        match self.session {
            Some(ref mut s) => { s.delete(); }
            None => { }
        }
        self.session = None;
    }
}

/// Helper struct used when processing partition read batches.
pub struct PartitionRead {
    pub partition: Vec<u8>,
    pub table: String,
    pub columns: Vec<String>,
    pub keyset: KeySet,
    pub index: String
}

/// Helper struct used when processing partition query batches.
pub struct PartitionQuery {
    pub partition: Vec<u8>,
    pub sql: String,
    pub params: Option<HashMap<String, Value>>,
    pub param_types: Option<HashMap<String, Type>>
}

/// Helper enum used to differentiate between Query or Read batches.
pub enum BatchType {
    Query(PartitionQuery),
    Read(PartitionRead)
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::Client;

    #[test]
    fn init() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = Instance::new(String::from("archived"), &client, None, None, None);
        let db = Database::new(String::from("testdata"), &instance, None);
        assert_eq!(db.name(), instance.name() + "/databases/testdata");
    }

    #[test]
    fn create_database() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = client.instance(String::from("archived"), None, None, None);
        let db = instance.database(String::from("new_db"), None);
        let op = db.create();
        match op.result {
            OperationResult::Response => {
                assert_eq!(db.exists(), true);
            },
            OperationResult::Error(_) => { }
        }
        assert_eq!(db.name(), String::from("projects/rusty-206403/instances/archived/databases/new_db"));
    }

    #[test]
    fn exists() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = client.instance(String::from("archived"), None, None, None);
        let db = instance.database(String::from("new_db"), None);
        let fake_db = instance.database(String::from("fake_db"), None);
        assert_eq!(db.exists(), true);
        assert_eq!(fake_db.exists(), false);
    }

    #[test]
    fn reload() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = client.instance(String::from("archived"), None, None, None);
        let mut db = instance.database(String::from("new_db"), None);
        db.reload();
        assert_eq!(db.ddl_statements().as_ref().unwrap().len(), 1);
    }

    #[test] 
    fn drop() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = client.instance(String::from("archived"), None, None, None);
        let db = instance.database(String::from("other_db"), None);
        let op = db.create();
        match op.result {
            OperationResult::Response => {
                assert_eq!(db.exists(), true);
                db.drop();
                assert_eq!(db.exists(), false);
            },
            OperationResult::Error(_) => { }
        }
    }

    #[test] 
    fn create_session() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = client.instance(String::from("archived"), None, None, None);
        let mut db = instance.database(String::from("testdata"), None);
        let mut session = db.session();
        session.create();
        assert_eq!(session.name(), String::from("projects/rusty-206403/instances/archived/databases/testdata/sessions/") + session.id().unwrap());
        session.delete();
        assert_eq!(session.exists(), false);
    }
}