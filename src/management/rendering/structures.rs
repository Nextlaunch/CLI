#[derive(Debug, Clone)]
pub struct Frame {
    pub rows: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub width: usize,
    pub height: usize,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub align: Alignment,
    pub length: usize,
    pub content: String,
    pub title: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Middle,
    Left,
    Right,
    Split,
}