# Multithreaded Web Server in Rust

### Description
This is a HTTP web server which is a modified and extended version of the one from The Rust Handbook

### How to use
1. Create html file in public folder
2. Create css file in public folder
3. In `route.rs`, creates a new Route object with `path` and `file`: `Route::new("/hello", "hello.html")`
4. Execute `cargo run` on the terminal
5. Go to the appropriate url
