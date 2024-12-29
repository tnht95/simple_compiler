# Simple Compiler

## Key Features

### Tail Call Optimization (TCO)
The compiler implements Tail Call Optimization during code generation to improve the efficiency of recursive function calls. This feature:
- Eliminates stack frame growth for tail-recursive calls.
- Enhances performance for algorithms relying on recursion.
- Provides robust support for deeply recursive functions by preventing stack overflow.

### Constant Folding
Constant folding is implemented as a compile-time optimization that evaluates constant expressions. This reduces runtime computation by:
- Replacing constant expressions with precomputed values during compilation.
- Simplifying the generated code for improved execution performance.
- Reducing redundant operations in the output program.

For example, an expression such as `2 + 3 * 4` is evaluated to `14` during compilation.

## Project Structure

- **Lexer and Parser**: A front-end capable of converting source code into an Abstract Syntax Tree (AST).
- **Intermediate Representation (IR)**: Generates a streamlined, assembly-like IR optimized for further processing.
- **Optimization Passes**:
    - **Constant Folding**: Simplifies constant expressions directly within the IR.
- **Code Generation**: Outputs stack-based machine-like instructions, with TCO applied at this phase.

## Grammar
EBNF file can be found [here](/src/grammar.ebnf)

## Getting Started


### Building and Running

1. Clone the repository:
   ```bash
   git clone https://github.com/username/simple-compiler.git
   cd simple-compiler
   ```

2. Build the compiler:
   ```bash
   cargo run -r -- <source_code_file_path>
   ```


## Constant Folding Example 

### Input Program 
```plaintext
fn example(m: int) -> int{
    m = 9 + 10 + m;
    return m * 0 * 100;
};

example(5);
```

### Optimized Intermediate Representation (IR)
```plaintext
0 DECLARE(
    "example",
)
1 ENTER
2 STORE(
    "m",
)
3 PUSH(
    19,
)
4 LOAD(
    "m",
)
5 ADD
6 STORE(
    "m",
)
7 PUSH(
    0,
)
8 RET
9 EXIT
10 PUSH(
    5,
)
11 CALL(
    "example",
)

```

## TCO Example

### Input Program (Non-tail call)
```
fn factorial(n: int) -> int {
     if n == 0 {
        return 1;
     };

     return n * factorial(n - 1);
};

print(factorial(5));
```

### Virtual Machine Output
```
Allocate stack frame for function: "factorial"
Allocate stack frame for function: "factorial"
Allocate stack frame for function: "factorial"
Allocate stack frame for function: "factorial"
Allocate stack frame for function: "factorial"
Allocate stack frame for function: "factorial"
120
```

### Input Program (Tail Call)
```
fn factorial(n: int, acc: int) -> int {
    if n == 0 {
        return acc;
    } else {
        return factorial(n - 1, acc * n);
    };
};

print(factorial(5, 1));
```

### Virtual Machine Output
```
Allocate stack frame for function: "factorial"
Tail call - reuse stack frame for function: factorial
Tail call - reuse stack frame for function: factorial
Tail call - reuse stack frame for function: factorial
Tail call - reuse stack frame for function: factorial
Tail call - reuse stack frame for function: factorial
120
```



## Future Enhancements

| Feature                             | Status  | Notes                                                                                                     |
|-------------------------------------|---------|-----------------------------------------------------------------------------------------------------------|
| More control flow                   | Planned | Includes for, while, else if, etc...                                                                      |
| Negative number support             | Planned | Currently lacks handling for negative numbers                                                             |
| WebAssembly compile                 | Planned | Compile source code to WebAssembly                                                                        |
| Redundant code remove               | Planned | Will analyze and remove dead code                                                                         |
| Register-based code generation      | Planned | Transition from stack-based to register-based instructions for improved efficiency                        |
| SSA (Static Single Assignment) form | Planned | Implement SSA to simplify optimization and analysis                                                       |
| Inlining                            | Planned | Replace function calls with function body code to reduce overhead                                         |
| Peephole optimization               | Planned | Perform localized optimizations on small code segments                                                    |
| Control flow graph analysis         | Planned | Enable advanced optimizations by analyzing program flow                                                   |
| Extended data type support          | Planned | Add support for floating-point and other complex data types                                               |
| Assembly Code Generation            | Planned | Support compiling source code directly to specific assembly languages to create a fully compiled language |
