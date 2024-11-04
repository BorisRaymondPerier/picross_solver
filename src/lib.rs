#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

pub mod image;
pub mod clues;
pub mod display;
pub mod picross;
pub mod solver;