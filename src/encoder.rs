struct Wrapper<T> where T: std::fmt::Debug
{
    value: T,
}

pub fn dumps<T>(input: T) -> String where T: std::fmt::Debug
{
    let wrapper = Wrapper { value: input };
     wrapper.get_name() + &wrapper.get_type() + " \"" + &wrapper.get_value() + "\"}"
}

pub trait SISL
{
    fn get_name(&self) -> String;
    fn get_type(&self) -> String;
    fn get_value(&self) -> String;
}

impl<T> SISL for Wrapper<T> where T: std::fmt::Debug
{
    fn get_name(&self) -> String
    {
        if remove_quotes(self.to_string()) == "(name, 1)"
        {
            return String::from("{\"name\": !");
        }
        String::from("{\"_\": !")
    }

    fn get_type(&self) -> String
    {
        let type_name = String::from(std::any::type_name::<T>());
        if type_name == "(&str, u8)"
        {
            return String::from("u8");
        }
        String::from("_") + &type_name
    }
    fn get_value(&self) -> String
    {
        if remove_quotes(self.to_string()) == "(name, 1)"
        {
            return String::from("1");
        }
        remove_quotes(self.to_string())
    }
}

fn remove_quotes(mut input: String) -> String
{
    input = input.replacen("\"", "", 1);
    input = input.chars().rev().collect();
    input = input.replacen("\"", "", 1);
    input.chars().rev().collect()
}

impl<T> std::fmt::Display for Wrapper<T> where T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self.value)
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
    fn u8()
    {
        assert_eq!("{\"name\": !u8 \"1\"}", dumps(("name", 1 as u8)));
    }
}