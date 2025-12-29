#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Turn {
    X,
    O,
}
impl Turn {
    pub fn other(&mut self) -> Self {
        match self {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        }
    }
}
