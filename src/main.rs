use rust_webserver::Server;

fn main() {
    // Example Usage
    let app: Server = Server {};

    app.get("/", "some_func".to_string());

    app.listen(7878);
}
