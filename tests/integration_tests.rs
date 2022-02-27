use sisl;

#[test]
fn anon_string()
{
    assert_eq!("{\"_\": !_str \"test string\"}", sisl::anon_types::dumps("test string"));
}

#[test]
fn anon_i8()
{
    assert_eq!("{\"_\": !_int \"1\"}", sisl::anon_types::dumps(1 as i8));
}

#[test]
fn anon_i16()
{
    assert_eq!("{\"_\": !_int \"32766\"}", sisl::anon_types::dumps(32766 as i16));
}