use riscv_debugger::Debugger;

fn main() {
    let debugger = Debugger::new();

    // Set a breakpoint
    debugger.set_breakpoint("main.rs", 10);

    // Start the debugger
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
