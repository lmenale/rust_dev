/// This is a simple Rust module to demonstrate documentation with Rustdoc.
///
/// # Examples
///
/// ```
/// use rustdoc_demo::greet;
/// let message = greet("Alice");
/// assert_eq!(message, "Hello, Alice!");
/// ```
pub mod rustdoc_demo {
  /// Generates a greeting message.
  ///
  /// Takes a name as an argument and returns a formatted greeting string.
  ///
  /// # Arguments
  ///
  /// * `name` - The name to greet.
  ///
  /// # Returns
  ///
  /// A string containing the greeting message.
  ///
  /// # Examples
  ///
  /// ```
  /// let message = rustdoc_demo::greet("Bob");
  /// assert_eq!(message, "Hello, Bob!");
  /// ```
  pub fn greet(name: &str) -> String {
      format!("Hello, {}!", name)
  }
}
