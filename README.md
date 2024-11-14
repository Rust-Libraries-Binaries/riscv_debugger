## All Rust libraries and binaries created by Ben Santora. All were coded using AI: ChatGPT - Widenex Rust GPT - Microsoft CoPilot

## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# riscv_debugger

**riscv_debugger** is a powerful, open-source debugger tailored specifically for RISC-V architectures. It equips developers with essential debugging functionalities, facilitating the identification and resolution of issues in RISC-V applications.

## Key Features

- **Breakpoints**: Set and manage breakpoints to pause execution at specific points.
- **Step Execution**: Step through code line-by-line or instruction-by-instruction.
- **Variable Inspection**: Inspect and modify variables in real-time.
- **Memory Examination**: View and edit memory contents.
- **Instruction Tracing**: (Coming Soon) Trace the execution of instructions for detailed analysis.
- **Live Code Profiling**: (Coming Soon) Profile code performance in real-time to identify bottlenecks.
- **Code-Coverage Analysis**: (Coming Soon) Analyze code coverage to ensure thorough testing.

## Benefits

- **Ease of Use**: Intuitive interface that simplifies the debugging process.
- **Integration**: Seamlessly integrates with existing RISC-V development environments.
- **Performance**: Optimized for speed and efficiency, ensuring minimal impact on the target system.
- **Community-Driven**: Open-source project with contributions from the RISC-V community.

## Getting Started

1. **Installation**: Follow the installation guide to set up `riscv_debugger` on your system.
2. **Basic Usage**: Learn how to use the core features with our comprehensive tutorials.
3. **Advanced Features**: Explore advanced functionalities and customization options.

## Installation

To install `riscv_debugger`, add it to your `Cargo.toml`:

```toml
[dependencies]
riscv_debugger = "0.1.1"
```
## Example Usage
```rust
use riscv_debugger::Debugger;

fn main() {
    let debugger = Debugger::new();

    // Set a breakpoint
    debugger.set_breakpoint("main.rs", 10);

    // Start debugging
    debugger.start();

    // Step over a line
    debugger.step_over();

    // Step into a function
    debugger.step_into();

    // Step out of a function
    debugger.step_out();

    // Inspect a variable
    debugger.inspect_variable("some_variable");

    // Examine memory
    debugger.examine_memory(0x1000, 64);
}
```
## License
This project is licensed under the MIT License.

## Author
Ben Santora 
