pub fn dumps<T: Type>(input: T) -> String where T: std::fmt::Display
{
    String::from("{\"_\": !_") + &input.get_type() + " \"" + &input.to_string() + "\"}"
}

pub trait Type
{
    fn get_type(&self) -> String;
}

impl Type for &str
{
    fn get_type(&self) -> String
    {
        String::from("str")
    }
}

#[duplicate::duplicate_item(integer; [i8]; [i16])]
impl Type for integer
{
    fn get_type(&self) -> String
    {
        String::from("int")
    }
}