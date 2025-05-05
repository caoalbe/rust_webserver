use rust_webserver::Server;


fn test_callback() -> () {
    println!("Hello World!")
}

fn main() {
    // Example Usage
    let mut app: Server = Server::build();

    app.get("/", test_callback);

    app.listen(7878);
}
