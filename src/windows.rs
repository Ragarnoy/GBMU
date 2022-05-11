#[derive(Debug, Clone)]
pub enum WindowType {
    Keybindings,
    Debugger(Option<Vec<String>>),
}
