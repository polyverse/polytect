// Copyright (c) 2019 Polyverse Corporation

use crate::events;
use chrono::Duration as ChronoDuration;

#[derive(PartialEq, Debug)]
pub struct KMsg {
    pub duration_from_system_start: ChronoDuration,
    pub facility: events::LogFacility,
    pub level: events::LogLevel,
    pub message: String,
}
