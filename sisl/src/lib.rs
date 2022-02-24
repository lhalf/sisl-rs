pub mod anon_types;

#[cfg(test)]
mod tests {
    #[test]
    fn anon_string() {
        assert_eq!("test string", crate::anon_types::dumps("test string"));
    }
}
