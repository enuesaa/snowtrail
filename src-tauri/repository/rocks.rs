use rocksdb::{DB, Options, SingleThreaded, DBWithThreadMode, IteratorMode};

#[derive(Debug)]
pub struct Kv {
    pub key: String,
    pub value: String,
}

pub struct RocksRepository {}
impl RocksRepository {
    pub fn new() -> Self {
        RocksRepository {}
    }

    pub fn cfs() -> Vec<String> {
        // need to write here ?
        vec!["event".to_string()]
    }

    pub fn connect() -> DBWithThreadMode<SingleThreaded> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        DB::open_cf(&opts, "rocksdb", RocksRepository::cfs()).unwrap()
    }

    pub fn get(self, cfname: &str, key: &str) -> Kv {
        let db = RocksRepository::connect();
        let cf = db.cf_handle(cfname).unwrap();

        if let Ok(Some(value)) = db.get_cf(&cf, key.as_bytes()) {
            return Kv {
                key: String::from(key),
                value: String::from_utf8(value).unwrap(),
            };
        };
        return Kv {
            key: String::from(key),
            value: String::from("{}"),
        };
    }

    pub fn put(self, cfname: &str, key: &str, val: &str) {
        let db = RocksRepository::connect();
        let cf = db.cf_handle(cfname).unwrap();

        let _ = db.put_cf(cf, key.as_bytes(), val.as_bytes());
    }

    pub fn delete(self, cfname: &str, key: &str) {
        let db = RocksRepository::connect();
        let cf = db.cf_handle(cfname).unwrap();

        let _ = db.delete_cf(cf, key.as_bytes());
    }

    pub fn list(self, cfname: &str) -> Vec<Kv> {
        let db = RocksRepository::connect();
        let cf = db.cf_handle(cfname).unwrap();
        let iter = db.iterator_cf(cf, IteratorMode::Start);
        let mut kvs: Vec<Kv> = vec![];
        for item in iter {
            let (key, value) = item.unwrap();
            kvs.push(Kv {
                key: String::from_utf8(Vec::from(key)).unwrap(),
                value: String::from_utf8(Vec::from(value)).unwrap(),
            });
        };

        kvs
    }
}
