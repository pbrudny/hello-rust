# Rust Add Numbers

A simple Rust program that demonstrates how to write and test a function for adding two numbers. This project includes a unit test to ensure the function behaves correctly.

## Features

- Strongly typed function to add two integers.
- Unit tests to verify functionality.
- Simple example to help beginners learn Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

## Usage

### Running the Program

1. Clone the repository or create the file manually:
   ```bash
   git clone https://github.com/pbrudny/hello-rust
   cd hello-rust
   ```

2. Run the program:
   ```bash
   cargo run
   ```

   You should see the output:
   ```
   15
   ```

### Running the Tests

To ensure everything works as expected, run the tests:

1. Run the following command:
   ```bash
   cargo test
   ```

2. If everything is correct, the output will look like:
   ```
   running 1 test
   test tests::test_add_numbers ... ok
   ```

## Code Overview

### Main Function

The `add_numbers` function takes two integers as input and returns their sum. The `main` function calls this function and prints the result to the console.

```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add_numbers(5, 10);
    println!("{}", result);
}
```

### Unit Tests

Unit tests are included to verify the correctness of the `add_numbers` function. They check various cases, including positive, negative, and zero values.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(0, 0), 0);
        assert_eq!(add_numbers(-2, 2), 0);
        assert_eq!(add_numbers(-3, -7), -10);
    }
}
```

## Key Concepts

- **Strong Typing:** Explicit parameter and return types ensure safety at compile time.
- **Immutability:** Variables are immutable by default for safer and more predictable code.
- **Testing:** Built-in testing support with the `#[test]` attribute and assertion macros like `assert_eq!`.

## Comparisons to TypeScript

This program highlights differences between Rust and TypeScript:

| Feature           | Rust                            | TypeScript                       |
|-------------------|---------------------------------|----------------------------------|
| **Typing**         | Strong, strict, and explicit   | Strong but more permissive       |
| **Immutability**   | Immutable by default           | Mutable by default (`let`)       |
| **Performance**    | Compiled, high performance     | Transpiled, runtime-dependent    |
| **Testing**        | Built-in support               | Requires external libraries      |

## Contributing

Feel free to open an issue or submit a pull request if you'd like to contribute or suggest improvements.

## License

This project is licensed under the [MIT License](LICENSE).
