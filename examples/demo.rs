use poem::{handler, route, route::get, web::Path, EndpointExt, Server};
use poem_next::{middleware::SetHeader, NextMiddlewareGroup};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() {
    let set_header = SetHeader::new().appending("hello", "poem");

    let mut middleware_group = NextMiddlewareGroup::default();
    middleware_group.push(set_header);

    let app = route().at("/hello/:name", get(hello.with(middleware_group)));
    let listener = poem::listener::TcpListener::bind("127.0.0.1:3000");

    let server = Server::new(listener).await.unwrap();
    server.run(app).await.unwrap();
}
