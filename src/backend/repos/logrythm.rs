use crate::backend::domains::alarm::*;


pub struct Logrythm {
    api_key: String,
}

impl Logrythm {
    pub fn new(api_key: &str) -> Self {
        Self { api_key: api_key.to_string() }
    }

    pub fn get_alerts(count: i32) -> Result<Vec<Alarm>, ApiMessage> {
        todo!()
    }

    pub fn post_comment(message: String, alert_id: AlarmID) -> Result<(), ApiMessage> {
        todo!()
    }

    pub fn drilldown() -> Result<(), ApiMessage> {
        todo!()
    }
}

pub enum ApiMessage {
    Ok,
    ApiErr(String),
    NotFound(String),
}

