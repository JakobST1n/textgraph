const ASCII_0: char = '─';
const ASCII_1: char = '│';
const ASCII_2: char = '╭';
const ASCII_3: char = '╰';
const ASCII_4: char = '╮';
const ASCII_7: char = '╯';

/// Convenience function for converting a bitstring to a 6dot braille unicode character (brc: braille char)
///
/// # Arguments
///
/// - `i` - Bitstring, representing the dots of the braille character as below:
///         1  4 | 1  4
///         2  5 | 2  5
///         3  6 | 3  6
///              | 7  8
///         If this only supported the 6dot, it could have used u8
///         the brr function is useful for mapping a sensible way to use dot8, to the actual format
/// - `btype` - Which braile variant the bitstring represents (dot6 or dot8)
pub fn brc(i: u32) -> char {
    const BRAILLE_UNICODE_OFFSET: u32 = 0x2800;
    if i == 0 {
        ' '
    } else if i < 255 {
        std::char::from_u32(BRAILLE_UNICODE_OFFSET + i as u32).unwrap()
    } else {
        ' '
    }
}

/// Map sensible braille mapping to legacy (actual) mapping (brr : braille real)
fn brr(i: usize, btype: &BrailleType) -> usize {
    match btype {
        BrailleType::dot6 => i,
        BrailleType::dot8 => match i {
            3 => 6,
            4 => 3,
            5 => 4,
            6 => 5,
            _ => i,
        },
    }
}

enum Pixel {
    Char(char),
    Braille(u32),
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Char(c) => format!("{}", c),
                Pixel::Braille(i) => format!("{}", brc(*i)),
            }
        )
    }
}

#[derive(Clone)]
#[allow(dead_code)]
enum GraphPixel<T> {
    Normal(T),
    Green(T),
    Blue(T),
    Red(T),
    Yellow(T),
    Magenta(T),
    Cyan(T),
    Blank,
}

impl<T> std::default::Default for GraphPixel<T> {
    fn default() -> Self {
        GraphPixel::Blank
    }
}

impl<T: std::fmt::Display> std::fmt::Display for GraphPixel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GraphPixel::Normal(c) => format!("{}", c),
                GraphPixel::Green(c) => format!("\x1b[32m{}\x1b[0m", c),
                GraphPixel::Blue(c) => format!("\x1b[34m{}\x1b[0m", c),
                GraphPixel::Red(c) => format!("\x1b[31m{}\x1b[0m", c),
                GraphPixel::Yellow(c) => format!("\x1b[33m{}\x1b[0m", c),
                GraphPixel::Magenta(c) => format!("\x1b[33m{}\x1b[0m", c),
                GraphPixel::Cyan(c) => format!("\x1b[36m{}\x1b[0m", c),
                GraphPixel::Blank => String::from(" "),
            }
        )
    }
}

#[derive(PartialEq, Clone)]
pub enum BrailleType {
    dot6,
    dot8,
}

/// Available options for how the graph should look
#[derive(PartialEq, Clone)]
pub enum GraphType {
    /// Use only * symbols
    Star,
    /// Use pretty characters from the ascii range
    Ascii,
    /// Draw using braille unicode characters
    Braille(BrailleType),
}

impl std::default::Default for GraphType {
    fn default() -> Self {
        GraphType::Star
    }
}

impl GraphType {
    fn btype(&self) -> BrailleType {
        match self {
            GraphType::Braille(b) => b.clone(),
            _ => panic!("cannot get btype on non Braille GraphType"),
        }
    }
}

/// Temporary variables used while building a graph
#[allow(dead_code)]
pub struct GraphBuilder {
    /// A array of pixels, this will ultimately be turned to a string, is initialized to width * height
    elements: Vec<GraphPixel<char>>,
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
    /// The values of the x-axis of the graph
    x_values: Vec<f64>,
    /// The values of the y-axis of the graph
    y_values: Vec<Vec<f64>>,
    /// Decides whether axis will be drawn on the resulting graph
    enable_axis: bool,
    /// Which GraphType to use when the graph is drawn
    graph_type: GraphType,
    /// Special case of running keep_tail once
    cut_overflow: bool,
    /// Whether or not to use color pixels
    enable_color: bool,
}

