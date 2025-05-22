pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Greets the user with a message.
/// # Examples
/// ```
/// use effective_rust::item30::greet;
///
/// let name = "Alice";
/// let greeting = greet(name);
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
/// # Arguments
/// * `name` - A string slice that holds the name of the user.
/// # Returns
/// A string containing the greeting message.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn factorial(n: u128) -> u128 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_negative_and_positive() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn overflow_panics_in_debug() {
        let _ = add(i32::MAX, 1);
    }
}
