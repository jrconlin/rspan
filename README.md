
🚨 | **NOTE** | 🚨 
---: | :---: | :---
🚨 | DO NOT USE | 🚨 
🚨 | This is "recovered" code from a repo that probably wisely deleted itself. It exists here purely for my own education. | 🚨 
🚨 | DO NOT USE | 🚨 

# rustyspanner

A RPC Spanner Client for Rust. This client is based on Google's [Python Client](https://github.com/GoogleCloudPlatform/google-cloud-python/tree/master/spanner).

## Documentation

Check [documentation](https://docs.rs/rustyspanner) to get a deep understanding of the client.

## Installation

Use cargo to install the client in your project

```
cargo install rustyspanner
```
## Getting started

### Authentication

To authenticate this application this client uses Google's [Application Default Credentials](https://cloud.google.com/docs/authentication/production) to find your application's credentials

### Instantiating a Client

First we provide the client with the project id to instantiate.

```
let mut client = Client::new(String::from("project_id"));
assert_eq!(client.project_name(), String::from("projects/project_id"));
```

With this client object we can list the available instances to get an instance we're insterested or
we can create an instance object of an already existing instance

```
//List instances
let instances = client.list_instances().instances;

//Instantiate an already existing instance
let id = String::from("instance_id")
let instance = let instance = client.instance(id, None, None, None);
assert_eq!(instance.name(), client.project_name() + "/instances/instance_id");
```

### Database

Once we have an instance object we can list the databases availables inside the instance to select the
one to use, we can instantiate an object of an already existing one or create a new one

```
//List databases
let databases = instance.list_databases().databases;

//Instantiate an already existing database
let id = String::from("database_id");
let db = instance.database(String::from("database_id"), None);
db.reload(); //Loads metadata

//Create a new database
let db = instance.database(String::from("new_db"), None);
let operation = db.create();
```

We can also drop or check if the database already exists

```
//Drop database
db.drop();

//Check if exists
db.exists();
```

To run a transaction on the we need to provide the function 'run_in_transaction' with a closure,
this closure receives the Transaction object and it should return the same Transaction object after
adding Mutations

```
let result = db.run_in_transaction(|mut txn| {
    //Table and columns
    let table = String::from("table_name");
    let columns = vec![String::from("key"), String::from("value")];

    //Protobuf Value object for the values to add
    let mut key = Value::new();
    key.set_string_value(String::from("test"));
    let mut val = Value::new();
    val.set_string_value(String::from("27"));

    //Insert Mutation
    txn.insert(table.clone(), columns.clone(), vec![vec![key.clone(), val.clone()]]);

    //Update Mutation
    val.set_string_value(String::from("28"));
    txn.update(table.clone(), columns.clone(), vec![vec![key.clone(), val.clone()]]);

    //Update or insert Mutation
    key.set_string_value(String::from("testd"));
    val.set_string_value(String::from("11"));
    txn.upsert(table.clone(), columns.clone(), vec![vec![key.clone(), val.clone()]]);

    //Delete Mutation
    let keyset = KeySet::new(Some(vec![String::from("testd")]), None, None);
    txn.delete(table.clone(), keyset);

    //Commit transaction
    txn.commit();
    txn
});
```

### Sessions

To run transactions on the database we can use the database directly like the section before or we can create a session with the database object and run the transaction there

```
let mut session = db.session();
session.create();
let result = session.run_in_transaction(|mut txn| {
    //Table and columns
    let table = String::from("table_name");
    let columns = vec![String::from("key"), String::from("value")];

    //Protobuf Value object for the values to add
    let mut key = Value::new();
    key.set_string_value(String::from("test"));
    let mut val = Value::new();
    val.set_string_value(String::from("27"));

    //Insert Mutation
    txn.insert(table.clone(), columns.clone(), vec![vec![key.clone(), val.clone()]]);

    //Commit transaction
    txn.commit();
    txn
});
```

Once you're done with the session you can call the `delete` method to remove it from the cloud

 ```
 //Delete session
 session.delete();
 ```

### Read and Query

To read and query data from the database we can use the `Snapshot` object, this object is created with a `Session` object. This next example shows how to read data using a `KeySet`. Currently `KeySet` receives a `String` representation of the value to obtain, this may change in the future.

```
//Snapshot creation in a specific timestamp
let timestamp = Utc::now();
let mut snapshot = session.snapshot(Some(timestamp), None, None, None, false);

// Table and columns
let table = String::from("table_name");
let columns = vec![String::from("key"), String::from("value")];

//Keyset where we provide a vector of string where we request the object with key "hello"
let keyset = KeySet::new(Some(vec![String::from("hello")]), None, None);

//Call the method `read` and get a `StreamedResultSet` object
let mut streamed_result = snapshot.read(table, columns, keyset, None, None, None);

//Get the value
let v = streamed_result.one();
assert_eq!(v[0].get_string_value(), "hello");
assert_eq!(v[1].get_string_value(), "5");
```

To query or execute a sql in the database we can follow the next example

```
//Snapshot creation in a specific timestamp
let timestamp = Utc::now();
let mut snapshot = session.snapshot(Some(timestamp), None, None, None, false);

//SQL to execute
let sql = String::from("SELECT key, value FROM table_name WHERE value < @threshold");

//Parameters to replace on SQL
let mut params = HashMap::new();
let mut val = Value::new();
val.set_string_value(String::from("10"));
params.insert(String::from("threshold"), val);

//Types of parameters
let mut param_types = HashMap::new();
param_types.insert(String::from("threshold"), Type::INT64);

//Use method `execute_sql` to run query
let mut streamed_result = snapshot.execute_sql(sql, Some(params), Some(param_types), None, None);

//This query returned multiple values so we call method `next` to get the next value
let v = streamed_result.next().unwrap();
assert_eq!(v[0].get_string_value(), "hello");
assert_eq!(v[1].get_string_value(), "5");
let x = streamed_result.next().unwrap();
assert_eq!(x[0].get_string_value(), "goodbye");
assert_eq!(x[1].get_string_value(), "6");
```

## Built With

* [gRPC-rs](https://github.com/pingcap/grpc-rs) -  Rust wrapper of gRPC Core.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Acknowledgments

* [Python Client for Cloud Spanner](https://github.com/GoogleCloudPlatform/google-cloud-python/tree/master/spanner) - Python idiomatic client for Cloud Spanner.
