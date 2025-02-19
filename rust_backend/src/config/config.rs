use dotenv::dotenv;
use std::env;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub mongo_uri: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        // Load environment variables from a .env file if available.
        dotenv()?;
        
        let mongo_uri = env::var("MONGO_URI")?;
        let jwt_secret = env::var("JWT_SECRET")?;
        
        Ok(Self { mongo_uri, jwt_secret })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_init() {
        // Set dummy environment variables for testing.
        env::set_var("MONGO_URI", "mongodb://localhost:27017");
        env::set_var("JWT_SECRET", "mysecret");

        let config = Config::init().expect("Failed to initialize config");
        assert_eq!(config.mongo_uri, "mongodb://localhost:27017");
        assert_eq!(config.jwt_secret, "mysecret");
    }
}
