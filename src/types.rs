use std::sync::{Arc, Mutex};

pub type Arced<T> = Arc<Mutex<T>>;
