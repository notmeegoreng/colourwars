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
            self.val &= 0b11;
            self.val += 1;
            let c = self.val;
            self.val |= id;
            c == 0b11
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
pub struct State<const R: usize, const N: usize> {
    // row major order
    pub board: [Cell; N],
}

impl<const R: usize, const N: usize> Default for State<R, N> {
    fn default() -> Self {
        State {
            board: [Cell::default(); N],
        }
    }
}

impl<const R: usize, const N: usize> State<R, N> {
    fn inc(&mut self, idx: usize, id: u8, v: &mut Vec<usize>) {
        if self.board[idx].inc(id) {
            println!("pop at {}", idx);
            self.board[idx].clear();
            let i = idx % R;
            if i != 0 {
                v.push(idx - 1);
            }
            if idx >= R {
                v.push(idx - R);
            }
            if i != R - 1 {
                v.push(idx + 1);
            }
            if idx + R < N {
                v.push(idx + R)
            }
        }
    }

    pub fn inc_all(&mut self, idx: usize, id: u8) {
        let mut c: Vec<usize> = vec![idx];
        let mut n: Vec<usize> = vec![];
        while !c.is_empty() {
            for idx in c {
                self.inc(idx, id, &mut n);
            }
            c = n;
            n = vec![];
        }
    }

    pub fn index(&self, i: usize, j: usize) -> usize {
        i + j * R
    }
}
