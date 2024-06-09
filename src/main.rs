use textgraph::graph;
use textgraph::parseopts::{parseopts, Opts};
use std::io::{self, BufRead, Write};
use std::str::FromStr;

/// Will graph what comes in through stdin,
/// For each new line, the graph will be re-drawn.
///
/// # Arguments
///
/// * `opts` -  textgraph::parseopts::Opts
fn filter(opts: Opts) {
    print!("\x1b[?1049h");

    let mut x_values: Vec<f64> = Vec::new();
    let mut y_values: Vec<f64> = Vec::new();
    let mut i = 0.0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        i += 1.0;
        let line = line.expect("Could not read...");

        let y = f64::from_str(line.as_str()).expect("TG7 invalid number");
        y_values.push(y);
        x_values.push(i);

        let graph_options: textgraph::graph::GraphOptions = (&opts).into();
        let g = match opts.graph_type {
            textgraph::parseopts::GraphType::Ascii => {
                graph::ascii(&y_values, &x_values, &graph_options)
            }
            textgraph::parseopts::GraphType::Star => graph::star(&y_values, &x_values, &graph_options),
        };
        print!("\x1B[2J\x1B[H");
        println!("{}", g);
    }

    print!("\x1B[?1049l");
    io::stdout().flush().unwrap();
}

/// Will graph the contents of a file
/// This assumes opts.in_file is Some, or it will panic!
/// Currently this only supports a single column, with no x-values
///
/// # Arguments
///
/// * `opts` -  textgraph::parseopts::Opts
fn graph_file(opts: Opts) {
    let raw_y_values = std::fs::read_to_string(opts.in_file.clone().unwrap()).expect("TG6");

    let mut y_values: Vec<f64> = Vec::new();
    let mut x_values: Vec<f64> = Vec::new();
    for (i, line) in raw_y_values.lines().enumerate() {
        y_values.push(f64::from_str(line).expect("TG7"));
        x_values.push(i as f64);
    }

    let graph_options: textgraph::graph::GraphOptions = (&opts).into();
    let g = match opts.graph_type {
        textgraph::parseopts::GraphType::Ascii => {
            graph::ascii(&y_values, &x_values, &graph_options)
        }
        textgraph::parseopts::GraphType::Star => graph::star(&y_values, &x_values, &graph_options),
    };
    println!("{}", g);
}

/// Main entry point for the binary of textgraph
fn main() {
    let opts = parseopts();

    if opts.in_file.is_none() {
        filter(opts);
    } else {
        graph_file(opts);
    }
}
