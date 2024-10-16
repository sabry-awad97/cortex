# Cortex: A Brainfuck Interpreter

Cortex is a robust and efficient Brainfuck interpreter implemented in Rust. This project demonstrates advanced programming concepts and showcases the power and flexibility of the Rust programming language.

## Features

- Full support for the Brainfuck programming language
- Efficient memory management
- Comprehensive error handling
- Cross-platform compatibility

## Installation

To install Cortex, make sure you have Rust and Cargo installed on your system. Then, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/cortex.git
cd cortex
cargo build --release
```

## Usage

To run a Brainfuck program using Cortex, use the following command:

```bash
cargo run -- path/to/your/brainfuck/file.bf
```

For example:

```bash
cargo run -- examples/hello.bf
```

## Examples

We've included several example Brainfuck programs in the `examples/` directory:

- `hello.bf`: Prints "Hello, World!"
- `add.bf`: Adds two numbers
- `fibonacci.bf`: Generates Fibonacci sequence

Feel free to use these examples to test Cortex or as a starting point for your own Brainfuck programs.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## About Brainfuck

Brainfuck is an esoteric programming language created in 1993 by Urban Müller. It consists of only eight simple commands and an instruction pointer. Despite its minimalism, it's Turing-complete.

### Basic Concepts

1. **Memory Model**

   - The program operates on an array of 30,000 byte cells, initialized to zero.
   - There's a pointer that starts at the first cell and can move left or right.

2. **Commands**

   - `>` : Move the pointer to the right
   - `<` : Move the pointer to the left
   - `+` : Increment the current cell by 1
   - `-` : Decrement the current cell by 1
   - `.` : Output the ASCII character of the current cell
   - `,` : Input a character and store its ASCII value in the current cell
   - `[` : If the current cell is zero, jump to the matching `]`
   - `]` : If the current cell is nonzero, jump back to the matching `[`

3. **Program Structure**

   - A Brainfuck program is a sequence of these commands.
   - Any character that's not one of the eight commands is ignored (can be used for comments).

4. **Loops**
   - Loops are denoted by `[` and `]`.
   - They continue as long as the current cell is non-zero.

### Example Breakdown

Let's analyze a simple Brainfuck program that outputs "Hi":

```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

Breaking it down step by step:

1. `++++++++`

   - Add 8 to cell #0

2. `[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]`

   - This is the main loop that sets up the memory cells
   - It runs 8 times (the value we set in cell #0)
   - Inside this loop, we set up the ASCII values for 'H' (72) and 'i' (105)

3. `>>.`

   - Move the pointer to cell #2 and output it (ASCII 72, 'H')

4. `>---.`

   - Move to cell #3, subtract 3, and output it (ASCII 105, 'i')

5. `+++++++..+++.`

   - Add 7 to cell #3, output twice (ASCII 108, 'l')
   - Add 3 and output again (ASCII 111, 'o')

6. `>>.`

   - Move to cell #5 and output (ASCII 32, space)

7. `<-.`

   - Move back to cell #4, subtract 1, and output (ASCII 87, 'W')

8. `<.`

   - Move to cell #3 and output (ASCII 111, 'o')

9. `+++.------.--------.`

   - Modify and output cell #3 three more times:
     - Add 3 (ASCII 114, 'r')
     - Subtract 6 (ASCII 108, 'l')
     - Subtract 8 (ASCII 100, 'd')

10. `>>+.`
    - Move to cell #5, add 1, and output (ASCII 33, '!')

This program efficiently sets up the memory cells with the ASCII values for "Hello World!" and then outputs them character by character.

For more information on Brainfuck, check out the [Esolang Wiki](https://esolangs.org/wiki/Brainfuck).
