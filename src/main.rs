use rust_webserver::{Request, Response, Server};
use std::{thread, time::Duration};

fn test_callback(req: Request, mut res: Response) -> () {
    if req.get_query("name") == "anthonyEdwards" {
        res.set_content("jordan".to_string());
    } else {
        res.set_content("mwoya?".to_string());
    }
    res.send();
}

fn slow_callback(_req: Request, mut res: Response) -> () {
    thread::sleep(Duration::from_secs(5));
    res.set_content("...slow response...".to_string());
    res.send();
}

fn main() {
    // Example Usage
    let app: &mut Server = Server::build();
    // app.set_thread_count(4);

    app.get("/", test_callback);
    app.get("/slow", slow_callback);

    app.listen(7878, Some(|| println!("Listening on port {}", 7878)));
}
