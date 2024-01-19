use chrono;
use hyper::Request;

pub fn log_request<T>(request: Request<T>) {
    let dt = chrono::Local::now().to_rfc2822();

    let method = request.method();
    let path = request.uri().path();

    println!("[{dt}] {method} \"{path}\"");
}
