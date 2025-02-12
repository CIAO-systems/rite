use model::value::Value;
use rand::{seq::IndexedMutRandom, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait RandomFunction {
    fn generate(&self) -> Value;
}

pub struct Milliseconds;

impl RandomFunction for Milliseconds {
    fn generate(&self) -> Value {
        Value::I64(
            if let Ok(value) = SystemTime::now().duration_since(UNIX_EPOCH) {
                value.as_millis() as i64
            } else {
                0
            },
        )
    }
}

pub struct Timezone;
impl RandomFunction for Timezone {
    fn generate(&self) -> Value {
        let mut all_tz: Vec<&str> = chrono_tz::TZ_VARIANTS.iter().map(|&tz| tz.name()).collect();
        let random_timezone = all_tz.choose_mut(&mut rand::rng()).unwrap();
        Value::String(random_timezone.to_string())
    }
}

pub struct Uuid;
impl RandomFunction for Uuid {
    fn generate(&self) -> Value {
        Value::String(uuid::Uuid::new_v4().to_string())
    }
}

pub struct RandomInteger;
impl RandomFunction for RandomInteger {
    fn generate(&self) -> Value {
        let v: i32 = rand::rng().random();
        Value::I32(v)
    }
}

pub struct RandomFloat;
impl RandomFunction for RandomFloat {
    fn generate(&self) -> Value {
        let v: f32 = rand::rng().random();
        Value::F32(v)
    }
}

pub struct RandomString;
impl RandomFunction for RandomString {
    fn generate(&self) -> Value {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";
        let mut rng = rand::rng();
        let length = rng.random_range(1..32);
        let random_string: String = (0..length)
            .map(|_| {
                let idx = (rng.random_range(0..CHARSET.len())) as usize;
                CHARSET[idx] as char
            })
            .collect();
        Value::String(random_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::TZ_VARIANTS;

    #[test]
    fn test_generate_timezone() {
        let generator = Timezone;
        let result = generator.generate();
        println!("{:?}", result);

        // Check if the result is a String variant
        match result {
            Value::String(timezone_name) => {
                // Check if the result is a valid IANA timezone
                assert!(TZ_VARIANTS.iter().any(|&tz| tz.name() == timezone_name));
            }
            _ => panic!("Expected a String variant, but got a different variant."),
        }
    }
}
