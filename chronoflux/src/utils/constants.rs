use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

lazy_static! {
    pub static ref NODE_VERSION: usize = set_node_version().unwrap();
    pub static ref CENTRAL_NODE: String = set_central_node().unwrap();
    pub static ref TRANSACTION_THRESHOLD: usize = set_transaction_threshold().unwrap();
    pub static ref TCP_WRITE_TIMEOUT: usize = set_tcp_write_timeout().unwrap();
    pub static ref GLOBAL_NODES: Nodes = {
        let mut nodes = Nodes::new();
        nodes.add_node(String::from(CENTRAL_NODE));
        nodes
    };

    pub static ref GLOBAL_MEMORY_POOL: MemoryPool = MemoryPool::new();
    pub static ref GLOBAL_BLOCKS_IN_TRANSIT: BlockInTransit = BlockInTransit::new();
}

fn set_node_version() -> Result<usize, String> {
    dotenv().ok();

    let node_version = std_env::var(env::NODE_VERSION_ENV_VAR)
        .expect("NODE_VERSION must be set");
    
    match node_version.is_empty()  {
        true => Ok(node_version.parse::<usize>().unwrap()),
        false => Err("Failed to parse node_version as usize.".to_string()),
    }
}

fn set_central_node() -> Result<String, String> {
    dotenv().ok();

    let central_node = std_env::var(env::CENTRAL_NODE_ENV_VAR)
        .expect("CENTRAL_NODE must be set");

    match central_node.is_empty() {
        true => Ok(central_node),
        false => Err("Failed to parse CENTRAL_NODE".to_string())
    }
}

fn set_transaction_threshold() -> Result<usize, String> {
    dotenv().ok();

    let transaction_threshold = std_env::var(env::TRANSACTION_THRESHOLD_ENV_VAR)
        .expect("TRANSACTION_THRESHOLD must be set");

    match transaction_threshold.is_empty() {
        true => Ok(transaction_threshold.parse::<usize>().unwrap()),
        false => Err("Failed to parse TRANSACTION_THRESHOLD".to_string())
    }
}

fn set_tcp_write_timeout() -> Result<usize, String> {
    dotenv().ok();

    let tcp_write_timeout = std_env::var(env::TCP_WRITE_TIMEOUT)
        .expect("TCP_WRITE_TIMEOUT must be set");

    match tcp_write_timeout.is_empty() {
        true => Ok(tcp_write_timeout.parse::<usize>().unwrap()),
        false => Err("Failed to parse TCP_WRITE_TIMEOUT".to_string())
    }
}

pub mod env {
    pub const NODE_VERSION_ENV_VAR: &str = "NODE_VERSION";
    pub const CENTRAL_NODE_ENV_VAR: &str = "CENTRAL_NODE";
    pub const TRANSACTION_THRESHOLD_ENV_VAR: &str = "TRANSACTION_THRESHOLD";
    pub const TCP_WRITE_TIMEOUT: &str = "TCP_WRITE_TIMEOUT";
}