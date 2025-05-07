use rust_webserver::{Request, Response, Server};

fn test_callback(mut req: Request, mut res: Response) -> () {
    if req.get_query("name") == "anthonyEdwards" {
        res.set_content("jordan".to_string());
    } else {
        res.set_content("mwoya?".to_string());
    }
    res.send();
}

fn main() {
    // Example Usage
    let mut app: Server = Server::build();

    app.get("/", test_callback);

    app.listen(7878, Some(|| println!("Listening on port {}", 7878)));
}
