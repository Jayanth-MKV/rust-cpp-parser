# Rust Project - C++ Function Parser

This Rust project is a C++ function parser that converts C++ function code from an input file into a JSON representation. The output is saved in `output.json`.

## Usage

To run the project, follow these steps:

1. Make sure you have Rust and Cargo installed on your system.
2. Clone the repository or download the source code.
3. Navigate to the project directory in your terminal.
4. Run the following command to build the project:

   ```shell
   cargo build
   ```

   This will compile the Rust project and its dependencies.

5. Create an `input.cpp` file in the project folder. This file should contain the C++ function code you want to convert.
6. Run the following command in your terminal:

   ```shell
   cargo run -- input.cpp
   ```

   This will execute the Rust project and convert the C++ function code from the `input.cpp` file.

7. After running the command, the project will generate the output in the `output.json` file, which will contain the JSON representation of the C++ function code.

## Example

Suppose you have the following C++ function code in the `input.cpp` file:

```cpp
#include <iostream>

int main() {
    std::cout << "Hello, World!";
    return 0;
}
```

Running the command `cargo run -- input.cpp` will generate the `output.json` file with the following content:

```json
{
    "function": "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World!\";\n    return 0;\n}"
}
```

## Requirements

- Rust (and Cargo) installed on your system

## License

This project is licensed under the [MIT License](LICENSE).
```
