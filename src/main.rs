use rust_webserver::{Request, Response, Server};

fn test_callback(mut req: Request, mut res: Response) -> () {
    res.set_content("Diamond Hands".to_string());
    res.send();
}

fn main() {
    // Example Usage
    let mut app: Server = Server::build();

    app.get("/", test_callback);

    app.listen(7878, Some(|| println!("Listening on port {}", 7878)));
}
