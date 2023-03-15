use std::str::FromStr;

use tracing;
use serde_json::Value;


pub fn log_success(mut info: Value)  {
    let info_object = info.as_object_mut().expect("invalid log value");
    info_object.insert("status".to_string(), "SUCCESS".into());
    info_object.insert("status_code".to_string(), 200.into());
    tracing::info!("{:?}", info_object);
}