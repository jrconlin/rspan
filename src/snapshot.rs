use google::spanner::v1::spanner::{PartitionOptions, ReadRequest, BeginTransactionRequest, PartitionReadRequest, PartitionQueryRequest, ExecuteSqlRequest_QueryMode, ExecuteSqlRequest};
use google::spanner::v1::transaction::{TransactionOptions, TransactionOptions_ReadOnly, TransactionSelector};

use helpers::{datetime_to_pb_timestamp, datetime_to_duration_pb, make_type_pb_map, QueryMode, Type};
use keyset::KeySet;
use streamed::StreamedResultSet;
use session::Session;

use protobuf::well_known_types::{Value, Struct};
use protobuf::RepeatedField;
use chrono::{DateTime, Utc};
use futures::Stream;

use std::collections::HashMap;

/// Allow a set of read/SQL statements with shared staleness.
///
/// Allows reuse of API request methods with different transaction selector.
pub struct Snapshot<'a> {
    session: &'a Session<'a>,
    multi_use: bool,
    pub transaction_id: Option<Vec<u8>>,
    read_request_count: u32,
    read_timestamp: Option<DateTime<Utc>>,
    min_read_timestamp: Option<DateTime<Utc>>,
    max_staleness: Option<DateTime<Utc>>,
    exact_staleness: Option<DateTime<Utc>>,
}

impl<'a> Snapshot<'a> {
    /// Initializes a new Database owned by a given `session`.
    ///
    /// See [RPC docs]
    ///
    /// If not options are passed, reads will use the `strong` model,
    /// reading at a timestamp where all previously committed transaction
    /// are visible.
    ///
    /// # Arguments
    ///
    /// * `session` - The session used to perform the commit.
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
    /// A `Snapshot` owned by session.
    ///
    /// # Panics
    ///
    /// * If the supplied options are more than one.
    /// * If `multi_use` is True and supplied a `min_read_timestamp` or `max_staleness`.
    ///
    /// [`read`]: #method.read
    /// [`execute_sql`]: #method.execute_sql
    /// [RPC docs]: https://cloud.google.com/spanner/reference/rpc/google.spanner.v1#google.spanner.v1.TransactionOptions.ReadOnly
    pub fn new(session: &'a Session<'a>, read_timestamp: Option<DateTime<Utc>>, min_read_timestamp: Option<DateTime<Utc>>, max_staleness: Option<DateTime<Utc>>, exact_staleness: Option<DateTime<Utc>>, multi_use: bool) -> Self {
        let opts = vec![read_timestamp, min_read_timestamp, max_staleness, exact_staleness];
        let flagged: Vec<&Option<DateTime<Utc>>> = opts.iter().filter(|x| x.is_some()).collect();

        if flagged.len() > 1 {
            panic!("Supply zero or one options.");
        }

        if multi_use {
            if min_read_timestamp.is_some() || max_staleness.is_some() {
                panic!("'multi_use' is incompatible with 'min_read_timestamp' / 'max_staleness'.");
            }
        }

        Snapshot {
            session: session,
            transaction_id: None,
            read_request_count: 0,
            read_timestamp: read_timestamp,
            min_read_timestamp: min_read_timestamp,
            max_staleness: max_staleness,
            exact_staleness: exact_staleness,
            multi_use: multi_use
        }
    }

    /// Setter of new transaction id, used by transaction-related
    /// API calls.
    pub fn set_transaction_id(&mut self, id: Vec<u8>) {
        self.transaction_id = Some(id);
    }

    /// Perform a `StreamingRead` API request for rows in a table.
    ///
    /// See [RPC docs]
    ///
    /// # Arguments
    ///
    /// * `table` - Name of the table from which to fetch data.
    ///
    /// * `columns` - Names of columns to be retrieved.
    ///
    /// * `keyset` - Keys/ranges identifying rows to be retrieved.
    ///
    /// * `index` - (Optional) Name of index to use, rather than be
    ///             table's primary key.
    ///
    /// * `limit` - (Optional) Maximum number of rows to return.
    ///             Incomptable with `partition`.
    ///
    /// * `partition` - (Optional) One of the partition token returned
    ///                 from method [`partition_read`]. Incompatible with
    ///                 `limit`.
    ///
    /// # Return value
    ///
    /// A [`StreamedResultSet`] instance which can be used to consume rows.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.v1#google.spanner.v1.Spanner.StreamingRead
    /// [`partition_read`]: #method.partition_read
    /// [`StreamedResultSet`]: ../streamed/struct.StreamedResultSet.html
    pub fn read(&'a mut self, table: String, columns: Vec<String>, keyset: KeySet, index: Option<String>, limit: Option<i64>, partition: Option<Vec<u8>>) -> StreamedResultSet {
        if self.read_request_count > 0 {
            if !self.multi_use {
                panic!("Cannot re-use single-use snapshot.");
            }
            if self.transaction_id.is_none() {
                panic!("Transaction ID pending.");
            }
        }

        let transaction = self.make_txn_selector();
        let api = self.session.database().spanner_api();
        let mut req = ReadRequest::new();
        req.set_session(self.session.name());
        req.set_transaction(transaction);
        req.set_table(table);
        req.set_columns(RepeatedField::from_vec(columns));
        req.set_key_set(keyset.to_pb());
        req.set_index(index.unwrap_or(String::from("")));
        req.set_limit(limit.unwrap_or(0 as i64));
        if partition.is_some() {
            req.set_partition_token(partition.unwrap());
        }
        let res = api.streaming_read(&req).unwrap();

        self.read_request_count += 1;

        if self.multi_use {
            StreamedResultSet::new(res.into_future(), Some(self))
        } else {
            StreamedResultSet::new(res.into_future(), None)
        }
    }

