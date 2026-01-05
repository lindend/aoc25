use std::io::Lines;

use crate::util::vec2::Vec2;

#[derive(Clone)]
pub struct Grid<T> {
    pub cells: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            cells: vec![default; width * height],
            width,
            height,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn from_lines<'a, TL: Fn(char) -> T>(
        lines: impl Iterator<Item = &'a str>,
        from_str: TL,
    ) -> Self {
        let mut height = 0;
        let mut width = 0;

        let mut cells = Vec::new();

        for line in lines {
            if height == 0 {
                width = line.len();
            }

            height += 1;
            for c in line.chars() {
                cells.push(from_str(c));
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn from_str<TL: Fn(char) -> T>(input: &str, from_str: TL) -> Self {
        Grid::from_lines(input.lines(), from_str)
    }
}

static NEIGHBOUR_OFFSETS: [Vec2<i64>; 8] = [
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: 1 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: 1, y: 1 },
    Vec2 { x: -1, y: -1 },
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 1, y: -1 },
];

impl<T: Clone> Grid<T> {
    pub fn at(&self, x: i64, y: i64) -> Option<T> {
        if x < 0 || x >= self.width as i64 || y < 0 || y >= self.height as i64 {
            None
        } else {
            Some(self.cells[(x + y * self.width as i64) as usize].clone())
        }
    }

    pub fn neighbours(&self, x: i64, y: i64) -> impl Iterator<Item = (Vec2<i64>, T)> {
        NEIGHBOUR_OFFSETS
            .iter()
            .filter_map(move |n| match self.at(n.x + x, n.y + y) {
                Some(v) => Some((n.clone(), v)),
                None => None,
            })
    }

    pub fn update(&mut self, x: i64, y: i64, v: T) {
        self.cells[(x + y * self.width as i64) as usize] = v;
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Vec2<i64>, &T)> {
        self.cells.iter().enumerate().map(|(i, t)| {
            (
                Vec2::new((i % self.width) as i64, (i / self.width) as i64),
                t,
            )
        })
    }
}
