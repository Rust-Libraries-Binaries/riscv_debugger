pub struct Debugger;

impl Debugger {
    pub fn new() -> Self {
        Debugger
    }

    pub fn set_breakpoint(&self, file: &str, line: u32) {
        println!("Breakpoint set at {}:{}", file, line);
    }

    pub fn start(&self) {
        println!("Starting debugger...");
    }
}
