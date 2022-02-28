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

#[duplicate::duplicate_item(integer; [i8]; [i16]; [i32]; [i64]; [i128]; [u8]; [u16]; [u32]; [u64]; [u128])]
impl Type for integer
{
    fn get_type(&self) -> String
    {
        String::from("int")
    }
}

#[duplicate::duplicate_item(float; [f32]; [f64])]
impl Type for float
{
    fn get_type(&self) -> String
    {
        String::from("float")
    }
}

impl Type for bool
{
    fn get_type(&self) -> String
    {
        String::from("bool")
    }
}