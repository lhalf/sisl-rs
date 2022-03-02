pub fn dumps<T>(input: T) -> String where T: std::fmt::Display
{
    String::from("{\"_\": !_") + std::any::type_name::<T>() + " \"" + &input.to_string() + "\"}"
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
}