    /// Perform a `ExecuteStreamSql` API request
    ///
    /// See [RPC docs]
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL query statement.
    ///
    /// * `params` -  (Optional) Values for parameter replacement. Keys must match
    ///              the names used `sql`.
    ///
    /// * `param_types` - (Optional) maps explicit types for one or more param
    ///                   values; required if parameters are passed.
    ///
    /// * `query_mode` - (Optional) Mode governing return of results/query plan.
    ///                  See [docs].
    ///
    /// * `partition` - (Optional) One of the partition tokens returned from
    ///                 method [`partition_query`].
    ///
    /// # Return value
    ///
    /// A [`StreamedResultSet`] instance which can be used to consume rows.
    ///
    /// # Panics
    ///
    /// * If reusing single-use snapshot.
    /// * If a transaction ID is already pending for multiple-use snapshots.
    /// * If didn't supplied param_types for params
    /// 
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.v1#google.spanner.v1.Spanner.ExecuteStreamingSql
    /// [docs]: https://cloud.google.com/spanner/reference/rpc/google.spanner.v1#google.spanner.v1.ExecuteSqlRequest.QueryMode
    /// [`partition_query`]: #method.partition_query
    /// [`StreamedResultSet`]: ../streamed/struct.StreamedResultSet.html
    pub fn execute_sql(&'a mut self, sql: String, params: Option<HashMap<String, Value>>, param_types: Option<HashMap<String, Type>>, query_mode: Option<QueryMode>, partition: Option<Vec<u8>>) -> StreamedResultSet {
        if self.read_request_count > 0 {
            if !self.multi_use {
                panic!("Cannot re-use single-use snapshot.");
            }
            if self.transaction_id.is_none() {
                panic!("Transaction ID pending.");
            }
        }

        if params.is_some() && param_types.is_none() {
            panic!("Specify 'param_types' when passing 'params'.");
        }

        let params_pb: Option<Struct> = match params {
            Some(p) => {
                let mut st = Struct::new();
                st.set_fields(p);
                Some(st)
            },
            None => { None }
        };

        let transaction = self.make_txn_selector();
        let api = self.session.database().spanner_api();
        let mut req = ExecuteSqlRequest::new();
        req.set_session(self.session.name());
        req.set_sql(sql);
        req.set_transaction(transaction);
        if params_pb.is_some() {
            req.set_params(params_pb.unwrap());
        }

        if param_types.is_some() {
            req.set_param_types(make_type_pb_map(param_types.unwrap()));
        }

        if query_mode.is_some() {
            match query_mode.unwrap() {
                QueryMode::NORMAL => { req.set_query_mode(ExecuteSqlRequest_QueryMode::NORMAL); },
                QueryMode::PLAN => { req.set_query_mode(ExecuteSqlRequest_QueryMode::PLAN); },
                QueryMode::PROFILE => { req.set_query_mode(ExecuteSqlRequest_QueryMode::PROFILE); },
            };
        } else {
            req.set_query_mode(ExecuteSqlRequest_QueryMode::NORMAL);
        }

        if partition.is_some() {
            req.set_partition_token(partition.unwrap());
        }

        let res = api.execute_streaming_sql(&req).unwrap();

        self.read_request_count += 1;
        
        if self.multi_use {
            StreamedResultSet::new(res.into_future(), Some(self))
        } else {
            StreamedResultSet::new(res.into_future(), None)
        }
    }

