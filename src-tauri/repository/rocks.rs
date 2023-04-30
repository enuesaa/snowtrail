use rocksdb::{DB, Options, SingleThreaded, DBWithThreadMode, ColumnFamily};
use dirs::home_dir;

#[derive(Debug, Clone)]
pub struct RocksKv {
    pub key: String,
    pub value: String,
}
impl From<&str> for RocksKv {
    fn from(key: &str) -> Self {
        RocksKv { key: key.to_string(), value: "{}".to_string() }
    }
}
impl RocksKv {
    pub fn key(mut self, key: Vec<u8>) -> Self {
        self.key = String::from_utf8(key).unwrap();
        self
    }

    pub fn value(mut self, value: Vec<u8>) -> Self {
        self.value = String::from_utf8(value).unwrap();
        self
    }
}

#[derive(Clone)]
pub struct RocksRepository {}
impl RocksRepository {
    pub fn new() -> Self {
        RocksRepository {}
    }

    pub fn cfs() -> Vec<String> {
        ["event", "workspace", "project", "script", "project_script"].iter().map(|s| s.to_string()).collect()
    }

    pub fn path() -> String {
        let path = home_dir().unwrap().join(".snowtrail/rocksdb");
        path.to_str().unwrap().to_string()
    }

    pub fn options() -> Options {
        let mut opts: Options = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts
    }

    pub fn connect() -> DBWithThreadMode<SingleThreaded> {
        match DB::open_cf(&Self::options(), &Self::path(), &Self::cfs()) {
            Ok(db) => db,
            Err(err) => panic!("Failed to open rocksdb because {message}", message = err),
        }
    }

    pub fn check_connect() -> String {
        match DB::open_cf(&Self::options(), &Self::path(), &Self::cfs()) {
            Ok(db) => "ok".to_string(),
            Err(err) => err.to_string(),
        }
    }

    pub fn cf<'a>(db: &'a DBWithThreadMode<SingleThreaded>, cfname: &'a str) -> &'a ColumnFamily {
        db.cf_handle(cfname).unwrap()
    }

    pub fn get(&self, cfname: &str, key: &str) -> RocksKv {
        let db = Self::connect();
        let cf = Self::cf(&db, cfname);

        match db.get_cf(cf, key.as_bytes()) {
            Ok(Some(value)) => RocksKv::from(key).value(value),
            Ok(None) => RocksKv::from(key),
            Err(err) => RocksKv::from(key),
        }
    }

    pub fn put(&self, cfname: &str, key: &str, val: &str) {
        let db = Self::connect();
        let cf = Self::cf(&db, cfname);

        let _ = db.put_cf(cf, key.as_bytes(), val.as_bytes());
    }

    pub fn delete(&self, cfname: &str, key: &str) {
        let db = Self::connect();
        let cf = Self::cf(&db, cfname);

        let _ = db.delete_cf(cf, key.as_bytes());
    }

    pub fn list(&self, cfname: &str, prefix: &str, limit: usize) -> Vec<RocksKv> {
        let db = Self::connect();
        let cf = Self::cf(&db, cfname);

        let mut ret: Vec<RocksKv> = vec![];
        let mut cfiter = db.raw_iterator_cf(cf);
        cfiter.seek(prefix.to_string().as_bytes());
        while cfiter.valid() {
            let (key, value) = cfiter.item().unwrap();
            ret.push(RocksKv::from("").key(key.to_vec()).value(value.to_vec()));
            if ret.iter().count() >= limit {
                break;
            };
            cfiter.next();
        };
        ret
    }

    pub fn list_all(&self, cfname: &str, prefix: &str) -> Vec<RocksKv> {
        let db = Self::connect();
        let cf = Self::cf(&db, cfname);

        let mut ret: Vec<RocksKv> = vec![];
        let mut cfiter = db.raw_iterator_cf(cf);
        cfiter.seek(prefix.to_string().as_bytes());
        while cfiter.valid() {
            let (key, value) = cfiter.item().unwrap();
            ret.push(RocksKv::from("").key(key.to_vec()).value(value.to_vec()));
            cfiter.next();
        };
        ret
    }
}
