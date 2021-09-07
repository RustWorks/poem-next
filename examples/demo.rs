use poem::{handler, route, route::get, web::Path, Server, EndpointExt};
use poem_next::{NextMiddlewareGroup, middleware::SetHeader};


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
    let server = Server::bind("127.0.0.1:3000").await.unwrap();
    server.run(app).await.unwrap();
}