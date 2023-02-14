#[derive(Default)]
pub struct Server {
    pub name: String,
    pub port: i32
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: Option<&str>) -> Self {
        self.name = name.unwrap_or("Rex").to_string();
        self
    }
    pub fn port(mut self, port: Option<i32>) -> Self {
        self.port = port.unwrap_or(8080);
        self
    }
}

impl crate::rex::RexServer for Server {
    fn name(&self, _name: Option<&str>) -> () {}
    fn port(&self, _port: Option<i32>) -> () {}
}