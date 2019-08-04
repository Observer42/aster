pub use failure::Error;

#[derive(Debug, Fail)]
pub enum AsError {
    #[fail(display = "invalid message")]
    BadMessage,

    #[fail(display = "message is ok but request bad or not allowed")]
    BadReqeust,

    #[fail(display = "request not spport")]
    RequestNotSupport,

    #[fail(display = "message reply is bad")]
    BadReply,

    #[fail(display = "fail to parse integer {}", _0)]
    ParseIntError(btoi::ParseIntegerError),

    #[fail(display = "CLUSTER SLOTS must be replied with array")]
    WrongClusterSlotsReplyType,

    #[fail(display = "CLUSTER SLOTS must contains slot info")]
    WrongClusterSlotsReplySlot,

    #[fail(display = "unexcept io error {}", _0)]
    IoError(tokio::io::Error),

    #[fail(display = "there is nothing happend")]
    None,
}

impl From<tokio::io::Error> for AsError {
    fn from(oe: tokio::io::Error) -> AsError {
        AsError::IoError(oe)
    }
}

impl From<btoi::ParseIntegerError> for AsError {
    fn from(oe: btoi::ParseIntegerError) -> AsError {
        AsError::ParseIntError(oe)
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    clusters: Vec<ClusterConfig>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum CacheType {
    #[serde(rename = "redis")]
    Redis,
    #[serde(rename = "memcache")]
    Memcache,
    #[serde(rename = "memcache_binary")]
    MemcacheBinary,
    #[serde(rename = "redis_cluster")]
    RedisCluster,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClusterConfig {
    pub name: String,
    pub listen_addr: String,
    pub hash_tag: Option<String>,

    pub thread: Option<usize>,
    pub cache_type: CacheType,

    pub read_timeout: Option<u64>,
    pub write_timeout: Option<u64>,

    pub servers: Vec<String>,

    // cluster special
    pub fetch: Option<u64>,
    pub read_from_slave: Option<bool>,

    // proxy special
    pub ping_fail_limit: Option<usize>,
    pub ping_interval: Option<usize>,

    // dead codes

    // command not support now
    pub dial_timeout: Option<u64>,
    // dead option: not support other proto
    pub listen_proto: Option<String>,

    // dead option: always 1
    pub node_connections: Option<usize>,
}
