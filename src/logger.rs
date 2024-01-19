use hyper::Request;

pub fn log_request<T>(request: Request<T>) {
    let x = request.method().as_str();
    let z = request.uri().path();

    println!("Request income: {x} \"{z}\"");
}
