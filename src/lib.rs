pub mod template;

// Yoinked and modified from https://users.rust-lang.org/t/more-efficient-iteration-through-cells-in-a-grid/20045/6
pub struct Grid<T> {
    dim: (usize, usize),
    data: Vec<T>,
}

pub type Pos = (usize, usize); // written as (row, column)

impl<T> Grid<T> {
    pub fn new(dim: (usize, usize), data: Vec<T>) -> Self {
        assert_eq!(dim.0 * dim.1, data.len());
        Grid { dim, data }
    }

    pub fn flat_index(&self, pos: Pos) -> usize {
        assert!(pos.0 < self.dim.0);
        assert!(pos.1 < self.dim.1);

        let strides = (self.dim.1, 1);

        pos.0 * strides.0 + pos.1 * strides.1
    }

    pub fn pos(&self, index: usize) -> Pos {
        (index / self.dim.1, index % self.dim.1)
    }

    pub fn at_pos(&self, pos: Pos) -> &T {
        let idx = self.flat_index(pos);
        &self.data[idx]
    }

    pub fn at_pos_mut(&mut self, pos: Pos) -> &mut T {
        let idx = self.flat_index(pos);
        &mut self.data[idx]
    }

    pub fn neighbors(&self, pos: Pos) -> Vec<&T> {
        let mut out = Vec::with_capacity(4);
        self.each_neighbor_pos(pos, |neighbor| {
            out.push(self.at_pos(neighbor));
        });
        out
    }

    pub fn each_neighbor_pos(&self, pos: Pos, mut func: impl FnMut(Pos)) {
        if pos.0 != 0 {
            func((pos.0 - 1, pos.1));
        }
        if pos.1 != 0 {
            func((pos.0, pos.1 - 1));
        }

        if pos.0 + 1 != self.dim.0 {
            func((pos.0 + 1, pos.1));
        }
        if pos.1 + 1 != self.dim.1 {
            func((pos.0, pos.1 + 1));
        }

        if pos.0 != 0 && pos.1 != 0 {
            func((pos.0 - 1, pos.1 - 1));
        }
        if pos.0 != 0 && pos.1 + 1 != self.dim.1 {
            func((pos.0 - 1, pos.1 + 1));
        }

        if pos.0 + 1 != self.dim.0 && pos.1 != 0 {
            func((pos.0 + 1, pos.1 - 1));
        }
        if pos.0 + 1 != self.dim.0 && pos.1 + 1 != self.dim.1 {
            func((pos.0 + 1, pos.1 + 1));
        }
    }
}
