use rocksdb::DB;

pub struct Rocks {}
impl Rocks {
    pub fn new() -> Self {
        Rocks {}
    }

    pub fn get(self, key: &str) -> String {
        let res = DB::open_default("rocksdb").unwrap().get(key.to_string().as_bytes());
        if let Ok(Some(value)) = res {
            return String::from_utf8(value).unwrap();
        };
        "{}".to_string()
    }

    pub fn set(self, key: &str, val: &str) {
        let _ = DB::open_default("rocksdb").unwrap().put(
            key.to_string().as_bytes(),
            val.to_string().as_bytes()
        );
    }

    // pub fn list(self, prefix: &str) {
    //     let mut iter = DB::open_default("rocksdb").unwrap().raw_iterator();
    //     iter.seek(prefix.to_string().as_bytes());
    // }
}
