mod rex; mod http;

fn main() {
    rex::start (
        http::Server::new()
        .name(Some("Rex"))
        .port(Some(8080))
    )
}