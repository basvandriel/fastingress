use chrono;

#[derive(Clone, Copy)]
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
