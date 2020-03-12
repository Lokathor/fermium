use fermium::*;

/// Asserts the existence of a function with a given name and type.
///
/// * **Usage:** `assert_fn!(name: type);`
///
/// The assertion is a static check, so if there's a problem it will
/// cause a compilation failure rather than a runtime error.
macro_rules! assert_fn {
  ($f:ident : $t:ty) => {
    const _: $t = $f;
  };
}
