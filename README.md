# rex

simple web server written in rust

```
mod rex; mod http;

fn main() {
    rex::start (
        http::Server::new()
        .name(Some("Rex"))
        .port(Some(8080))
    )
}
```

working on making api simple as possible

async support powered by tokio

goal is to have no other dependancies

full implementation coming soon