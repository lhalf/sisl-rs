pub fn dumps<T>(input: T) -> String where T: std::fmt::Display
{
    String::from("{\"_\": !_") + std::any::type_name::<T>() + " \"" + &input.to_string() + "\"}"
}

#[cfg(test)]
mod tests
{
    use crate::dumps;

    #[test]
    fn anon_i8()
    {
        assert_eq!("{\"_\": !_i8 \"-1\"}", dumps(-1 as i8));
    }
}