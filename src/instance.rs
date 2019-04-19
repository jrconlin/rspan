use google::spanner::admin::instance::v1::spanner_instance_admin;
use google::spanner::admin::instance::v1::spanner_instance_admin::{CreateInstanceRequest, GetInstanceRequest, DeleteInstanceRequest, UpdateInstanceRequest};
use google::spanner::admin::database::v1::spanner_database_admin::{ListDatabasesRequest};
use client::Client;
use database::{Database, ListDatabases};
use regex::Regex;
use operation::{Operation, OperationResult};
use protobuf::well_known_types::{FieldMask};
use protobuf::{RepeatedField};

/// Representation of a Cloud Spanner Instance
///
/// We can use `Instance` to:
/// * `reload` itself
/// * `create` itself
/// * `update` itself
/// * `delete` itself
#[derive(Debug)]
pub struct Instance<'a> {
    pub instance_id: String,
    client: &'a Client,
    config_name: Option<String>,
    pub node_count: i32,
    pub display_name: String
}

const DEFAULT_NODE_COUNT: i32 = 1;

impl<'a> Instance<'a> {
    /// Initializes a new Instance owned by a given [`Client`]
    ///
    /// # Arguments
    ///
    /// * `client` - The client that owns the instance. Provides aithorization and
    ///              a project ID.
    ///
    /// * `config_name` - (Optional) Name of the instance configuration used to set up
    ///                   instance's cluster, in the form:
    ///                   `projects/<project_id>/instanceConfigs/<config>`.
    ///                   **Required** for instances which do not yet exist.
    ///
    /// * `display_name` - (Optional) The display name for the instance in the
    ///                    Cloud Console UI. (Must be between 4 and 30 characters.)
    ///                    If this value is not set in the constructor, will fall back
    ///                    to the instance ID.
    ///
    /// * `node_count` - (Optional) The number of nodes in the instance's cluster;
    ///                  used to set up the instance's cluster.
    ///
    /// # Return value
    ///
    /// An `Instance` owned by client.
    pub fn new(instance_id: String, client: &'a Client, config_name: Option<String>, display_name: Option<String>, node_count: Option<i32>) -> Self {
        let clone_id = instance_id.clone();
        Instance {
            instance_id: instance_id,
            client: client,
            config_name: config_name,
            display_name: display_name.unwrap_or(clone_id),
            node_count: node_count.unwrap_or(DEFAULT_NODE_COUNT)
        }
    }


    /// Helper for session-related API calls.
    pub fn client(&self) -> &Client {
        self.client
    }

    /// Instance name used in requests.
    ///
    /// The instance name is of form:
    ///
    /// `projects/{project_id}/instances/{instance_id}`
    ///
    /// # Return value
    ///
    /// A `String` representing the instance name
    pub fn name(&self) -> String {
        self.client.project_name() + "/instances/" + &self.instance_id
    }

    /// Getter of instace id.
    ///
    /// # Return value
    ///
    /// A `&String` of the instance id.
    pub fn id(&self) -> &String {
        &self.instance_id
    }