    /// Perform a `PartitionRead` API request for rows in a table.
    ///
    /// See [RPC docs]
    ///
    /// # Arguments
    ///
    /// * `table` - Name of the table from which to fetch data.
    ///
    /// * `columns` - Names of columns to be retrieved.
    ///
    /// * `keyset` - Keys/ranges identifying rows to be retrieved.
    ///
    /// * `index` - (Optional) Name of index to use, rather than be
    ///             table's primary key.
    ///
    /// * `partition_size_bytes` - (Optional) Desired size for each partition
    ///                            generated. The service uses this as a hint,
    ///                            the actual partition size may differ.
    ///
    /// * `max_partitions` - (Optional) Desired maximum number of partitions
    ///                      generated. The uses this as a hint, the actual
    ///                      number of partitions may differ.
    ///
    /// # Return value
    ///
    /// A vector of partitions tokens.
    ///
    /// # Panics
    ///
    /// * If reusing single-use snapshot.
    /// * If a transaction hasn't started.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.v1#google.spanner.v1.Spanner.PartitionRead
    pub fn partition_read(&self, table: String, columns: Vec<String>, keyset: KeySet, index: Option<String>, partition_size_bytes: Option<i64>, max_partitions: Option<i64>) -> Vec<Vec<u8>>{
        if !self.multi_use {
            panic!("Cannot use single-use snapshot.");
        }
        if self.transaction_id.is_none() {
            panic!("Transaction not started.");
        }

        let api = self.session.database().spanner_api();
        let transaction = self.make_txn_selector();
        let mut partition_options = PartitionOptions::new();
        if partition_size_bytes.is_some() {
            partition_options.set_partition_size_bytes(partition_size_bytes.unwrap());
        }
        if max_partitions.is_some() {
            partition_options.set_max_partitions(max_partitions.unwrap());
        }
        let mut req = PartitionReadRequest::new();
        req.set_session(self.session.name());
        req.set_table(table);
        req.set_columns(RepeatedField::from_vec(columns));
        req.set_key_set(keyset.to_pb());
        req.set_transaction(transaction);
        req.set_index(index.unwrap_or(String::from("")));
        req.set_partition_options(partition_options);
        let mut response = api.partition_read(&req).unwrap();
        response.take_partitions().into_vec().iter().map(|x| x.get_partition_token().to_vec()).collect()
    }

    /// Perform a `PartitionQuery` API request for rows in a table.
    ///
    /// See [RPC docs]
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL query statement.
    ///
    /// * `params` -  (Optional) Values for parameter replacement. Keys must match
    ///              the names used `sql`.
    ///
    /// * `param_types` - (Optional) maps explicit types for one or more param
    ///                   values; required if parameters are passed.
    ///
    /// * `partition_size_bytes` - (Optional) Desired size for each partition
    ///                            generated. The service uses this as a hint,
    ///                            the actual partition size may differ.
    ///
    /// * `max_partitions` - (Optional) Desired maximum number of partitions
    ///                      generated. The uses this as a hint, the actual
    ///                      number of partitions may differ.
    ///
    /// # Return value
    ///
    /// A vector of partitions tokens.
    ///
    /// # Panics
    ///
    /// * If reusing single-use snapshot.
    /// * If a transaction hasn't started.
    /// * If didn't supplied param_types for params.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.v1#google.spanner.v1.Spanner.PartitionQuery
    pub fn partition_query(&self, sql: String, params: Option<HashMap<String, Value>>, param_types: Option<HashMap<String, Type>>, partition_size_bytes: Option<i64>, max_partitions: Option<i64>) -> Vec<Vec<u8>> {
        if !self.multi_use {
            panic!("Cannot use single-use snapshot.");
        }
        if self.transaction_id.is_none() {
            panic!("Transaction not started.");
        }
        if params.is_some() && param_types.is_none() {
            panic!("Specify 'param_types' when passing 'params'.");
        }
        
        let params_pb: Option<Struct> = match params {
            Some(p) => {
                let mut st = Struct::new();
                st.set_fields(p);
                Some(st)
            },
            None => { None }
        };

        let transaction = self.make_txn_selector();
        let api = self.session.database().spanner_api();
        let mut partition_options = PartitionOptions::new();
        if partition_size_bytes.is_some() {
            partition_options.set_partition_size_bytes(partition_size_bytes.unwrap());
        }
        if max_partitions.is_some() {
            partition_options.set_max_partitions(max_partitions.unwrap());
        }

        let mut req = PartitionQueryRequest::new();
        req.set_sql(sql);
        req.set_session(self.session.name());
        req.set_transaction(transaction);
        if params_pb.is_some() {
            req.set_params(params_pb.unwrap());
        }
        if param_types.is_some() {
            req.set_param_types(make_type_pb_map(param_types.unwrap()));
        }
        req.set_partition_options(partition_options);
        let mut res = api.partition_query(&req).unwrap();
        res.take_partitions().into_vec().iter().map(|x| x.get_partition_token().to_vec()).collect()        
    }

