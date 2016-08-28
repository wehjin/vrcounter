use std::env;

static A_STATIC_VARIABLE: bool = match env::var("HOME") {
    Ok(val) => {
        if val.starts_with("/User/") {
            false
        } else {
            true
        }
    },
    Err(e) => true
};

fn main() {
}