    /// Creates an instance from a protobuf
    ///
    /// # Arguments
    ///
    /// * `instance_pb` - An instance protobuf object.
    ///
    /// * `client` - The client that owns the instance
    ///
    /// # Return value
    ///
    /// An `Instance` parsed from the protobuf response.
    /// If the instance name foes not match:
    ///
    /// `projects/{project_id}/instances/{instance_id}`
    ///
    /// or if the parsed project ID does not match the client.
    pub fn from_pb(instance_pb: &spanner_instance_admin::Instance, client: &'a Client) -> Instance<'a> {
        let re = Regex::new(r"^projects/(?P<project>[^/]+)/instances/(?P<instance_id>[a-z][-a-z0-9]*)$").unwrap();
        let caps = re.captures(&instance_pb.name);
        if caps.is_none() {
            panic!("Instance protobuf name was not in the expected format: {}", instance_pb.name);
        }
        let c = caps.unwrap();
        if &c["project"] != client.id() {
            panic!("Project ID on instance does not match the project ID on the client");
        }

        let instance_id = &c["instance_id"];
        let config_name = instance_pb.config.to_string();
        let mut result = Instance::new(instance_id.to_string(), client, Some(config_name), None, None);
        result.update_from_pb(instance_pb);
        result
    }

    /// Refresh self from the server-provided protobuf.
    ///
    /// Helper for method `from_pb` and method `reload`.
    fn update_from_pb(&mut self, instance_pb: &spanner_instance_admin::Instance) {
        if instance_pb.display_name.is_empty() {
            panic!("Instance protobuf does not contain display_name");
        }
        self.display_name = instance_pb.display_name.to_string();
        self.config_name = Some(instance_pb.config.to_string());
        self.node_count = instance_pb.node_count;
    }

    /// Create this instance.
    ///
    /// See [RPC docs]
    ///
    /// Uses the `project_name` on Instance owner and `instance_id`
    /// on the currente `Instance` in addition to the `display_name`.
    /// To change them before creating, reset the values via
    ///
    /// ```rust
    /// instance.display_name = String::from("New display name");
    /// instance.instance_id = String::from("i-changed-my-mind");
    /// ```
    ///
    /// before calling method `create`.
    ///
    /// # Return value
    ///
    /// An [`Operation`] instance.
    ///
    /// # Panics
    ///
    /// If the instance already exists.
    ///
    /// [`Operation`]: ../operation/struct.Operation.html
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.instance.v1#google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance
    pub fn create(&self) -> Operation {
        let api = self.client.instance_admin_api();
        
        let mut req = CreateInstanceRequest::new();

        req.set_parent(self.client.project_name());
        req.set_instance_id(self.instance_id.clone());
        let mut instance_pb = spanner_instance_admin::Instance::new();
        instance_pb.set_name(self.name());
        match self.config_name {
            Some(ref cn) => { instance_pb.set_config(cn.to_string()); },
            None => { }
        }
        instance_pb.set_display_name(self.display_name.clone());
        instance_pb.set_node_count(self.node_count.clone());
        req.set_instance(instance_pb);
        let res = api.create_instance(&req);
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

    /// Test whether this instance exists.
    ///
    /// See [RPC docs]
    ///
    /// # Return value
    ///
    /// True if the instance exists, else false.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.instance.v1#google.spanner.admin.instance.v1.InstanceAdmin.GetInstance
    pub fn exists(&self) -> bool {
        let api = self.client.instance_admin_api();
        let mut req = GetInstanceRequest::new();
        req.set_name(self.name());

        let res = api.get_instance(&req);
        match res {
            Ok(_) => { true },
            _ => { false }
        }
    }

    /// Reload the metadata for this instance.
    ///
    /// See [RPC docs]
    ///
    /// # Panics
    ///
    /// If the instance does not exist.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.instance.v1#google.spanner.admin.instance.v1.InstanceAdmin.GetInstance
    pub fn reload(&mut self) {
        let api = self.client.instance_admin_api();
        let mut req = GetInstanceRequest::new();
        req.set_name(self.name());

        let res = api.get_instance(&req).unwrap();
        self.update_from_pb(&res);
    }

    /// Update this instance.
    ///
    /// See [RPC docs]
    ///
    /// Updates the `display_name` and `node_count`. To change those
    /// values before updating, set them via
    ///
    /// ```rust
    /// instance.display_name = String::from("New display name");
    /// instance.node_count = 5;
    /// ```
    ///
    /// before calling method `update`.
    ///
    /// # Return value
    ///
    /// An [`Operation`] instance.
    ///
    /// # Panics
    ///
    /// If the instance does not exist.
    ///
    /// [`Operation`]: ../operation/struct.Operation.html
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.instance.v1#google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance
    pub fn update(&self) -> Operation {
        let api = self.client.instance_admin_api();
        let mut req = UpdateInstanceRequest::new();
        let mut instance_pb = spanner_instance_admin::Instance::new();
        instance_pb.set_name(self.name());
        match self.config_name {
            Some(ref cn) => { instance_pb.set_config(cn.to_string()); },
            None => { }
        }
        instance_pb.set_display_name(self.display_name.clone());
        instance_pb.set_node_count(self.node_count.clone());
        let paths = vec![String::from("config"), String::from("display_name"), String::from("node_count")];
        let mut field_mask = FieldMask::new();
        field_mask.set_paths(RepeatedField::from_vec(paths));
        req.set_instance(instance_pb);
        req.set_field_mask(field_mask);

        let res = api.update_instance(&req);
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

    /// Mark and instance and all of its databases for permanent deletion.
    ///
    /// See [RPC docs]
    ///
    /// Immediately upon completion of the request:
    ///
    /// * Billing will cease for all of the instance's reserved resources.
    ///
    /// Soon afterward:
    ///
    /// * The instance and all databases within the instance will be deleted.
    ///   All data in the databases will be permanently deleted.
    ///
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.instance.v1#google.spanner.admin.instance.v1.InstanceAdmin.DeleteInstance
    pub fn delete(&self) {
        let api = self.client.instance_admin_api();
        let mut req = DeleteInstanceRequest::new();
        req.set_name(self.name());
        api.delete_instance(&req).unwrap();
    }

    /// Factory to create a database within this instance.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the instance.
    ///
    /// * `ddl_statements` - (Optional) DDL statements, exclusing the `CREATE DATABASE` statement.
    ///
    /// # Return value
    ///
    /// A [`Database`] owned by this instance.
    ///
    /// [`Database`]: ../database/struct.Database.html
    pub fn database(&'a self, id: String, ddl_statements: Option<Vec<String>>) -> Database<'a> {
        Database::new(id, self, ddl_statements)
    }

    /// List databases for the instance.
    ///
    /// See [RPC docs]
    ///
    /// # Arguments
    ///
    /// * `page_size` - (Optional) Maximum number of results to return.
    ///
    /// * `page_token` - (Optional) Token for fetching next page of results.
    ///
    /// # Return value
    ///
    /// A [`ListDatabases`] struct with the result.
    ///
    /// [`ListDatabases`]: ../database/struct.ListDatabases.html
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.database.v1#google.spanner.admin.database.v1.DatabaseAdmin.ListDatabases
    pub fn list_databases(&self, page_size: Option<i32>, page_token: Option<String>) -> ListDatabases {
        let mut req = ListDatabasesRequest::new();
        req.set_parent(self.name());
        match page_size {
            Some(p) => { req.set_page_size(p); },
            None => { }
        }
        match page_token {
            Some(p) => { req.set_page_token(p); },
            None => { }
        }
        let mut res = self.client().database_admin_api().list_databases(&req).unwrap();
        let mut databases = Vec::new();
        for x in res.take_databases().into_vec().iter() {
            databases.push(Database::from_pb(x, self));
        }
        ListDatabases {
            databases: databases,
            next_page_token: res.take_next_page_token()
        }
    }
}

/// Named configurations for Spanner Instances.
#[derive(Debug)]
pub struct InstanceConfig {
    name: String,
    display_name: String
}

impl InstanceConfig {
    /// Initializes a new named configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - ID of the instance configuration.
    ///
    /// * `display_name` - NAme of the instance configuration.
    ///
    /// # Return value
    ///
    /// An `InstanceConfig`.
    pub fn new(name: String, display_name: String) -> Self {
        InstanceConfig {
            name: name,
            display_name: display_name
        }
    }

    /// Constructs and instance from the equivalent protobuf.
    ///
    /// # Arguments
    ///
    /// * `config_pb` - The protobuf to parse
    ///
    /// # Return value
    ///
    /// An instance of this struct.
    pub fn from_pb(config_pb: &spanner_instance_admin::InstanceConfig) -> InstanceConfig {
        InstanceConfig::new(config_pb.name.to_string(), config_pb.display_name.to_string())
    }
}

/// Helper struct used when listing all instance configurations.
pub struct ListInstanceConfigs {
    pub instance_configs: Vec<InstanceConfig>,
    pub next_page_token: String
}

/// Helper struct used when listing all instances.
pub struct ListInstances<'a> {
    pub instances: Vec<Instance<'a>>,
    pub next_page_token: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let client = Client::new(String::from("rusty-206403"));
        let id = String::from("archived");
        let display_name = String::from("archived");
        let config_name = String::from("projects/rusty-206403/instanceConfigs/regional-us-west1");
        let instance = Instance::new(id, &client, Some(config_name), Some(display_name), Some(1));
        assert_eq!(instance.name(), client.project_name() + "/instances/archived");
        assert_eq!(instance.list_databases(None, None).databases.len(), 2);
    }

    #[test]
    fn create_database() {
        let client = Client::new(String::from("rusty-206403"));
        let instance = client.instance(String::from("archived"), None, None, None);
        let db = instance.database(String::from("testdata"), None);
        assert_eq!(db.name(), String::from("projects/rusty-206403/instances/archived/databases/testdata"));
    }

}