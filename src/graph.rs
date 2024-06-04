const _BRAILLE_1: char = '⣿';

const ASCII_0: char = '─';
const ASCII_1: char = '│';
const ASCII_2: char = '╭';
const ASCII_3: char = '╰';
const ASCII_4: char = '╮';
const ASCII_7: char = '╯';

const BRAILLE_1_0: char = '⡀';
const BRAILLE_1_1: char = '⣀';
const BRAILLE_1_2: char = '⣀';
const BRAILLE_2_0: char = '⡄';
const BRAILLE_3_0: char = '⡆';
const BRAILLE_4_0: char = '⡇';


/*
  ╭────────╮
╭─╯        │
╰          ╰╮
            ╰────────
*/

#[derive(Debug)]
pub struct GraphOptions {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct SeriesAspects<T> {
    max: T,
    min: T,
}

pub trait SeriesTraits: std::cmp::PartialOrd + Clone + std::ops::Div + std::ops::Sub {}
impl<T: std::cmp::PartialOrd + Clone + std::ops::Div + std::ops::Sub> SeriesTraits for T {}

impl<T: SeriesTraits> From<&Vec<T>> for SeriesAspects<T> {
    fn from(series: &Vec<T>) -> SeriesAspects<T> {
        let mut it = series.iter();
        let first = it.next();
        let mut min = first.expect("TG2");
        let mut max = first.expect("TG3");
        while let Some(i) = it.next() {
            if i < min {
                min = i;
            }
            if i > max {
                max = i;
            }
        }
        SeriesAspects {
            max: max.clone(),
            min: min.clone(),
        }
    }
}


pub fn downsample(series: &[f64], column_count: usize) -> Vec<f64> {
    let factor = series.len() as f64 / column_count as f64;
    (0..column_count).map(|i| series[(i as f64 * factor) as usize]).collect()
}

pub fn interpolate(series: &[f64], marks: &[f64], column_count: usize) -> Vec<f64> {
    let min_mark = marks.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_mark = marks.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let step = (max_mark - min_mark) / (column_count as f64 - 1.0);
    let mut interpolated_data = Vec::new();
    
    for i in 0..column_count {
        let target_mark = min_mark + i as f64 * step;
        let mut j = 0;
        while j < marks.len() - 1 && marks[j + 1] < target_mark {
            j += 1;
        }
        let t0 = marks[j];
        let t1 = marks[j + 1];
        let d0 = series[j];
        let d1 = series[j + 1];
        let value = d0 + (d1 - d0) * (target_mark - t0) / (t1 - t0);
        interpolated_data.push(value);
    }
    
    interpolated_data
}

fn scale(series: &[f64], row_count: usize) -> Vec<usize> {
    let min_value = series.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_value = series.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let scale_factor = (row_count - 1) as f64 / (max_value - min_value);
    series.iter().map(|&y| ((y - min_value) * scale_factor).round() as usize).collect()
}

pub fn star(series: &[f64], options: &GraphOptions) -> String {
    let scaled_data = scale(series, options.height as usize);
    let mut graph = vec![vec![' '; options.width as usize]; options.height as usize];
    
    for (i, &value) in scaled_data.iter().enumerate() {
        let y = options.height as usize - value - 1; // Invert y-axis for ASCII graph
        graph[y][i] = '*';
    }
    
    graph.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().join("\n")
}

pub fn ascii_trailing(series: &[f64], options: &GraphOptions) -> String {
    let scaled_data = scale(series, options.height as usize);
    let mut graph = vec![vec![' '; options.width as usize]; options.height as usize];
    
    for (i, &value) in scaled_data.iter().enumerate() {
        let y = options.height as usize - value - 1; // Invert y-axis for ASCII graph
        graph[y][i] = '*';
    }
    
    graph.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().join("\n")
}

pub fn braille(series: &Vec<f64>, options: &GraphOptions) -> String {
    let aspects = SeriesAspects::from(series);
    let canvas = String::with_capacity((options.width * options.height) as usize);


    /*
    r = (max - min)  
    r' = (max' - min')  
    y' = (((y - min) * r') / r) + min'
    */
    let r = aspects.max - aspects.min;
    let r_marked = options.height;

    let norm_after = options.height;

    //for (x, y) in series.iter().enumerate() {
    //    let y = norm(y.clone(), 0.0, options.height);
    //    let x = norm(x.clone(), 0.0, options.width);
    //}

    String::from("")
}
