# rust_webserver
A web framework written in Rust

## Overview
A minimalist web framework in pure Rust.  With additional concurrency support by pooling threads.

## Installing
Download the library with:
```
$ git clone https://github.com/caoalbe/rust_webserver.git
```

In your project's `Cargo.toml` file, add the dependency with the path to this repository:
```
[dependencies]
...
rust_webserver = { path = "../rust_webserver" }
```

## Examples
Here is a simple example which returns "Hello World!" to the root route:
```
use rust_webserver::{Request, Response, Server};

fn root_function(req: Request, mut res: Response) -> () {
    res.set_content("Hello World!".to_string());
    res.send();
}

fn main() {
    let app: &mut Server = Server::build();

    app.get("/", root_function);

    app.listen(3001, Some(|| println!("Listening on port 3001")));
}
```

You can introduce concurrency by specifying the amount of worker threads to handle requests:
```
let app: &mut Server = Server::build();
app.set_thread_count(4);
```