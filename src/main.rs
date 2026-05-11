mod board;

pub fn main() {
    let mut s: board::State<3, 9> = board::State::default();
    let i = s.index(1, 1);
    s.board[i].set(0b100);
    s.inc_all(i, 0b100);
    println!("{:?}", s);
}
