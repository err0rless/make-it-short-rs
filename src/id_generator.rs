use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use redis::Commands;

/// timestamp at '2023 01 01 00:00:00'
///
/// this is for smaller timestamp field
const CUSTOM_EPOCH: u64 = 1672498800;

/// This can generate up to 4095 unique IDs a second
/// timestamp:  41 bits
/// machine_id: 10 bits
/// sequence:   12 bits (4095)
pub struct Snowflake {
    /// redis connection for the unique shared sequence number across server instances
    redis: Mutex<redis::Connection>,

    /// Database ID
    machine_id: u64,
}

impl Snowflake {
    pub fn new(redis: redis::Connection, machine_id: u64) -> Self {
        Snowflake {
            redis: Mutex::new(redis),
            machine_id,
        }
    }

    fn construct_id(ts: u64, machine_id: u64, seq: u64) -> u64 {
        let mut id = 0;
        id |= ts << 22;
        id |= (machine_id & 0b0011_1111_1111) << 12;
        id |= seq & 0b1111_1111_1111;
        id
    }

    pub fn generate(&self) -> Option<u64> {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() - CUSTOM_EPOCH;
        let Ok(seq) = self.redis.lock().unwrap().incr::<u64, u64, u64>(ts, 1) else {
            return None;
        };

        if seq >= 4096 {
            // too many requests within a second
            None
        } else {
            Some(Snowflake::construct_id(ts, self.machine_id, seq))
        }
    }
}
