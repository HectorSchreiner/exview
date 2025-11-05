pub struct Alarm {
    name: String,
    alarm_id: AlarmID,
    entity_id: i32,
    log_message: String,
}

pub struct AlarmID(i32);