use crate::configuration::{
    db_config,
    default_user_config,
    jwt_config,
    server_config,
};

use super::server_config::ServerConfig;

#[derive(Clone)]
pub struct Config{
    pub server_config: ServerConfig,
    pub open_api: bool,
}