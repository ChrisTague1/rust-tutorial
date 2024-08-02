//! # The Project
//!
//! `the-project` is the best crate ever made

/// Adds one to the number given.
/// This should be good for people learning addition.
///
/// > For those who would like an add_two function, please consider calling this function twice.
/// 
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = the_project::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
