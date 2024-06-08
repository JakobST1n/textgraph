use std::ops::{Index, IndexMut};
//use std::io::IsTerminal;
//dbg!(std::io::stdout().is_terminal());

#[derive(Clone)]
pub enum GraphPixel {
    Normal(char),
    Green(char),
    Blank,
}

impl std::default::Default for GraphPixel {
    fn default() -> Self {
        GraphPixel::Blank
    }
}

impl std::fmt::Display for GraphPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GraphPixel::Normal(c) => format!("{}", c),
                GraphPixel::Green(c) => format!("\x1b[32m{}\x1b[0m", c),
                GraphPixel::Blank => String::from(" "),
            }
        )
    }
}

pub struct GraphCanvas<T> {
    elements: Vec<T>,
    width: usize,
    height: usize,
    draw_width: usize,
    draw_height: usize,
    col_offset: usize,
    row_offset: usize,
}

impl<T: Clone + Default + std::fmt::Display> GraphCanvas<T> {
    pub fn new(width: usize, height: usize) -> Self {
        GraphCanvas::new_default(T::default(), width, height)
    }

    pub fn new_default(default: T, width: usize, height: usize) -> Self {
        GraphCanvas {
            elements: vec![default; width * height],
            width,
            height,
            draw_width: width,
            draw_height: height,
            col_offset: 0,
            row_offset: 0,
        }
    }

    pub fn to_string(&self) -> String {
        let mut out = String::with_capacity(self.height * (self.width + 1));
        for (i, px) in self.elements.iter().enumerate() {
            out.push_str(&px.to_string());
            if (i + 1) % self.width == 0 && i < (self.height * self.width - 1) {
                out.push('\n');
            }
        }
        out
    }

    pub fn axis(&mut self, c1: T, c2: T, c3: T, c4: T, c5: T, c6: T) {
        if self.height < 2 || self.width < 2 {
            return;
        }
        for i in 0..self.height {
            self.elements[i * self.width] = c1.clone();
            self.elements[i * self.width + self.width - 1] = c1.clone();
        }
        for i in 1..self.width - 1 {
            self.elements[i] = c2.clone();
            self.elements[(self.height - 1) * self.width + i] = c2.clone();
        }
        self.elements[0] = c4.clone();
        self.elements[self.width - 1] = c6.clone();
        self.elements[(self.height - 1) * self.width] = c3.clone();
        self.elements[self.height * self.width - 1] = c5.clone();
        if self.draw_height > 2 {
            self.draw_height = self.height - 2;
        }
        if self.draw_width > 2 {
            self.draw_width = self.width - 2;
        }
        self.col_offset = 1;
        self.row_offset = 1;
    }

    pub fn width(&self) -> usize {
        self.draw_width
    }

    pub fn full_width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.draw_height
    }

    pub fn full_height(&self) -> usize {
        self.height
    }

    pub fn set(&mut self, x: usize, y: usize, px: T) {
        let pos = y * self.width + x;
        self.elements[pos] = px;
    }

    fn element_position(&self, row: usize, col: usize) -> usize {
        (row + self.row_offset) * self.width + (col + self.col_offset)
    }
}

impl<T: Clone + Default + std::fmt::Display> Index<(usize, usize)> for GraphCanvas<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.elements[self.element_position(row, col)]
    }
}

impl<T: Clone + Default + std::fmt::Display> IndexMut<(usize, usize)> for GraphCanvas<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        let pos = self.element_position(row, col);
        &mut self.elements[pos]
    }
}
