pub struct ServerConfig {
    pub app_name: String,
    pub password: String,
    pub base_url: String,
}

impl ServerConfig {
    pub fn new(app_name: String, password: String, base_url: String) -> Self {
        ServerConfig {
            app_name,
            password,
            base_url,
        }
    }
}
