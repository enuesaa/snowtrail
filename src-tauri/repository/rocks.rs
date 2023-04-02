use rocksdb::{DB, Options, SingleThreaded, DBWithThreadMode};
use dirs::home_dir;

#[derive(Debug, Clone)]
pub struct Kv {
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct RocksRepository {}
impl RocksRepository {
    pub fn new() -> Self {
        RocksRepository {}
    }

    pub fn cfs() -> Vec<String> {
        // need to write here ?
        vec!["event".to_string()]
    }

    pub fn path() -> String {
        let mut path = home_dir().unwrap();
        path.push(".snowtrail/rocksdb");
        path.to_str().unwrap().to_string()
    }

    pub fn connect() -> DBWithThreadMode<SingleThreaded> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        if let Err(err) = DB::open_cf(&opts, RocksRepository::path(), RocksRepository::cfs()) {
            println!("{:?}", err);
            panic!("open failed because");
        };
        DB::open_cf(&opts, "rocksdb", RocksRepository::cfs()).unwrap()
    }

    pub fn check_connect() -> String {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        if let Err(err) = DB::open_cf(&opts, RocksRepository::path(), RocksRepository::cfs()) {
            err.to_string()
        } else {
            "ok".to_string()
        }
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

    pub fn list(self, cfname: &str, prefix: &str, limit: u8) -> Vec<Kv> {
        let db = RocksRepository::connect();
        let cf = db.cf_handle(cfname).unwrap();
        let mut iter = db.raw_iterator_cf(cf);
        iter.seek(prefix.to_string().as_bytes());

        let mut kvs: Vec<Kv> = vec![];
        let mut i: u8 = 0;
        while iter.valid() {
            let (key, value) =iter.item().unwrap();
            kvs.push(Kv {
                key: String::from_utf8(Vec::from(key)).unwrap(),
                value: String::from_utf8(Vec::from(value)).unwrap(),
            });
            iter.next();
            i += 1;
            if i >= limit {
                break;
            }
        };
        kvs
    }

    pub fn list_all(self, cfname: &str, prefix: &str) -> Vec<Kv> {
        let db = RocksRepository::connect();
        let cf = db.cf_handle(cfname).unwrap();
        let mut iter = db.raw_iterator_cf(cf);
        iter.seek(prefix.to_string().as_bytes());

        let mut kvs: Vec<Kv> = vec![];
        while iter.valid() {
            let (key, value) =iter.item().unwrap();
            kvs.push(Kv {
                key: String::from_utf8(Vec::from(key)).unwrap(),
                value: String::from_utf8(Vec::from(value)).unwrap(),
            });
            iter.next();
        };
        kvs
    }
}
