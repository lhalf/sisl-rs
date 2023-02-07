pub fn dumps<T: SISL>(input: T) -> String {
    format!(
        "{{\"{}\": !{}{} {}}}",
        input.get_name().unwrap_or("_".to_string()),
        input.get_name().map(|_| "").unwrap_or("_"),
        input.get_type(),
        input.get_value()
    )
}

pub trait SISL {
    fn get_name(&self) -> Option<String>;
    fn get_type(&self) -> String;
    fn get_value(&self) -> String;
}

use duplicate::duplicate_item;
#[duplicate_item(int_types; [i8]; [i16]; [i32]; [i64]; [u8]; [u16]; [u32]; [u64])]
impl SISL for int_types {
    fn get_name(&self) -> Option<String> {
        None
    }
    fn get_type(&self) -> String {
        "int".to_string()
    }
    fn get_value(&self) -> String {
        format!("\"{}\"", self)
    }
}

#[duplicate_item(float_types; [f32]; [f64])]
impl SISL for float_types {
    fn get_name(&self) -> Option<String> {
        None
    }
    fn get_type(&self) -> String {
        String::from("float")
    }
    fn get_value(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl SISL for &str {
    fn get_name(&self) -> Option<String> {
        None
    }
    fn get_type(&self) -> String {
        "str".to_string()
    }
    fn get_value(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl SISL for bool {
    fn get_name(&self) -> Option<String> {
        None
    }
    fn get_type(&self) -> String {
        "bool".to_string()
    }
    fn get_value(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl<T: SISL> SISL for Option<T> {
    fn get_name(&self) -> Option<String> {
        None
    }
    fn get_type(&self) -> String {
        // use unwrap_or_else with closure, prevents "null" String being allocated if not needed
        self.as_ref()
            .map(|inner| inner.get_type())
            .unwrap_or_else(|| "null".to_string())
    }
    fn get_value(&self) -> String {
        self.as_ref()
            .map(|inner| inner.get_value())
            .unwrap_or_else(|| "\"\"".to_string())
    }
}

impl<T: SISL> SISL for &[T] {
    fn get_name(&self) -> Option<String> {
        None
    }
    fn get_type(&self) -> String {
        "list".to_string()
    }
    fn get_value(&self) -> String {
        // enumerate turns into (index, item)
        let items: Vec<String> = self
            .iter()
            .enumerate()
            .map(|(index, item)| {
                format!("\"_{}\": !{} {}", index, item.get_type(), item.get_value())
            })
            .collect();
        format!("{{{}}}", items.join(", "))
    }
}

impl<T: SISL, const SIZE: usize> SISL for [T; SIZE] {
    fn get_name(&self) -> Option<String> {
        self.as_ref().get_name()
    }
    fn get_type(&self) -> String {
        self.as_ref().get_type()
    }
    fn get_value(&self) -> String {
        self.as_ref().get_value()
    }
}

impl<T: SISL> SISL for Vec<T> {
    fn get_name(&self) -> Option<String> {
        self.as_slice().get_name()
    }
    fn get_type(&self) -> String {
        self.as_slice().get_type()
    }
    fn get_value(&self) -> String {
        self.as_slice().get_value()
    }
}

impl<T: SISL> SISL for &T {
    fn get_name(&self) -> Option<String> {
        (*self).get_name()
    }
    fn get_type(&self) -> String {
        (*self).get_type()
    }
    fn get_value(&self) -> String {
        (*self).get_value()
    }
}

impl<T: SISL> SISL for (&str, T) {
    fn get_name(&self) -> Option<String> {
        Some(self.0.to_string())
    }
    fn get_type(&self) -> String {
        self.1.get_type()
    }
    fn get_value(&self) -> String {
        self.1.get_value()
    }
}

#[cfg(test)]
mod tests {
    use crate::dumps;

    #[test]
    fn anon_str() {
        assert_eq!("{\"_\": !_str \"test string\"}", dumps("test string"));
    }

    #[test]
    fn anon_i8() {
        assert_eq!("{\"_\": !_int \"-1\"}", dumps(-1 as i8));
    }

    #[test]
    fn anon_i16() {
        assert_eq!("{\"_\": !_int \"-32768\"}", dumps(-32768 as i16));
    }

    #[test]
    fn anon_i32() {
        assert_eq!("{\"_\": !_int \"-2147483648\"}", dumps(-2147483648 as i32));
    }

    #[test]
    fn anon_i64() {
        assert_eq!("{\"_\": !_int \"-2147483649\"}", dumps(-2147483649 as i64));
    }

    #[test]
    fn anon_u8() {
        assert_eq!("{\"_\": !_int \"1\"}", dumps(1 as u8));
    }

    #[test]
    fn anon_u16() {
        assert_eq!("{\"_\": !_int \"65535\"}", dumps(65535 as u16));
    }

    #[test]
    fn anon_u32() {
        assert_eq!("{\"_\": !_int \"4294967295\"}", dumps(4294967295 as u32));
    }

    #[test]
    fn anon_u64() {
        assert_eq!("{\"_\": !_int \"4294967296\"}", dumps(4294967296 as u64));
    }

    #[test]
    fn anon_f32() {
        assert_eq!("{\"_\": !_float \"10.1\"}", dumps(10.1 as f32));
    }

    #[test]
    fn anon_f64() {
        assert_eq!("{\"_\": !_float \"1000.1\"}", dumps(1000.1 as f64));
    }

    #[test]
    fn anon_bool() {
        assert_eq!("{\"_\": !_bool \"true\"}", dumps(true));
    }

    #[test]
    fn anon_array_ref() {
        assert_eq!(
            "{\"_\": !_list {\"_0\": !int \"1\", \"_1\": !int \"2\"}}",
            dumps([1, 2].as_ref())
        )
    }

    #[test]
    fn anon_vec_ref() {
        assert_eq!(
            "{\"_\": !_list {\"_0\": !int \"1\", \"_1\": !int \"2\"}}",
            dumps(vec![1, 2].as_slice())
        )
    }

    #[test]
    fn anon_vec() {
        assert_eq!(
            "{\"_\": !_list {\"_0\": !int \"1\", \"_1\": !int \"2\"}}",
            dumps(vec![1, 2])
        )
    }

    #[test]
    fn anon_array() {
        assert_eq!(
            "{\"_\": !_list {\"_0\": !int \"1\", \"_1\": !int \"2\"}}",
            dumps([1, 2])
        )
    }

    #[test]
    fn anon_none_bool() {
        assert_eq!("{\"_\": !_null \"\"}", dumps(None::<bool>))
    }

    #[test]
    fn anon_some_bool() {
        assert_eq!("{\"_\": !_bool \"true\"}", dumps(Some(true)))
    }

    #[test]
    fn str() {
        assert_eq!(
            "{\"str name\": !str \"test string\"}",
            dumps(("str name", "test string"))
        )
    }

    #[test]
    fn i8() {
        assert_eq!("{\"i8 name\": !int \"-1\"}", dumps(("i8 name", -1 as i8)))
    }

    #[test]
    fn i16() {
        assert_eq!(
            "{\"i16 name\": !int \"-32768\"}",
            dumps(("i16 name", -32768 as i16))
        )
    }

    #[test]
    fn i32() {
        assert_eq!(
            "{\"i32 name\": !int \"-2147483648\"}",
            dumps(("i32 name", -2147483648 as i32))
        )
    }

    #[test]
    fn i64() {
        assert_eq!(
            "{\"i64 name\": !int \"-2147483649\"}",
            dumps(("i64 name", -2147483649 as i64))
        )
    }

    #[test]
    fn array_ref() {
        assert_eq!(
            "{\"array name\": !list {\"_0\": !int \"1\", \"_1\": !int \"2\"}}",
            dumps(("array name", [1, 2].as_ref()))
        )
    }

    #[test]
    fn vec_ref() {
        assert_eq!(
            "{\"vec name\": !list {\"_0\": !int \"1\", \"_1\": !int \"2\"}}",
            dumps(("vec name", vec![1, 2].as_slice()))
        )
    }

    #[test]
    fn vec() {
        assert_eq!(
            "{\"vec name\": !list {\"_0\": !int \"1\", \"_1\": !int \"2\"}}",
            dumps(("vec name", vec![1, 2]))
        )
    }

    #[test]
    fn array() {
        assert_eq!(
            "{\"array name\": !list {\"_0\": !int \"1\", \"_1\": !int \"2\"}}",
            dumps(("array name", [1, 2]))
        )
    }

    #[test]
    fn bool_ref() {
        assert_eq!(
            "{\"bool name\": !bool \"true\"}",
            dumps(("bool name", &true))
        )
    }
}
