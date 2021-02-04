#[derive(Debug, Clone)]
pub struct Frame {
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub width: u16,
    pub height: u16,
    pub content: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub align: Alignment,
    pub length: u16,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Middle,
    Left,
    Right,
}