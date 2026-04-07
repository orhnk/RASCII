#[derive(Clone, Debug)]
pub struct Cell {
    pub ch: String,
    pub color_pre: String,
    pub color_suf: String,
}

pub type Grid = Vec<Vec<Cell>>;
