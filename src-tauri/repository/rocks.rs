use rocksdb::{DB, SingleThreaded, DBWithThreadMode};

pub struct Kv {
    pub key: String,
    pub value: String,
}

pub struct Rocks {}
impl Rocks {
    pub fn new() -> Self {
        Rocks {}
    }

    fn connect(self) -> DBWithThreadMode<SingleThreaded> {
        DB::open_default("rocksdb").unwrap()
    }

    pub fn get(self, key: &str) -> Kv {
        let res = self.connect().get(key.as_bytes());
        if let Ok(Some(value)) = res {
            return Kv {
                key: String::from(key),
                value: String::from_utf8(value).unwrap(),
            };
        };
        return Kv {
            key: String::from(key),
            value: String::from(""),
        };
    }

    pub fn set(self, key: &str, val: &str) {
        let _ = self.connect().put(
            key.as_bytes(),
            val.as_bytes()
        );
    }

    pub fn list(self, prefix: &str, limit: Option<u8>) -> Vec<Kv> {
        let limit = match limit {
            Some(l) => l,
            None => 10,
        };
        let db = self.connect();
        let mut iter = db.raw_iterator();
        iter.seek(prefix.to_string().as_bytes());

        let mut kvs: Vec<Kv> = vec![];
        let mut i: u8 = 0;
        while iter.valid() {
            let key = Vec::from(iter.key().unwrap());
            let value = Vec::from(iter.value().unwrap());
            kvs.push(Kv {
                key: String::from_utf8(key).unwrap(),
                value: String::from_utf8(value).unwrap(),
            });
            iter.next();
            i += 1;
            if i >= limit {
                break;
            }
        };
        kvs
    }
}
