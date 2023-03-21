use rs_docker::Docker;
use rs_docker::container::ContainerCreate;
use std::collections::HashMap;

pub struct Dataround {}
impl Dataround {
    pub fn up() -> String {
        let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
            Ok(docker) => docker,
            Err(e) => { panic!("{}", e); }
        };
        let mut ports = HashMap::new();
        ports.insert("6380/tcp".to_string(), HashMap::new());

        let _ = docker.create_container("snowtrail-redis".to_string(), ContainerCreate {
            Image: "redis".to_string(),
            Labels: None,
            ExposedPorts: Some(ports),
            HostConfig: None,
        });
        let _ = docker.start_container("snowtrail-redis");
        "snowtrail-redis".to_string()
    }
    
    pub fn down() {
        let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
            Ok(docker) => docker,
            Err(e) => { panic!("{}", e); }
        };
        let _ = docker.stop_container("snowtrail-redis");
        let _ = docker.delete_container("snowtrail-redis");
    }
}
