use serde::{Deserialize,Serialize};


#[derive(Clone,Serialize,Deserialize)]
pub struct DbConfig{
    pub connection_string: String,
    pub audit_connection_string: String,
    pub audit_enabled: bool,
    pub audit_ttl: u64,
}

impl DbConfig {
    pub fn new(
        connection_string: String,
        audit_connection_string: String,
        audit_enabled: bool,
        audit_ttl: u64
    )->DbConfig{
        DbConfig { 
            connection_string, 
            audit_connection_string, 
            audit_enabled, 
            audit_ttl
        }
    }
}