use chrono;
use hyper::Request;

pub fn log_request<T>(request: Request<T>, duration_ms: u128) {
    let method = request.method();
    let path = request.uri().path();

    let message = format!("{} \"{}\" - took {}ms", method, path, duration_ms);

    let logger = Logger {};
    logger.info(&message);
}

pub struct Logger {}

impl Logger {
    fn getdatetime() -> String {
        return chrono::Local::now().to_rfc2822();
    }
    pub fn info(&self, message: &str) {
        let dt = Self::getdatetime();
        println!("[{dt}] INFO: {message}");
    }
}
