

#[derive(Debug,Clone)]
pub struct  JwtConfig{
    pub jwt_secret: String,
    pub jwt_expiration: usize,
}

impl JwtConfig {
    pub fn new(jwt_secret: String, jwt_expiration: usize)->JwtConfig{
        JwtConfig { 
            jwt_secret, 
            jwt_expiration 
        }
    }
}