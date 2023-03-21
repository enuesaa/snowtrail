use rs_docker::Docker;
use rs_docker::container::{ContainerCreate, HostConfigCreate, PortBinding};
use std::collections::HashMap;

pub struct Port {
    pub container: String,
    pub host: String,
}
pub struct Container {
    name: String,
    image: String,
    ports: Vec<Port>,
}
impl Container {
    pub fn new(name: &str, image: &str, ports: Vec<Port>) -> Self {
        Container {
            name: name.to_string(),
            image: image.to_string(),
            ports,
        }
    }

    pub fn connect(&self) -> Docker {
        Docker::connect("unix:///var/run/docker.sock").unwrap()
    }

    pub fn create(self) -> Self {
        let mut ports = HashMap::new();
        self.ports.iter().for_each(|p| {
            ports.insert(p.container.clone(), vec![PortBinding { HostIp: None, HostPort: p.host.clone() }]);
        });

        let mut con = self.connect();
        let _ = con.create_container(self.name.clone(), ContainerCreate {
            Image: self.image.clone(),
            Labels: None,
            ExposedPorts: None,
            HostConfig: Some(HostConfigCreate {
                NetworkMode: None,
                PublishAllPorts: None,
                PortBindings: Some(ports),
            }),
        });
        self
    }

    pub fn start(self) -> Self {
        let mut con = self.connect();
        let _ = con.start_container(&self.name.clone());
        self
    }

    // pub fn status(self) {

    // }

    pub fn stop(self) -> Self {
        let mut con = self.connect();
        let _ = con.stop_container(&self.name.clone());
        self
    }

    pub fn delete(self) -> Self {
        let mut con = self.connect();
        let _ = con.delete_container(&self.name.clone());
        self
    }
}
