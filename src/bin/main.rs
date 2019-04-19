extern crate rustyspanner;

use rustyspanner::client::*;

fn main() {
    let mut client = Client::new(String::from("rusty-206403"));
    let res = client.list_instance_configs(None, None);
    println!("instances config {:?}", res.instance_configs);

    let res = client.list_instances(None, None, None);

    println!("instances: {:?}", res.instances);;
}