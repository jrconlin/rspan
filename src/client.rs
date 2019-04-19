use google::spanner::admin::instance::v1::spanner_instance_admin_grpc::*;
use google::spanner::admin::database::v1::spanner_database_admin_grpc::*;
use google::spanner::admin::instance::v1::spanner_instance_admin::*;

use grpcio::{ChannelBuilder, EnvBuilder, ChannelCredentials};

use instance;
use std::sync::Arc;
use std::fmt;

/// Client for interacting with Cloud Spanner API.
pub struct Client {
    project_id: String,
    instance_admin_api: InstanceAdminClient,
    database_admin_api: DatabaseAdminClient
}

impl Client {

    /// Initializes a new Client with a given `project_id`.
    ///
    /// # Arguments
    ///
    /// * `project_id` - This `String` represents the project id to work with.
    ///                  Note: Don't confuse the project id with the project name.
    ///
    /// # Return value
    ///
    /// A `Client` connected to a specified project.
    pub fn new(project_id: String) -> Self {
        let credentials = ChannelCredentials::google_default_credentials().unwrap();
        let credentials1 = ChannelCredentials::google_default_credentials().unwrap();

        let env = Arc::new(EnvBuilder::new().build());
        let env1 = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).secure_connect("spanner.googleapis.com", credentials);
        let ch1 = ChannelBuilder::new(env1).secure_connect("spanner.googleapis.com", credentials1);
        Client {
            project_id: project_id,
            instance_admin_api: InstanceAdminClient::new(ch),
            database_admin_api: DatabaseAdminClient::new(ch1)
        }
    }

    /// Project name to be used with Spanner APIs.
    ///
    /// The project name is of form:
    ///
    /// `projects/{project_id}`
    ///
    /// # Return value
    ///
    /// A `String` representing the project name to be used with the Cloud Spanner
    /// Admin API RPC service.
    pub fn project_name(&self) -> String {
        format!("projects/{}", self.project_id)
    }

    /// Getter of project id.
    ///
    /// # Return value
    ///
    /// A `&String` of the project id.
    pub fn id(&self) -> &String {
        &self.project_id
    }

    /// Helper for session-related API calls.
    pub fn instance_admin_api(&self) -> &InstanceAdminClient {
        &self.instance_admin_api
    }

    /// Helpers for session-related API calls.
    pub fn database_admin_api(&self) -> &DatabaseAdminClient {
        &self.database_admin_api
    }

    /// List available instance configutations for the client's project.
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
    /// A [`ListInstanceConfigs`] struct with the result.
    ///
    /// [`ListInstanceConfigs`]: ../instance/struct.ListInstanceConfigs.html
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.instance.v1#google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs
    pub fn list_instance_configs(&mut self, page_size: Option<i32>, page_token: Option<String>) -> instance::ListInstanceConfigs {
        let mut req = ListInstanceConfigsRequest::new();
        req.set_parent(self.project_name().to_string());
        match page_size {
            Some(p) => { req.set_page_size(p); },
            None => { }
        }
        match page_token {
            Some(p) => { req.set_page_token(p); },
            None => { }
        }
        let mut res = self.instance_admin_api().list_instance_configs(&req).unwrap();
        instance::ListInstanceConfigs {
            instance_configs: res.take_instance_configs().into_vec().iter().map(|x| instance::InstanceConfig::from_pb(x)).collect(),
            next_page_token: res.take_next_page_token()
        }
    }

    /// Factory to create a instance associated with this client.
    ///
    /// # Arguments
    ///
    /// * `instance_id` - The ID of the instance.
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
    /// An [`Instance`] owned by this client.
    ///
    /// [`Instance`]: ../instance/struct.Instance.html
    pub fn instance(&self, instance_id: String, config_name: Option<String>, display_name: Option<String>, node_count: Option<i32>) -> instance::Instance {
        instance::Instance::new(instance_id, self, config_name, display_name, node_count)
    }

    /// List instances for the client's project.
    ///
    /// See [RPC docs]
    ///
    /// # Arguments
    ///
    /// * `filter` - (Optional) Filter to select instances listed. See the `ListIntancesRequest` docs above for examples.
    ///
    /// * `page_size` - (Optional) Maximum number of results to return.
    ///
    /// * `page_token` - (Optional) Token for fetching next page of results.
    ///
    /// # Return value
    ///
    /// A [`ListInstances`] struct with the result.
    ///
    /// [`ListInstances`]: ../instance/struct.ListInstances.html
    /// [RPC docs]: https://cloud.google.com/spanner/docs/reference/rpc/google.spanner.admin.instance.v1#google.spanner.admin.instance.v1.InstanceAdmin.ListInstances
    pub fn list_instances(&mut self, filter: Option<String>, page_size: Option<i32>, page_token: Option<String>) -> instance::ListInstances {
        let mut req = ListInstancesRequest::new();
        req.set_parent(self.project_name());
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
        let mut res = self.instance_admin_api().list_instances(&req).unwrap();
        let mut instances = Vec::new();
        for x in res.take_instances().into_vec().iter() {
            instances.push(instance::Instance::from_pb(x, self));
        }
        instance::ListInstances {
            instances: instances,
            next_page_token: res.take_next_page_token()
        }
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client {{ project_name: {} }}", self.project_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let mut client = Client::new(String::from("rusty-206403"));
        assert_eq!(client.project_name(), String::from("projects/rusty-206403"));
        assert_eq!(client.name(), "rusty-206403");
        assert_eq!(client.list_instances(None, None, None).instances.len(), 1);
    }

    #[test]
    fn create_instance() {
        let client = Client::new(String::from("rusty-206403"));
        let id = String::from("archived");
        let instance = client.instance(id, None, None, None);
        assert_eq!(instance.name(), client.project_name() + "/instances/archived");
    }

}