use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Alarm {
    pub id: AlarmID,
    pub name: String,
    pub severity: AlarmSeverity,
    pub status: AlarmStatus,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub description: String,
}

impl Alarm {
    pub fn new(id: AlarmID, name: String) -> Self {
        Self {
            id,
            name,
            severity: todo!(),
            status: todo!(),
            timestamp: todo!(),
            source: todo!(),
            description: todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct AlarmID(i32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]

pub enum AlarmSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum AlarmStatus {
    Any,
    New,
    Open,
    Working,
    Escalated,
    Closed,
    FalsePositive,
    Monitor,
    Reported,
    Resolved,
    Unresolved,
    NotClosed
}