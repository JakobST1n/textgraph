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

/// Temporary variables used while building a graph
pub struct GraphCanvas<T> {
    /// A array of pixels, this will ultimately be turned to a string, is initialized to width * height
    elements: Vec<T>,
    /// Width of canvas
    width: usize,
    /// Height of canvas
    height: usize,
    /// Width of the area of the canvas left for the actual graph
    draw_width: usize,
    /// Height of the area of the canvas left for the actual graph
    draw_height: usize,
    /// x-offset for where the graph draw area begins
    col_offset: usize,
    /// y-offset for where the graph draw area begins
    row_offset: usize,
}

impl<T: Clone + Default + std::fmt::Display> GraphCanvas<T> {

    /// Create a new canvas with desired width and height
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the output canvas
    /// * `height` - Height of the output canvas
    pub fn new(width: usize, height: usize) -> Self {
        GraphCanvas::new_default(T::default(), width, height)
    }

    /// Create a new canvas with desired width, height, and default canvas pixel
    ///
    /// # Arguments
    ///
    /// * `default` - Pixel to use for the "background" of the canvas
    /// * `width` - Width of the output canvas
    /// * `height` - Height of the output canvas
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

    /// Turn canvas into a string
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

    /// Add axis to the canvas and move graph drawing area inside axis
    ///
    /// # Arguments
    ///
    /// * `c1` - Horizontal axis lines
    /// * `c2` - Vertical axis lines
    /// * `c4` - Bottom left axis pixel
    /// * `c5` - Top left axis pixel
    /// * `c6` - Bottom right axis pixel
    /// * `c7` - Top right axis pixel
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

    /// Width of drawable area of graph
    pub fn width(&self) -> usize {
        self.draw_width
    }

    /// Total width of graph canvas
    pub fn full_width(&self) -> usize {
        self.width
    }

    /// Height of drawable area of graph
    pub fn height(&self) -> usize {
        self.draw_height
    }

    /// Total height of graph canvas
    pub fn full_height(&self) -> usize {
        self.height
    }

    /// Set a pixel at a absolute position in the canvas
    ///
    /// # Argument
    ///
    /// * `x` - X-position of pixel
    /// * `y` - Y-position of pixel
    /// * `px` - The pixel to set
    pub fn set(&mut self, x: usize, y: usize, px: T) {
        let pos = y * self.width + x;
        self.elements[pos] = px;
    }

    /// Get the absolite position of a character from a coordinate drawable part of the canvas 
    /// 
    /// # Argument
    ///
    /// * `x` - Relative X-position of pixel
    /// * `y` - Relative Y-position of pixel
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
