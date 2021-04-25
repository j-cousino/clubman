
mod test_basic_types {
    use clubman::basic;

    #[test]
    fn propername_test() {
        let mut name = basic::ProperName::default();
        let empty = { value: String::new() };

        assert!( empty.value.is_empty());
        name.value = String::from("ProperName");
        assert_eq!(name.value, "Propername")

    }
}
