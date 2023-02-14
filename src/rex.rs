pub trait RexServer {
    fn name(&self, name: Option<&str>) -> ();
    fn port(&self, port: Option<i32>) -> ();
}

pub fn start(server: crate::http::Server) {
    println!("server name: {}", server.name);
}