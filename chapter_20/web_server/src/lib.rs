pub enum REQUEST {}
impl REQUEST {
    pub const INDEX:&'static str = "GET / HTTP/1.1";
    pub const SLEEP:&'static str = "GET /sleep HTTP/1.1";
    pub const EXIT:&'static str = "GET /exit HTTP/1.1";
}

pub enum STATUS {}
impl STATUS {
    pub const OK:&'static str = "HTTP/1.1 200 OK";
    pub const NOT_FOUND:&'static str = "HTTP/1.1 404 NOT FOUND";
}

pub enum WEBFILES {}
impl WEBFILES {
    pub const INDEX:&'static str = "index.html";
    pub const SHUTDOWN:&'static str = "shutdown.html";
    pub const NOT_FOUND:&'static str = "404.html";
}