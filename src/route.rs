pub struct Route<'a> {
    path: &'a str,
    file: &'a str,
}

impl<'a> Route<'a> {
    pub fn new(path: &'a str, file: &'a str) -> Self {
        Self { path, file }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn file(&self) -> &str {
        &self.file
    }
}

// --[ Set routes here ] --
pub fn routes() -> Vec<Route<'static>> {
    Vec::from([
        Route::new("/", "index.html"),
        Route::new("/hello", "hello.html"),
        Route::new("/sleep", "hello.html"),
    ])
}
