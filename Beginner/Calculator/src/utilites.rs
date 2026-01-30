#[derive(Clone, Copy)]
pub enum CalcKey {
    Number(u8),
    Delete,
    Clear,
    Percentage,
    Dot,
    Reverse,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

pub struct Util;

impl Util {
    fn parse(input: String) {}
}
