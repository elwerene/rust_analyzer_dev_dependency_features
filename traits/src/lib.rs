pub trait Bla {
    fn always_available();

    #[cfg(feature = "test")]
    fn only_for_test_cases();
}