impl GraphBuilder {
    /// Create a new canvas with desired width and height
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the output canvas
    /// * `height` - Height of the output canvas
    pub fn new(x_values: &[f64], y_values: &[f64], width: usize, height: usize) -> Self {
        GraphBuilder {
            elements: vec![GraphPixel::default(); width * height],
            width,
            height,
            draw_width: width,
            draw_height: height,
            col_offset: 0,
            row_offset: 0,
            x_values: x_values.to_vec(),
            y_values: vec![y_values.to_vec()],
            enable_axis: false,
            graph_type: GraphType::default(),
            cut_overflow: false,
            enable_color: true,
        }
    }

    /// Enable or disable axis in output
    pub fn axis(&mut self, enable_axis: bool) -> &Self {
        self.enable_axis = enable_axis;
        self
    }

    /// Set graph type
    pub fn graph_type(&mut self, graph_type: GraphType) -> &Self {
        self.graph_type = graph_type;
        self
    }

    /// Enable or disable color
    pub fn color(&mut self, enable_color: bool) -> &Self {
        self.enable_color = enable_color;
        self
    }

    /// Delete all saved samples before the last n
    /// Assumes that y_values and x_values has the same length
    ///
    /// # Arguments
    ///
    /// * `n` - Number of samples to keep
    pub fn keep_tail(&mut self, n: usize) -> &Self {
        for i in 0..self.y_values.len() {
            if self.y_values[i].len() > n {
                self.y_values[i] = self.y_values[i][self.y_values[0].len() - n..].to_vec();
                self.x_values = self.x_values[self.x_values.len() - n..].to_vec();
            }
        }
        self
    }

    /// Enable cutting overflow, this works differently to keep_tail directly,
    /// as the axis-calculations must be performed first.
    /// So keep_tail is run once first, so we can keep a approximate window,
    /// and then another time to get it exactly right.
    ///
    /// # Arguments
    ///
    pub fn cut_overflow(&mut self, enable: bool) -> &Self {
        self.cut_overflow = enable;
        self
    }

    /// Build the actual graph,
    /// this is potentially a heavy operation, and it will mutate &self!
    /// If you want to only see the "current state", you should clone first!
    pub fn build(&mut self) -> String {
        if self.cut_overflow {
            if self.graph_type == GraphType::Braille(BrailleType::dot6) {
                self.keep_tail(self.draw_width * 2);
            } else if self.graph_type == GraphType::Braille(BrailleType::dot8) {
                self.keep_tail(self.draw_width * 2);
            } else {
                self.keep_tail(self.draw_width);
            }
        }

        //let min_x = self.x_values.iter().cloned().fold(f64::INFINITY, f64::min);
        //let max_x = self
        //    .x_values
        //    .iter()
        //    .cloned()
        //    .fold(f64::NEG_INFINITY, f64::max);
        let min_y = self.y_values[0]
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let max_y = self.y_values[0]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        if self.enable_axis {
            self.draw_axis(
                min_y,
                max_y,
                GraphPixel::Normal(ASCII_1),
                GraphPixel::Normal(ASCII_0),
                GraphPixel::Normal('└'),
                GraphPixel::Normal('┌'),
                GraphPixel::Normal('┘'),
                GraphPixel::Normal('┐'),
            );
        }

        // Run a second time after axis has been calculated properly
        if self.cut_overflow {
            if self.graph_type == GraphType::Braille(BrailleType::dot6) {
                self.keep_tail(self.draw_width * 2);
            } else if self.graph_type == GraphType::Braille(BrailleType::dot8) {
                self.keep_tail(self.draw_width * 2);
            } else {
                self.keep_tail(self.draw_width);
            }
        }

        if true {
            // && x_values.windows(2).all(|w| w[1] - w[0] == w[0] - w[1]) {
            self.downsample();
        } else {
            // If the sample size is not consistent, we should interpolate
            todo!("interpolation is not implemented");
            //interpolate(&y_values, &x_values, graph.width())
        };

        // Scale the data
        let mut scale_height = self.draw_height;
        if self.graph_type == GraphType::Braille(BrailleType::dot6) {
            scale_height = self.draw_height * 3;
        } else if self.graph_type == GraphType::Braille(BrailleType::dot8) {
            scale_height = self.draw_height * 4;
        }
        let scale_factor = (scale_height - 1) as f64 / (max_y - min_y);
        for i in 0..self.y_values[0].len() {
            self.y_values[0][i] = ((self.y_values[0][i] - min_y) * scale_factor).round();
        }

        match self.graph_type {
            GraphType::Star => self.draw_star(0),
            GraphType::Ascii => self.draw_ascii(0),
            GraphType::Braille(BrailleType::dot6) => self.draw_braille(0),
            GraphType::Braille(BrailleType::dot8) => self.draw_braille(0),
        }

        self.to_string()
    }

