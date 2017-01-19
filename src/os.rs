use std::env;

#[allow(dead_code)]
pub fn is_windows() -> bool {
    match env::var("HOME") {
        Ok(val) => {
            if val.starts_with("/Users/") {
                false
            } else {
                true
            }
        },
        Err(_) => true
    }
}