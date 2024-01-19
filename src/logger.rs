use chrono;
use hyper::Request;

pub fn log_request<T>(request: Request<T>, duration_ms: u128) {
    let dt = chrono::Local::now().to_rfc2822();

    let method = request.method();
    let path = request.uri().path();

    println!("[{dt}] {method} \"{path}\" - took {duration_ms}ms");
}
