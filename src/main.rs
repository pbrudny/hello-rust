// The `add_numbers` function takes two 32-bit integers (`i32`) as inputs and returns their sum.
// In Rust, function signatures explicitly define parameter types and return types.
// In TypeScript, you would also define parameter and return types, but it’s more permissive about nullability
// and type inference compared to Rust.
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // In Rust, the last expression (without a semicolon) is the return value.
    // In TypeScript, you would use `return a + b;` to explicitly specify the return.
}

fn main() {
    // In Rust, variables are immutable by default unless you use `mut`.
    // In TypeScript, variables are mutable by default but can be declared immutable with `const`.
    let result = add_numbers(5, 10);

    // Printing to the console in Rust uses the `println!` macro with a formatting syntax.
    // In TypeScript, you would use `console.log(result)` for console output.
    println!("{}", result);
}

// In Rust tests are usually in the same file as code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        // Rust uses the `assert_eq!` macro to perform equality assertions during testing.
        // TypeScript typically uses testing frameworks like Jest or Mocha for assertions.
        // For example: `expect(addNumbers(2, 3)).toBe(5);` in Jest.
        assert_eq!(add_numbers(2, 3), 5); // Test a simple addition
        assert_eq!(add_numbers(0, 0), 0); // Test adding zeros
        assert_eq!(add_numbers(-2, 2), 0); // Test adding positive and negative numbers
        assert_eq!(add_numbers(-3, -7), -10); // Test adding two negative numbers
    }
}
