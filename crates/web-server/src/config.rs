#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://application_user:testpassword@host.docker.internal:30001/stack-app?sslmode=disable".to_string());

        Config {
            database_url,
        }
    }
}
