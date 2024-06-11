//use std::io::IsTerminal;
//dbg!(std::io::stdout().is_terminal());

const ASCII_0: char = '─';
const ASCII_1: char = '│';
const ASCII_2: char = '╭';
const ASCII_3: char = '╰';
const ASCII_4: char = '╮';
const ASCII_7: char = '╯';

#[derive(Clone)]
#[allow(dead_code)]
enum GraphPixel<T> {
    Normal(T),
    Green(T),
    Blue(T),
    Red(T),
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
                GraphPixel::Blue(c) => format!("\x1b[33m{}\x1b[0m", c),
                GraphPixel::Red(c) => format!("\x1b[31m{}\x1b[0m", c),
                GraphPixel::Blank => String::from(" "),
            }
        )
    }
}

/// Available options for how the graph should look
#[derive(Clone)]
pub enum GraphType {
    /// Use only * symbols
    Star,
    /// Use pretty characters from the ascii range
    Ascii,
}

impl std::default::Default for GraphType {
    fn default() -> Self {
        GraphType::Star
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
    y_values: Vec<f64>,
    /// Decides whether axis will be drawn on the resulting graph
    enable_axis: bool,
    /// Which GraphType to use when the graph is drawn
    graph_type: GraphType,
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
            y_values: y_values.to_vec(),
            enable_axis: false,
            graph_type: GraphType::default(),
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

    /// Build the actual graph,
    /// this is potentially a heavy operation, and it will mutate &self!
    /// If you want to only see the "current state", you should clone first!
    pub fn build(&mut self) -> String {
        //let min_x = self.x_values.iter().cloned().fold(f64::INFINITY, f64::min);
        //let max_x = self
        //    .x_values
        //    .iter()
        //    .cloned()
        //    .fold(f64::NEG_INFINITY, f64::max);
        let min_y = self.y_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_y = self
            .y_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        if self.enable_axis {
            self.draw_axis(
                GraphPixel::Normal(ASCII_1),
                GraphPixel::Normal(ASCII_0),
                GraphPixel::Normal('└'),
                GraphPixel::Normal('┌'),
                GraphPixel::Normal('┘'),
                GraphPixel::Normal('┐'),
            );
        }

        if true {
            // && x_values.windows(2).all(|w| w[1] - w[0] == w[0] - w[1]) {
            if self.y_values.len() >= self.draw_width {
                // Downsample using a common downsampling, this allows us to avoid doing anything
                // with the x values

                let factor = self.y_values.len() as f64 / self.draw_width as f64;
                let mut new_values = Vec::with_capacity(self.draw_width);
                for i in 0..self.draw_width {
                    let new_value = self.y_values[(i as f64 * factor) as usize];
                    new_values.push(new_value);
                }
                self.y_values = new_values;
            }
        } else {
            // If the sample size is not consistent, we should interpolate
            todo!("interpolation is not implemented");
            //interpolate(&y_values, &x_values, graph.width())
        };

        // Scale the data
        let scale_factor = (self.draw_height - 1) as f64 / (max_y - min_y);
        for i in 0..self.y_values.len() {
            self.y_values[i] = ((self.y_values[i] - min_y) * scale_factor).round();
        }

        match self.graph_type {
            GraphType::Star => self.draw_star(),
            GraphType::Ascii => self.draw_ascii(),
        }

        self.to_string()
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
        c1: GraphPixel<char>,
        c2: GraphPixel<char>,
        c3: GraphPixel<char>,
        c4: GraphPixel<char>,
        c5: GraphPixel<char>,
        c6: GraphPixel<char>,
    ) {
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

    /// Draw a graph using * for the pixels of the graph
    fn draw_star(&mut self) {
        for i in 0..self.y_values.len() {
            let y = self.draw_height - (self.y_values[i] as usize) - 1;
            self.draw(i, y, GraphPixel::Normal('*'));
        }
    }

    /// Draw a graph using somewhat pretty ascii characters for pixels of the graph
    pub fn draw_ascii(&mut self) {
        if self.enable_axis {
            self.draw_exact(0, self.draw_height - self.y_values[0] as usize, GraphPixel::Green('├'));
            self.draw_exact(
                self.width - 1,
                self.height - self.y_values[self.y_values.len() - 1] as usize,
                GraphPixel::Green('┤'),
            );
        }
        for i in 0..self.y_values.len() {
            let y1 = self.draw_height - (self.y_values[i] as usize) - 1;
            let y2 = if i < self.y_values.len() - 1 {
                self.draw_height - (self.y_values[i + 1] as usize) - 1
            } else {
                y1
            };

            if y1 == y2 {
                self.draw(i, y1, GraphPixel::Green(ASCII_0));
            } else if y1 > y2 {
                self.draw(i, y1, GraphPixel::Green(ASCII_7));
                self.draw(i, y2, GraphPixel::Green(ASCII_2));
                for j in (y2 + 1)..y1 {
                    self.draw(i, j, GraphPixel::Green(ASCII_1));
                }
            } else {
                self.draw(i, y1, GraphPixel::Green(ASCII_4));
                self.draw(i, y2, GraphPixel::Green(ASCII_3));
                for j in (y1 + 1)..y2 {
                    self.draw(i, j, GraphPixel::Green(ASCII_1));
                }
            }
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

//const _BRAILLE_1: char = '⣿';
//const BRAILLE_1_0: char = '⡀';
//const BRAILLE_1_1: char = '⣀';
//const BRAILLE_1_2: char = '⣀';
//const BRAILLE_2_0: char = '⡄';
//const BRAILLE_3_0: char = '⡆';
//const BRAILLE_4_0: char = '⡇';
// pub fn braille(y_values: &Vec<f64>, options: &GraphOptions) -> String {
//     let aspects = SeriesAspects::from(y_values);
//     let canvas = String::with_capacity((options.width * options.height) as usize);
//
//     /*
//     r = (max - min)
//     r' = (max' - min')
//     y' = (((y - min) * r') / r) + min'
//     */
//     let r = aspects.max - aspects.min;
//     let r_marked = options.height;
//
//     let norm_after = options.height;
//
//     //for (x, y) in y_values.iter().enumerate() {
//     //    let y = norm(y.clone(), 0.0, options.height);
//     //    let x = norm(x.clone(), 0.0, options.width);
//     //}
//
//     String::from("")
// }
