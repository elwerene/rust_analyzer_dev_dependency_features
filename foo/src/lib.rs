pub struct Foo;

impl traits::Bla for Foo {
    fn always_available() {}

    #[cfg(test)]
    fn only_for_test_cases() {}
}
