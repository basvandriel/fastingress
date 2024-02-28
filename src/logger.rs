use chrono;

#[derive(Clone, Copy, Debug)]
pub struct Logger {}

impl Logger {
    fn getdatetime() -> String {
        chrono::Local::now().to_rfc2822()
    }
    pub fn info(&self, message: &str) {
        let dt = Self::getdatetime();
        println!("[{dt}] INFO: {message}");
    }
}
