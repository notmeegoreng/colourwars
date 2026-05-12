use std::fmt;

#[derive(Copy, Clone)]
pub struct Cell {
    // 2 lsb: count - 1
    // rest: id of player
    val: u8,
}

impl fmt::Debug for Cell {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "Cell {{ id: {}, count: {} }}",
            self.get_player(),
            self.count()
        )?;
        Ok(())
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { val: Self::BLANK }
    }
}

impl Cell {
    const BLANK: u8 = u8::MAX;

    fn is_empty(&self) -> bool {
        self.val == Self::BLANK
    }

    fn count(&self) -> u8 {
        if self.is_empty() {
            0
        } else {
            (self.val & 0b11) + 1
        }
    }

    // set cell value to 3
    pub fn set(&mut self, p: u8, cs: &mut [usize]) {
        assert!(self.is_empty());
        cs[p as usize] += 1;
        self.val = (p << 2) | 0b10;
    }

    fn inc(&mut self, p: u8, cs: &mut [usize]) -> bool {
        if self.is_empty() {
            self.val = p << 2;
            cs[p as usize] += 1;
            false
        } else {
            let c = self.val & 0b11;
            if c == 0b11 {
                // already popping, should be same player, ignore
                return false;
            }
            let cp = self.get_player();
            if cp == p {
                self.val += 1;
            } else {
                cs[cp as usize] -= 1;
                cs[p as usize] += 1;
                self.val = (c + 1) | (p << 2);
            }
            c == 0b10
        }
    }

    fn clear(&mut self) {
        self.val = Self::BLANK;
    }

    fn get_player(&self) -> u8 {
        if self.is_empty() {
            0
        } else {
            (self.val & !0b11) >> 2
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board<const R: usize, const N: usize, const P: usize> {
    // row major order
    board: [Cell; N],
    pub player_counts: [usize; P],
}

impl<const R: usize, const N: usize, const P: usize> Default for Board<R, N, P> {
    fn default() -> Self {
        Board {
            board: [Cell::default(); N],
            player_counts: [0; P],
        }
    }
}

impl<const R: usize, const N: usize, const P: usize> Board<R, N, P> {
    pub fn set(&mut self, idx: usize, player: u8) {
        self.board[idx].set(player, &mut self.player_counts);
    }

    fn inc(&mut self, idx: usize, player: u8, v: &mut Vec<usize>) {
        if self.board[idx].inc(player, &mut self.player_counts) {
            v.push(idx);
        }
    }

    pub fn inc_all(&mut self, idx: usize, player: u8) {
        // next to pop
        let mut c: Vec<usize> = vec![];
        self.inc(idx, player, &mut c);
        while !c.is_empty() {
            let mut n = vec![];
            let r = c.len();
            for idx in c {
                self.board[idx].clear();
                let i = idx % R;
                if i != 0 {
                    self.inc(idx - 1, player, &mut n);
                }
                if idx >= R {
                    self.inc(idx - R, player, &mut n);
                }
                if i != R - 1 {
                    self.inc(idx + 1, player, &mut n);
                }
                if idx + R < N {
                    self.inc(idx + R, player, &mut n)
                }
            }
            // remove popped
            self.player_counts[player as usize] -= r;
            c = n;
        }
    }

    // self is not needed, but having it be a method is easier to use
    pub fn index(&self, i: usize, j: usize) -> usize {
        i + j * R
    }
}
