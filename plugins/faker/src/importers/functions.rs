use model::value::Value;
use rand::{
    seq::{IndexedMutRandom, IndexedRandom},
    Rng,
};
use resources::{
    EMAILS, FIRST_NAMES, GERMAN_CITIES, LAST_NAMES, STATES_PROVINCES, STREET_ADDRESSES,
};
use std::time::{SystemTime, UNIX_EPOCH};

mod resources;

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

pub struct RandomEmail;
impl RandomFunction for RandomEmail {
    fn generate(&self) -> Value {
        let mut rng = rand::rng();
        let value = EMAILS.choose(&mut rng).unwrap_or(&"default@example.com");

        Value::String(value.to_string())
    }
}

pub struct RandomCities;
impl RandomFunction for RandomCities {
    fn generate(&self) -> Value {
        let mut rng = rand::rng();
        let value = GERMAN_CITIES.choose(&mut rng).unwrap_or(&"Regensburg");

        Value::String(value.to_string())
    }
}

pub struct RandomLastNames;
impl RandomFunction for RandomLastNames {
    fn generate(&self) -> Value {
        let mut rng = rand::rng();
        let value = LAST_NAMES.choose(&mut rng).unwrap_or(&"Meier");

        Value::String(value.to_string())
    }
}

pub struct RandomFirstNames;
impl RandomFunction for RandomFirstNames {
    fn generate(&self) -> Value {
        let mut rng = rand::rng();
        let value = FIRST_NAMES.choose(&mut rng).unwrap_or(&"Max");

        Value::String(value.to_string())
    }
}

pub struct RandomAddressLine;
impl RandomFunction for RandomAddressLine {
    fn generate(&self) -> Value {
        let mut rng = rand::rng();
        let value = STREET_ADDRESSES.choose(&mut rng).unwrap_or(&"Holzweg 13");

        Value::String(value.to_string())
    }
}

pub struct RandomPostalCode;
impl RandomFunction for RandomPostalCode {
    fn generate(&self) -> Value {
        let mut rng = rand::rng();
        let value = rng.random_range(8000..100000);

        Value::U32(value)
    }
}

pub struct RandomStates;
impl RandomFunction for RandomStates {
    fn generate(&self) -> Value {
        let mut rng = rand::rng();
        let value = STATES_PROVINCES.choose(&mut rng).unwrap_or(&"Bayern");

        Value::String(value.to_string())
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

    // Function to validate an email address
    fn is_valid_email(email: &str) -> bool {
        let email_regex =
            regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }

    #[test]
    fn test_generate_email() {
        let generator = RandomEmail;
        for _ in 0..1000 {
            let value = generator.generate();
            assert!(is_valid_email(&value.to_string()));
        }
    }

    #[test]
    fn test_pick_random_city() {
        let generator = RandomCities;

        for _ in 0..1000 {
            let city = generator.generate();
            assert!(
                GERMAN_CITIES.contains(&city.to_string().as_str()),
                "The picked city is not in the array"
            );
        }
    }

    #[test]
    fn test_pick_random_last_name() {
        let generator = RandomLastNames;

        for _ in 0..1000 {
            let last_name = generator.generate();
            assert!(
                LAST_NAMES.contains(&last_name.to_string().as_str()),
                "The picked last name is not in the array"
            );
        }
    }

    #[test]
    fn test_pick_random_first_name() {
        let generator = RandomFirstNames;

        for _ in 0..1000 {
            let first_name = generator.generate();
            assert!(
                FIRST_NAMES.contains(&first_name.to_string().as_str()),
                "The picked first name is not in the array"
            );
        }
    }

    #[test]
    fn test_pick_random_address_line() {
        let generator = RandomAddressLine;

        for _ in 0..1000 {
            let value = generator.generate();
            assert!(
                STREET_ADDRESSES.contains(&value.to_string().as_str()),
                "The picked value is not in the array"
            );
        }
    }

    #[test]
    fn test_pick_random_postal_code() {
        let generator = RandomPostalCode;

        for _ in 0..1000 {
            if let Value::U32(value) = generator.generate() {
                assert!(
                    value >= 8000 && value < 100000,
                    "The picked value is not in the range"
                );
            } else {
                panic!("Wrong Value type");
            }
        }
    }

    #[test]
    fn test_pick_random_state() {
        let generator = RandomStates;

        for _ in 0..1000 {
            let value = generator.generate();
            assert!(
                STATES_PROVINCES.contains(&value.to_string().as_str()),
                "The picked value is not in the array"
            );
        }
    }
}
