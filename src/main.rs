mod board;
mod search;

pub fn main() {
    let mut s: board::Board<3, 9, 2> = board::Board::default();
    let i = s.index(1, 1);
    s.set(i, 0);
    s.inc_all(i, 0);
    println!("{:?}", s);
}
