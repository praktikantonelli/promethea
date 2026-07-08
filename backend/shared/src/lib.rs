//! As-of-yet empty library crate for shared logic between desktop and web server backends
///
/// sample function from initializing library crate
#[must_use]
#[inline]
#[allow(clippy::arithmetic_side_effects, reason = "unused sample function")]
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
