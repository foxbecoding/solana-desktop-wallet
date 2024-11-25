use rusqlite::{Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CacheValue {
    value: String,
}

struct Cache {
    conn: Connection,
}

