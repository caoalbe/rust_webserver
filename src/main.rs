use rust_webserver::{Request, Response, Server};

fn test_callback(req: &Request, res: &Response) -> () {
    println!("Hello Server!")
}

fn main() {
    // Example Usage
    let mut app: Server = Server::build();

    app.get("/", test_callback);

    app.listen(7878, Some(|| println!("Listening on port {}", 7878)));
}