    // Downsample using a common downsampling, this allows us to avoid doing anything
    // with the x values.
    // Make sure to only use one downsampling-algorithm
    fn downsample(&mut self) {
        for g in 0..self.y_values.len() {
            let mut scale_width = self.draw_width;
            if self.graph_type == GraphType::Braille(BrailleType::dot6)
                || self.graph_type == GraphType::Braille(BrailleType::dot8)
            {
                scale_width *= 2;
            }
            if self.y_values[g].len() < scale_width {
                continue;
            }

            let factor = self.y_values[g].len() as f64 / scale_width as f64;
            let mut new_values = Vec::with_capacity(scale_width);
            for i in 0..scale_width {
                let new_value = self.y_values[g][(i as f64 * factor) as usize];
                new_values.push(new_value);
            }
            self.y_values[g] = new_values;
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

    // Method that takes a closure to decide which GraphPixel variant to create
    // A more customizable variant of the color! macro
    fn color_pixel<F>(&self, px: char, creator: F) -> GraphPixel<char>
    where
        F: FnOnce(char) -> GraphPixel<char>,
    {
        if self.enable_color {
            creator(px)
        } else {
            GraphPixel::Normal(px)
        }
    }

    /// Set a pixel at a absolute position in the canvas
    ///
    /// # Argument
    ///
    /// * `x` - X-position of pixel
    /// * `y` - Y-position of pixel
    /// * `px` - The pixel to set
    fn draw_exact(&mut self, x: usize, y: usize, px: GraphPixel<char>) {
        let pos = y * self.width + x;
        self.elements[pos] = px;
    }

    /// Set a pixel in the drawable part of the canvas
    ///
    /// # Argument
    ///
    /// * `x` - Relative X-position of pixel
    /// * `y` - Relative Y-position of pixel
    /// * `px` - The pixel to set
    fn draw(&mut self, x: usize, y: usize, px: GraphPixel<char>) {
        let pos = (y + self.row_offset) * self.width + (x + self.col_offset);
        self.elements[pos] = px;
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
    fn draw_axis(
        &mut self,
        min_y: f64,
        max_y: f64,
        c1: GraphPixel<char>,
        c2: GraphPixel<char>,
        c3: GraphPixel<char>,
        c4: GraphPixel<char>,
        c5: GraphPixel<char>,
        c6: GraphPixel<char>,
    ) {
        let mut y_ticks: Vec<String> = Vec::with_capacity(self.height);
        let mut x_offset: usize = 0;
        for i in 0..self.height {
            let n = (min_y + (((max_y - min_y) / (self.height as f64 - 1.0)) * i as f64))
                .round()
                .to_string();
            if n.len() > x_offset {
                x_offset = n.len();
            }
            y_ticks.insert(0, n);
        }

        for i in 0..self.height {
            self.elements[i * self.width + x_offset] = c1.clone();
            self.elements[i * self.width + self.width - 1] = c1.clone();
            for (j, c) in y_ticks[i].chars().enumerate() {
                self.elements[i * self.width + j] = GraphPixel::Normal(c);
            }
        }
        for i in 1 + x_offset..self.width - 1 {
            self.elements[i] = c2.clone();
            self.elements[(self.height - 1) * self.width + i] = c2.clone();
        }
        self.elements[x_offset] = c4.clone();
        self.elements[self.width - 1] = c6.clone();
        self.elements[(self.height - 1) * self.width + x_offset] = c3.clone();
        self.elements[self.height * self.width - 1] = c5.clone();
        if self.draw_height > 2 {
            self.draw_height = self.height - 2;
        }
        if self.draw_width > 2 {
            self.draw_width = self.width - 2 - x_offset;
        }
        self.col_offset = x_offset + 1;
        self.row_offset = 1;
    }

    /// Draw a graph using * for the pixels of the graph
    fn draw_star(&mut self, g: usize) {
        for i in 0..self.y_values[g].len() {
            let y = self.draw_height - (self.y_values[g][i] as usize) - 1;
            self.draw(i, y, self.color_pixel('*', |px| GraphPixel::Green(px)));
        }
    }

    /// Draw a graph using somewhat pretty ascii characters for pixels of the graph
    pub fn draw_ascii(&mut self, g: usize) {
        if self.enable_axis {
            self.draw_exact(
                self.col_offset - 1,
                self.draw_height - self.y_values[g][0] as usize,
                self.color_pixel('├', |px| GraphPixel::Green(px)),
            );
            self.draw_exact(
                self.width - 1,
                self.draw_height - self.y_values[g][self.y_values[g].len() - 1] as usize,
                self.color_pixel('┤', |px| GraphPixel::Green(px)),
            );
        }
        for i in 0..self.y_values[g].len() {
            let y1 = self.draw_height - (self.y_values[g][i] as usize) - 1;
            let y2 = if i < self.y_values[g].len() - 1 {
                self.draw_height - (self.y_values[g][i + 1] as usize) - 1
            } else {
                y1
            };

            if y1 == y2 {
                self.draw(i, y1, self.color_pixel(ASCII_0, |px| GraphPixel::Green(px)));
            } else if y1 > y2 {
                self.draw(i, y1, self.color_pixel(ASCII_7, |px| GraphPixel::Green(px)));
                self.draw(i, y2, self.color_pixel(ASCII_2, |px| GraphPixel::Green(px)));
                for j in (y2 + 1)..y1 {
                    self.draw(i, j, self.color_pixel(ASCII_1, |px| GraphPixel::Green(px)));
                }
            } else {
                self.draw(i, y1, self.color_pixel(ASCII_4, |px| GraphPixel::Green(px)));
                self.draw(i, y2, self.color_pixel(ASCII_3, |px| GraphPixel::Green(px)));
                for j in (y1 + 1)..y2 {
                    self.draw(i, j, self.color_pixel(ASCII_1, |px| GraphPixel::Green(px)));
                }
            }
        }
    }

    /// Draw a graph using braille characters, this assumes the graph is scaled for 6dot characters
    /// meaning width is divided by 2, and height is divided by 3
    fn draw_braille(&mut self, g: usize) {
        let mut y_scale = 1;
        let x_scale = 2;

        let btype = self.graph_type.btype();
        if self.graph_type == GraphType::Braille(BrailleType::dot6) {
            y_scale = 3;
        }
        if self.graph_type == GraphType::Braille(BrailleType::dot8) {
            y_scale = 4;
        }

        let mut i = 0;
        while i < self.y_values[g].len() - 1 {
            let y1 = (self.draw_height * y_scale) - (self.y_values[g][i] as usize) - 1;
            let y1_abs = y1 / y_scale;
            let y2 = (self.draw_height * y_scale) - (self.y_values[g][i + 1] as usize) - 1;
            let y2_abs = y2 / y_scale;

            let pxx1 = 1 << brr(y1 % y_scale, &btype);
            let pxx2 = 1 << brr((y2 % y_scale) + y_scale, &btype);
            if y1_abs == y2_abs {
                self.draw(
                    i / x_scale,
                    y1_abs,
                    self.color_pixel(brc(pxx1 | pxx2), |px| GraphPixel::Green(px)),
                );
            } else {
                self.draw(
                    i / x_scale,
                    y1_abs,
                    self.color_pixel(brc(pxx1), |px| GraphPixel::Green(px)),
                );
                self.draw(
                    i / x_scale,
                    y2_abs,
                    self.color_pixel(brc(pxx2), |px| GraphPixel::Green(px)),
                );
            }
            i += 2;
        }
    }
}

// /// A better way to downsize, heavier and more complex, but should be used when sample speed is uneven.
// ///
// /// # Arguments
// ///
// /// * `y_values` - The y values that should be downsampled
// /// * `x_values` - X values, needed to interpolate while keeping sample distance
// /// * `column_count` - Desired resolution of the output
// pub fn interpolate(y_values: &[f64], x_values: &[f64], column_count: usize) -> Vec<f64> {
//     let min_x = x_values.iter().cloned().fold(f64::INFINITY, f64::min);
//     let max_x = x_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
//     let step = (max_x - min_x) / (column_count as f64 - 1.0);
//     let mut interpolated_data = Vec::new();
//
//     for i in 0..column_count {
//         let target_mark = min_x + i as f64 * step;
//         let mut j = 0;
//         while j < x_values.len() - 1 && x_values[j + 1] < target_mark {
//             j += 1;
//         }
//         let t0 = x_values[j];
//         let t1 = x_values[j + 1];
//         let d0 = y_values[j];
//         let d1 = y_values[j + 1];
//         let value = d0 + (d1 - d0) * (target_mark - t0) / (t1 - t0);
//         interpolated_data.push(value);
//     }
//
//     interpolated_data
// }
