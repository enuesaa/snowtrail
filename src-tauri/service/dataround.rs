use crate::repository::docker::{Container, Port};

pub struct Dataround {}
impl Dataround {
    pub fn up() -> String {
        let port = Port { container: "6379/tcp".to_string(), host: "6380".to_string() };
        let container = Container::new("snowtrail-redis", "redis", vec![port]);
        container.create().start();
        "snowtrail-redis".to_string()
    }

    pub fn is_started() -> bool {
        Container::is_started("snowtrail-redis")
    }
    
    pub fn down() {
        let container = Container::new("snowtrail-redis", "redis", vec![]);
        container.stop().delete();
    }
}
