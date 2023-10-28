use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// timestamp at '2023 01 01 00:00:00'
///
/// this is for smaller timestamp field
const CUSTOM_EPOCH: u64 = 1672498800;

#[derive(Debug)]
struct SnowflakeInner {
    ts: u64,
    seq: u64,
}

/// This can generate up to 4095 unique IDs a second
/// timestamp:  41 bits
/// machine_id: 10 bits
/// sequence:   12 bits (4095)
#[derive(Debug)]
pub struct Snowflake {
    machine_id: u64,
    inner: Mutex<SnowflakeInner>,
}

impl Snowflake {
    pub fn new(machine_id: u64) -> Self {
        Snowflake {
            machine_id,
            inner: Mutex::new(SnowflakeInner { ts: 0, seq: 0 }),
        }
    }

    pub fn generate(&self) -> Option<u64> {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() - CUSTOM_EPOCH;
        if let Ok(mut inner) = self.inner.lock() {
            // different timestamp, reset sequence number
            if ts > inner.ts {
                inner.ts = ts;
                inner.seq = 0;
            }

            let current_seq = inner.seq;
            if current_seq >= 4096 {
                None
            } else {
                let mut id = ts << 22;
                id |= (self.machine_id & 0b0011_1111_1111) << 12;
                id |= current_seq & 0b1111_1111_1111;
                inner.seq = current_seq + 1;
                Some(id)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn id_generator_test() {
        let machine1 = Snowflake::new(1);
        let machine2 = Snowflake::new(2);
        let machine3 = Snowflake::new(3);

        let mut arr = Vec::<u64>::with_capacity(4096 * 3);
        for _ in 0..4096 {
            // these (likely) have the same timestamp and sequence but different machine id
            arr.push(machine1.generate().unwrap());
            arr.push(machine2.generate().unwrap());
            arr.push(machine3.generate().unwrap());
        }
        assert_eq!(arr.len(), 4096 * 3);

        arr.dedup();
        assert_eq!(arr.len(), 4096 * 3);
    }
}
