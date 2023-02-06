pub fn dumps<T>(input: T) -> String where T: SISL
{
     String::from("{\"") + &input.get_name() + "\": !" + &input.get_type() + " \"" + &input.get_value() + "\"}"
}

pub trait SISL
{
    fn get_name(&self) -> String;
    fn get_type(&self) -> String;
    fn get_value(&self) -> String;
}

use duplicate::duplicate_item;
#[duplicate_item(basic_types; [i8]; [i16]; [i32]; [i64]; [u8]; [u16]; [u32]; [u64]; [f32]; [f64]; [&str]; [bool])]
impl SISL for basic_types
{
    fn get_name(&self) -> String
    {
        String::from("_")
    }
    fn get_type(&self) -> String
    {
        String::from("_") + &String::from(std::any::type_name::<basic_types>())
    }
    fn get_value(&self) -> String
    {
        self.to_string().replace("\"", "")
    }
}

impl SISL for (&str, &str)
{
    fn get_name(&self) -> String
    {
        String::from(self.0)
    }
    fn get_type(&self) -> String
    {
        String::from(std::any::type_name::<&str>())
    }
    fn get_value(&self) -> String
    {
        self.1.to_string().replace("\"", "")
    }
}

#[cfg(test)]
mod tests
{
    use crate::dumps;

    #[test]
    fn anon_string()
    {
    assert_eq!("{\"_\": !_&str \"test string\"}", dumps("test string"));
    }

    #[test]
    fn anon_i8()
    {
    assert_eq!("{\"_\": !_i8 \"-1\"}", dumps(-1 as i8));
    }

    #[test]
    fn anon_i16()
    {
    assert_eq!("{\"_\": !_i16 \"-32768\"}", dumps(-32768 as i16));
    }

    #[test]
    fn anon_i32()
    {
    assert_eq!("{\"_\": !_i32 \"-2147483648\"}", dumps(-2147483648 as i32));
    }

    #[test]
    fn anon_i64()
    {
    assert_eq!("{\"_\": !_i64 \"-2147483649\"}", dumps(-2147483649 as i64));
    }

    #[test]
    fn anon_u8()
    {
    assert_eq!("{\"_\": !_u8 \"1\"}", dumps(1 as u8));
    }

    #[test]
    fn anon_u16()
    {
    assert_eq!("{\"_\": !_u16 \"65535\"}", dumps(65535 as u16));
    }

    #[test]
    fn anon_u32()
    {
    assert_eq!("{\"_\": !_u32 \"4294967295\"}", dumps(4294967295 as u32));
    }

    #[test]
    fn anon_u64()
    {
    assert_eq!("{\"_\": !_u64 \"4294967296\"}", dumps(4294967296 as u64));
    }

    #[test]
    fn anon_f32()
    {
    assert_eq!("{\"_\": !_f32 \"10.1\"}", dumps(10.1 as f32));
    }

    #[test]
    fn anon_f64()
    {
    assert_eq!("{\"_\": !_f64 \"1000.1\"}", dumps(1000.1 as f64));
    }

    #[test]
    fn anon_bool()
    {
    assert_eq!("{\"_\": !_bool \"true\"}", dumps(true));
    }

    #[test]
    fn string()
    {
        assert_eq!("{\"name\": !&str \"test string\"}", dumps(("name", "test string")))
    }
}