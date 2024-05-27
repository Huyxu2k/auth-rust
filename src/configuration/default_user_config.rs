
#[derive(Clone)]
pub struct DefaultUserConfig{
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub enabled: bool,
}

impl DefaultUserConfig {
    pub fn new(
        username: String,
        email: Option<String>,
        password: String,
        enabled: bool,
    ) -> DefaultUserConfig {
        DefaultUserConfig {
            username,
            email,
            password,
            enabled,
        }
    }
}