use std::env;

#[derive(Clone)]
pub struct Config {
    pub render_dir: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let render_dir = env::var("RENDER_DIR").unwrap_or("render".to_string());
        let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);
        let port = port;
        Config {
            render_dir,
            host,
            port,
        }
    }
}
