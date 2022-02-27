pub fn dumps<T: Type>(input: T) -> String
{
    input.get_type()
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