    /// Helper for method [`read`]/[`execute_sql`].
    ///
    /// [`read`]: #method.read
    /// [`execute_sql`]: #method.execute_sql
    fn make_txn_selector(&self) -> TransactionSelector {
        if self.transaction_id.is_some() {
            let mut txn_sel = TransactionSelector::new();
            match self.transaction_id {
                Some(ref id) => { txn_sel.set_id(id.to_vec()); },
                None => { }
            }
            return txn_sel
        }

        let mut opts = TransactionOptions::new();
        let mut mode = TransactionOptions_ReadOnly::new();
        mode.set_strong(true);

        if self.read_timestamp.is_some() {
            mode.set_read_timestamp(datetime_to_pb_timestamp(self.read_timestamp.unwrap()));
        } else if self.min_read_timestamp.is_some() {
            mode.set_min_read_timestamp(datetime_to_pb_timestamp(self.min_read_timestamp.unwrap()));
        } else if self.max_staleness.is_some() {
            mode.set_max_staleness(datetime_to_duration_pb(self.max_staleness.unwrap()));
        } else if self.exact_staleness.is_some() {
            mode.set_exact_staleness(datetime_to_duration_pb(self.exact_staleness.unwrap()));
        }

        opts.set_read_only(mode);

        let mut sel = TransactionSelector::new();
        if self.multi_use {
            sel.set_begin(opts);
        } else {
            sel.set_single_use(opts);
        }
        sel
    }

    /// Begin a read-only transaction on the database.
    ///
    /// # Return value
    ///
    /// The ID for the newly begun transaction.
    ///
    /// # Panics
    /// 
    /// If the transaction is already begun, commited or rolled back.
    pub fn begin(&mut self) -> Vec<u8> {
        if !self.multi_use {
            panic!("Cannot call 'begin' on single-use snapshots.");
        }

        if self.transaction_id.is_some() {
            panic!("Read-only transaction already begun.");
        }

        if self.read_request_count > 0 {
            panic!("Read-only transaction already pending.");
        }

        let api = self.session.database().spanner_api();
        let mut txn_sel = self.make_txn_selector();
        let mut req = BeginTransactionRequest::new();
        req.set_session(self.session.name());
        req.set_options(txn_sel.take_begin());
        let res = api.begin_transaction(&req).unwrap();
        self.transaction_id = Some(res.get_id().to_vec());
        res.get_id().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::Client;
    use instance::Instance;
    use database::Database;
    use keyset::KeySet;
    use protobuf::well_known_types::Value;

    #[test]
    fn read() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = Instance::new(String::from("archived"), &client, None, None, None);
        let mut db = Database::new(String::from("new_db"), &instance, None);
        let mut session = db.session();
        session.create();
        let timestamp = Utc::now();
        {
            let mut snapshot = session.snapshot(Some(timestamp), None, None, None, false);
            let table = String::from("main_table");
            let mut key = Value::new();
            key.set_string_value(String::from("test"));
            let mut val = Value::new();
            val.set_string_value(String::from("27"));
            let columns = vec![String::from("key"), String::from("value")];
            let keyset = KeySet::new(Some(vec![String::from("hello")]), None, None);
            let mut streamed_result = snapshot.read(table, columns, keyset, None, None, None);
            let v = streamed_result.one();
            assert_eq!(v[0].get_string_value(), "hello");
            assert_eq!(v[1].get_string_value(), "5");
        }
        session.delete();
    }

    #[test]
    fn execute_sql() {
        let client = Client::new(String::from("rustyspanner-207813"));
        let instance = Instance::new(String::from("archived"), &client, None, None, None);
        let mut db = Database::new(String::from("new_db"), &instance, None);
        let mut session = db.session();
        session.create();
        let timestamp = Utc::now();
        {
            let mut snapshot = session.snapshot(Some(timestamp), None, None, None, false);
            let sql = String::from("SELECT key, value FROM main_table WHERE value < @threshold");
            let mut params = HashMap::new();
            let mut val = Value::new();
            val.set_string_value(String::from("10"));
            params.insert(String::from("threshold"), val);
            let mut param_types = HashMap::new();
            param_types.insert(String::from("threshold"), Type::INT64);
            let mut streamed_result = snapshot.execute_sql(sql, Some(params), Some(param_types), None, None);
            let v = streamed_result.next().unwrap();
            assert_eq!(v[0].get_string_value(), "hello");
            assert_eq!(v[1].get_string_value(), "5");
            let x = streamed_result.next().unwrap();
            assert_eq!(x[0].get_string_value(), "goodbye");
            assert_eq!(x[1].get_string_value(), "6");

        }
        session.delete();
    }
}