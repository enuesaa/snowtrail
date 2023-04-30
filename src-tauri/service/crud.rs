use crate::repository::rocks::RocksRepository;
use serde_json;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use uuid::Uuid;

// see https://stackoverflow.com/questions/54851996/rust-and-serde-deserializing-using-generics
pub trait Crud<T: Serialize + DeserializeOwned> {
    fn rocks(&self) -> RocksRepository;
    fn cfname(&self) -> &str;

    fn list(&self) -> Vec<T> {
        let kvs = self.rocks().list(self.cfname(), "", 100);
        kvs.iter().map(|kv| {
            serde_json::from_str(&kv.value).unwrap()
        }).collect()
    }

    fn get(&self, id: &str) -> T {
        let res = self.rocks().get(self.cfname(), id);
        serde_json::from_str(&res.value).unwrap()
    }

    fn create(&self, data: T) -> String {
        let id = Uuid::new_v4().to_string();
        let value = &serde_json::to_string(&data).unwrap();
        self.rocks().put(self.cfname(), &id, &value);
        id
    }

    fn delete(&self, id: &str) {
        self.rocks().delete(self.cfname(), id);
    }
}
