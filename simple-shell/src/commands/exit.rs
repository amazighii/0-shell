use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

pub static EXIT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub fn exit() {
    EXIT.store(true, Ordering::SeqCst);
}
