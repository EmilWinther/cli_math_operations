CLI Math Operations

A command-line interface (CLI) tool for performing basic mathematical operations: addition, subtraction, multiplication, and division.
Features

    Supports four basic mathematical operations: addition, subtraction, multiplication, and division.
    Built with Rust and utilizes the clap crate for argument parsing.
    Provides clear error messages for invalid inputs and operations.
Prerequisites:

Before proceeding with the installation, ensure that you have both Rust and Cargo installed on your machine. If not, you can install them using the provided link.
Installation

    Ensure you have Rust and Cargo installed on your machine.
    Clone this repository:


git clone https://github.com/soupyegg/cli-math-operations.git

Navigate to the project directory and build the project:


    cd cli-math-operations
    cargo build --release

If you want to run it directly in the terminal, you can run:
cargo install --path .

By using cargo install --path ., you're installing the binary crate locally, which means the executable will be placed in the ~/.cargo/bin directory.
If ~/.cargo/bin is in the system's PATH (which it typically is if you've set up Rust and Cargo correctly),
you can run the tool directly from the terminal without specifying the ./ prefix.

Usage

Run the tool with the following syntax:

./calc -o [operation] [operand1] [operand2]

Arguments

    -o, --operation: The mathematical operation to perform. Possible values: add, subtract, multiply, divide.
    operand1: The first operand for the operation.
    operand2: The second operand for the operation.

Examples

./calc -o add 5 3
Result: 8

./calc -o divide 8 0
Error: Division by zero is not allowed.

Contributing

Contributions are welcome! Please open an issue or submit a pull request if you'd like to contribute.
