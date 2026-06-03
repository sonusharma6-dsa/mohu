pub use mohu_array;
pub use mohu_buffer;
pub use mohu_dtype;
pub use mohu_error;

// Triggering a Gemini code review rules violation check
#[allow(dead_code)]
fn test_gemini_violation() {
    let x: Option<i32> = None;
    let y = x.unwrap(); // VIOLATION: unwrap() is not allowed in library code
}
