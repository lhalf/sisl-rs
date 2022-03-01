use sisl;

#[test]
fn anon_string()
{
    assert_eq!("{\"_\": !_str \"test string\"}", sisl::dumps("test string"));
}

#[test]
fn anon_i8()
{
    assert_eq!("{\"_\": !_int \"-1\"}", sisl::dumps(-1 as i8));
}

#[test]
fn anon_i16()
{
    assert_eq!("{\"_\": !_int \"-32768\"}", sisl::dumps(-32768 as i16));
}

#[test]
fn anon_i32()
{
    assert_eq!("{\"_\": !_int \"-2147483648\"}", sisl::dumps(-2147483648 as i32));
}

#[test]
fn anon_i64()
{
    assert_eq!("{\"_\": !_int \"-2147483649\"}", sisl::dumps(-2147483649 as i64));
}

#[test]
fn anon_u8()
{
    assert_eq!("{\"_\": !_int \"1\"}", sisl::dumps(1 as u8));
}

#[test]
fn anon_u16()
{
    assert_eq!("{\"_\": !_int \"65535\"}", sisl::dumps(65535 as u16));
}

#[test]
fn anon_u32()
{
    assert_eq!("{\"_\": !_int \"4294967295\"}", sisl::dumps(4294967295 as u32));
}

#[test]
fn anon_u64()
{
    assert_eq!("{\"_\": !_int \"4294967296\"}", sisl::dumps(4294967296 as u64));
}

#[test]
fn anon_f32()
{
    assert_eq!("{\"_\": !_float \"10.1\"}", sisl::dumps(10.1 as f32));
}

#[test]
fn anon_f64()
{
    assert_eq!("{\"_\": !_float \"1000.1\"}", sisl::dumps(1000.1 as f64));
}

#[test]
fn anon_bool()
{
    assert_eq!("{\"_\": !_bool \"true\"}", sisl::dumps(true));
}

#[test]
fn anon_none()
{
    assert_eq!("{\"_\": !_null \"\"}", sisl::dumps(None::<()>));
}