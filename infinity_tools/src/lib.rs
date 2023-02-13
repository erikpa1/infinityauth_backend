pub mod serdetools {
    use serde_json::{Value, json};


    pub struct SafeValue {
        value: Value,
    }

    impl SafeValue {
        pub fn from(value: Value) -> SafeValue {
            return SafeValue { value };
        }


        pub fn has_string(&self, key: &str) -> bool {

            if let Some(_value) = self.value.get(key) {
                return true;
            } else {
                return false;
            }
        }

        pub fn get_string_or(&self, key: &str, defaultValue: &str) -> String {
            if let Some(_value) = self.value.get(key) {
                return _value.as_str().unwrap().to_string();
            }

            String::from(defaultValue)
        }

        pub fn get_u64_or(&self, key: &str, defaultValue: u64) -> u64 {
            if let Some(_value) = self.value.get(key) {
                return _value.as_u64().unwrap();
            }
            defaultValue
        }

        pub fn get_json_or(&self, key: &str, defaultValue: SafeValue) -> SafeValue {
            if let Some(_value) = self.value.get(key) {
                let val = _value;
                return SafeValue::from(_value.to_owned());
            }

            defaultValue
        }
    }
}

