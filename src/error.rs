use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct JsonError {
    error: serde_error::Error,
}

pub fn exit_error_json(e: &anyhow::Error) {
    let js_err = JsonError { error: serde_error::Error::new(&**e) };
    println!("{}",serde_json::to_string(&js_err).unwrap_or("".to_string()));
    std::process::exit(1);
}
