use rocksdb::{DB, Options, SingleThreaded, DBWithThreadMode, IteratorMode};
// use std::convert::From;

#[derive(Debug)]
pub struct Kv {
    pub key: String,
    pub value: String,
}

pub struct Rocks {}
impl Rocks {
    pub fn new() -> Self {
        Rocks {}
    }

    fn connect(self, cfs: Vec<&str>) -> DBWithThreadMode<SingleThreaded> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        DB::open_cf(&opts, "rocksdb", cfs).unwrap()
    }

    pub fn get(self, key: &str) -> Kv {
        let db = self.connect(vec!["event"]);
        let eventcf = db.cf_handle("event").unwrap();
        let res = db.get_cf(&eventcf, key.as_bytes());
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
        let db = self.connect(vec!["event"]);
        let eventcf = db.cf_handle("event").unwrap();
        let _ = db.put_cf(
            eventcf,
            key.as_bytes(),
            val.as_bytes()
        );
    }

    pub fn list(self, prefix: &str, limit: Option<u8>) -> Vec<Kv> {
        let limit = match limit {
            Some(l) => l,
            None => 10,
        };
        let db = self.connect(vec!["aa", "event"]);
        let eventcf = db.cf_handle("event").unwrap();
        let mut iter = db.iterator_cf(eventcf, IteratorMode::Start);
        // iter.seek(prefix.to_string().as_bytes());

        let mut kvs: Vec<Kv> = vec![];
        for item in iter {
            let (key, value) = item.unwrap();
            let key = Vec::from(key);
            let value = Vec::from(value);
            kvs.push(Kv {
                key: String::from_utf8(key).unwrap(),
                value: String::from_utf8(value).unwrap(),
            });
        }
    
        // let mut i: u8 = 0;
        // while iter.valid() {
        //     let key = Vec::from(iter.key().unwrap());
        //     let value = Vec::from(iter.value().unwrap());
        //     kvs.push(Kv {
        //         key: String::from_utf8(key).unwrap(),
        //         value: String::from_utf8(value).unwrap(),
        //     });
        //     iter.next();
        //     i += 1;
        //     if i >= limit {
        //         break;
        //     }
        // };
        kvs
    }
}
