use crate::repository::rocks::RocksRepository;
use serde_json;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use uuid::Uuid;
use crate::service::withid::WithId;

// see https://stackoverflow.com/questions/54851996/rust-and-serde-deserializing-using-generics
pub trait Crud<T: Serialize + DeserializeOwned + WithId> {
    fn rocks(&self) -> RocksRepository;
    fn cfname(&self) -> &str;

    fn list(&self) -> Vec<T> {
        let kvs = self.rocks().list(self.cfname(), "", 100);
        kvs.iter().map(|kv| {
            let mut data: T = serde_json::from_str(&kv.value).unwrap();
            data.set_id(Some(kv.key.clone()));
            data
        }).collect()
    }

    fn get(&self, id: &str) -> T {
        let res = self.rocks().get(self.cfname(), id);
        let mut data: T = serde_json::from_str(&res.value).unwrap();
        data.set_id(Some(res.key));
        data
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
