#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Solve {
    None,
    Partial,
    Full,
    Unknown,
}

pub trait Solvable {
    fn solve(&mut self) -> Solve;
}