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

    pub fn connect() -> Docker {
        Docker::connect("unix:///var/run/docker.sock").unwrap()
    }

    pub fn create(self) -> Self {
        let mut ports = HashMap::new();
        self.ports.iter().for_each(|p| {
            ports.insert(p.container.clone(), vec![PortBinding { HostIp: None, HostPort: p.host.clone() }]);
        });

        let _ = Container::connect().create_container(self.name.clone(), ContainerCreate {
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
        let _ = Container::connect().start_container(&self.name.clone());
        self
    }

    pub fn is_started(name: &str) -> bool {
        let con = Container::connect().get_containers(true);
        if let Ok(containers) = con {
            let count = containers.iter()
            .filter(|&c| {
                return c.Names.iter().filter(|&n| n.eq(&format!("/{}", name))).count() > 0
            })
            .count();
            return count > 0
        };
        false
    }

    pub fn stop(self) -> Self {
        let _ = Container::connect().stop_container(&self.name.clone());
        self
    }

    pub fn delete(self) -> Self {
        let _ = Container::connect().delete_container(&self.name.clone());
        self
    }
}
