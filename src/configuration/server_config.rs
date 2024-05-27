
#[derive(Clone)]
pub struct ServerConfig{
    pub address: String,
    pub port: u16,
    pub max_limit: i64,
    pub workers: usize,
}

impl  ServerConfig {
    pub fn new(address: String, port: u16, max_limit:i64, workers: usize)->ServerConfig{
        ServerConfig { 
            address, 
            port, 
            max_limit, 
            workers
        }
    }
}