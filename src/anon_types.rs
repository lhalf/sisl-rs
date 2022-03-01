pub fn dumps<T: Type>(input: T) -> String
{
    String::from("{\"_\": !_") + &input.get_type() + " \"" + &input.get_value() + "\"}"
}

pub trait Type
{
    fn get_type(&self) -> String;
    fn get_value(&self) -> String;
}

impl Type for &str
{
    fn get_type(&self) -> String
    {
        String::from("str")
    }
    fn get_value(&self) -> String
    {
        self.to_string()
    }
}

#[duplicate::duplicate_item(integer; [i8]; [i16]; [i32]; [i64]; [i128]; [u8]; [u16]; [u32]; [u64]; [u128])]
impl Type for integer
{
    fn get_type(&self) -> String
    {
        String::from("int")
    }
    fn get_value(&self) -> String
    {
        self.to_string()
    }
}

#[duplicate::duplicate_item(float; [f32]; [f64])]
impl Type for float
{
    fn get_type(&self) -> String
    {
        String::from("float")
    }
    fn get_value(&self) -> String
    {
        self.to_string()
    }
}

impl Type for bool
{
    fn get_type(&self) -> String
    {
        String::from("bool")
    }
    fn get_value(&self) -> String
    {
        self.to_string()
    }
}

impl Type for Option<()>
{
    fn get_type(&self) -> String
    {
        String::from("null")
    }
    fn get_value(&self) -> String
    {
        String::from("")
    }
}