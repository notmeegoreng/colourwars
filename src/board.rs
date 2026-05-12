use std::fmt;

#[derive(Default, Copy, Clone)]
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
            (self.val & !0b11) >> 2,
            self.count()
        )?;
        Ok(())
    }
}

impl Cell {
    fn count(&self) -> u8 {
        if self.val == 0 {
            0
        } else {
            (self.val & 0b11) + 1
        }
    }

    fn inc(&mut self, id: u8) -> bool {
        if self.val == 0 {
            self.val = id;
            false
        } else {
            let c = self.val & 0b11;
            if c == 0b11 {
                // already popping, ignore
                return false;
            }
            self.val = (c + 1) | id;
            c == 0b10
        }
    }

    // set cell value to 3
    pub fn set(&mut self, id: u8) {
        self.val = id | 0b10;
    }

    fn clear(&mut self) {
        self.val = 0;
    }

    fn check(&self, id: u8) -> bool {
        self.val & !0b11 == id
    }
}

#[derive(Debug, Clone)]
pub struct Board<const R: usize, const N: usize> {
    // row major order
    pub board: [Cell; N],
}

impl<const R: usize, const N: usize> Default for Board<R, N> {
    fn default() -> Self {
        Board {
            board: [Cell::default(); N],
        }
    }
}

impl<const R: usize, const N: usize> Board<R, N> {
    fn inc(&mut self, idx: usize, id: u8, v: &mut Vec<usize>) {
        if self.board[idx].inc(id) {
            v.push(idx);
        }
    }

    pub fn inc_all(&mut self, idx: usize, id: u8) {
        // next to pop
        let mut c: Vec<usize> = vec![];
        self.inc(idx, id, &mut c);
        while !c.is_empty() {
            let mut n = vec![];
            for idx in c {
                self.board[idx].clear();
                let i = idx % R;
                if i != 0 {
                    self.inc(idx - 1, id, &mut n);
                }
                if idx >= R {
                    self.inc(idx - R, id, &mut n);
                }
                if i != R - 1 {
                    self.inc(idx + 1, id, &mut n);
                }
                if idx + R < N {
                    self.inc(idx + R, id, &mut n)
                }
            }
            c = n;
        }
    }

    // self is not needed, but having it be a method is easier to use
    pub fn index(&self, i: usize, j: usize) -> usize {
        i + j * R
    }
}
