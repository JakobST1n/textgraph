use crate::graph_canvas::{GraphCanvas, GraphPixel};

const ASCII_0: char = '─';
const ASCII_1: char = '│';
const ASCII_2: char = '╭';
const ASCII_3: char = '╰';
const ASCII_4: char = '╮';
const ASCII_7: char = '╯';

#[derive(Debug)]
pub struct GraphOptions {
    pub width: u64,
    pub height: u64,
    pub interpolate: bool,
    pub axis: bool,
}

/// Simply downsample, not the most correct way, but will likely not be too bad.
///
/// # Arguments
///
/// * `y_values` - The y values that should be downsampled
/// * `column_count` - Desired resolution of the output
pub fn downsample(y_values: &[f64], column_count: usize) -> Vec<f64> {
    let factor = y_values.len() as f64 / column_count as f64;
    (0..column_count)
        .map(|i| y_values[(i as f64 * factor) as usize])
        .collect()
}

/// A better way to downsize, heavier and more complex, but should be used when sample speed is uneven.
///
/// # Arguments
///
/// * `y_values` - The y values that should be downsampled
/// * `x_values` - X values, needed to interpolate while keeping sample distance
/// * `column_count` - Desired resolution of the output
pub fn interpolate(y_values: &[f64], x_values: &[f64], column_count: usize) -> Vec<f64> {
    let min_x = x_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_x = x_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let step = (max_x - min_x) / (column_count as f64 - 1.0);
    let mut interpolated_data = Vec::new();

    for i in 0..column_count {
        let target_mark = min_x + i as f64 * step;
        let mut j = 0;
        while j < x_values.len() - 1 && x_values[j + 1] < target_mark {
            j += 1;
        }
        let t0 = x_values[j];
        let t1 = x_values[j + 1];
        let d0 = y_values[j];
        let d1 = y_values[j + 1];
        let value = d0 + (d1 - d0) * (target_mark - t0) / (t1 - t0);
        interpolated_data.push(value);
    }

    interpolated_data
}

/// Scale a value to a new scale, useful for y values which needs to be scaled to fit within a size
///
/// # Arguments
/// 
/// * `values` - The values to scale to a new height
/// * `row_count` - The desired range of the new values (0 -> row_count)
fn scale(values: &[f64], row_count: usize) -> Vec<usize> {
    let min_value = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_value = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let scale_factor = (row_count - 1) as f64 / (max_value - min_value);
    values
        .iter()
        .map(|&y| ((y - min_value) * scale_factor).round() as usize)
        .collect()
}

/// Prepare the values of a graph before graphing
/// by applying scaling and interpolation/downscaling
///
/// # Arguments
/// 
/// * `x_values` - Values of the x-axis, needed for interpolation
/// * `y_values` - Graph values
/// * `graph` - The graph object, needed for knowing the information about width and height
/// * `options` - GraphOptions, used for forced interpolation
pub fn prepare(
    y_values: &[f64],
    x_values: &[f64],
    graph: &GraphCanvas<GraphPixel>,
    options: &GraphOptions,
) -> Vec<usize> {
    let y_values = if !options.interpolate {
        // && x_values.windows(2).all(|w| w[1] - w[0] == w[0] - w[1]) {
        if y_values.len() >= graph.width() {
            downsample(&y_values, graph.width())
        } else {
            y_values.to_vec()
        }
    } else {
        interpolate(&y_values, &x_values, graph.width())
    };

    let scaled_data = scale(&y_values, graph.height());
    scaled_data
}

/// Draw a graph using * for the pixels of the graph
///
/// # Arguments
/// 
/// * `x_values` - Values of the x-axis
/// * `y_values` - Graph values
/// * `options` - GraphOptions, used for forced interpolation
pub fn star(y_values: &[f64], x_values: &[f64], options: &GraphOptions) -> String {
    let mut graph = GraphCanvas::new(options.width as usize, options.height as usize);
    if options.axis {
        graph.axis(
            GraphPixel::Normal(ASCII_1),
            GraphPixel::Normal(ASCII_0),
            GraphPixel::Normal('└'),
            GraphPixel::Normal('┌'),
            GraphPixel::Normal('┘'),
            GraphPixel::Normal('┐'),
        );
    }

    let y_values = prepare(y_values, x_values, &graph, options);
    for (i, &value) in y_values.iter().enumerate() {
        let y = graph.height() - value - 1;
        graph[(y, i)] = GraphPixel::Normal('*');
    }

    graph.to_string()
}

/// Draw a graph using somewhat pretty ascii characters for pixels of the graph
///
/// # Arguments
/// 
/// * `x_values` - Values of the x-axis
/// * `y_values` - Graph values
/// * `options` - GraphOptions, used for forced interpolation
pub fn ascii(y_values: &[f64], x_values: &[f64], options: &GraphOptions) -> String {
    let mut graph = GraphCanvas::new_default(
        GraphPixel::Blank,
        options.width as usize,
        options.height as usize,
    );
    if options.axis {
        graph.axis(
            GraphPixel::Normal(ASCII_1),
            GraphPixel::Normal(ASCII_0),
            GraphPixel::Normal('└'),
            GraphPixel::Normal('┌'),
            GraphPixel::Normal('┘'),
            GraphPixel::Normal('┐'),
        );
    }

    let y_values = prepare(y_values, x_values, &graph, options);
    if options.axis {
        graph.set(0, graph.height() - y_values[0], GraphPixel::Green('├'));
        graph.set(
            graph.full_width() - 1,
            graph.height() - y_values[y_values.len() - 1],
            GraphPixel::Green('┤'),
        );
    }
    for i in 0..y_values.len() {
        let y1 = graph.height() - y_values[i] - 1;
        let y2 = if i < y_values.len() - 1 {
            graph.height() - y_values[i + 1] - 1
        } else {
            y1
        };

        if y1 == y2 {
            graph[(y1, i)] = GraphPixel::Green(ASCII_0);
        } else if y1 > y2 {
            graph[(y1, i)] = GraphPixel::Green(ASCII_7);
            graph[(y2, i)] = GraphPixel::Green(ASCII_2);
            for j in (y2 + 1)..y1 {
                graph[(j, i)] = GraphPixel::Green(ASCII_1);
            }
        } else {
            graph[(y1, i)] = GraphPixel::Green(ASCII_4);
            graph[(y2, i)] = GraphPixel::Green(ASCII_3);
            for j in (y1 + 1)..y2 {
                graph[(j, i)] = GraphPixel::Green(ASCII_1);
            }
        }
    }

    graph.to_string()
}

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
