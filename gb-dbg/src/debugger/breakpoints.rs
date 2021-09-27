
pub struct BreakpointEditor<MEM, REG> {
    breakpoints: Vec<Breakpoint>,
}

enum Breakpoint {
    Address(u16),
}
