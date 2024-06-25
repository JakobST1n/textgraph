use std::io::{self, BufRead};
use std::str::FromStr;
use textgraph::graph::GraphBuilder;
use textgraph::parseopts::{parseopts, OptsBuilder};

#[cfg(all(feature = "libc", feature = "ansi"))]
use std::io::Write;

#[cfg(all(feature = "libc", feature = "ansi"))]
extern "C" fn handle_sigint(_sig: std::os::raw::c_int) {
    print!("\x1b[?25h");
    print!("\x1B[?1049l");
    io::stdout().flush().unwrap();
    std::process::exit(0);
}

/// Set a signalhandler for swapping back to the the main screen on sigint
#[cfg(all(feature = "libc", feature = "ansi"))]
fn set_filter_signalhandler() {
    unsafe {
        let mut action: textgraph::term::SigAction = std::mem::zeroed();
        action.sa_flags = 0;
        action.sa_sigaction = handle_sigint as usize;

        textgraph::term::sigemptyset(&mut action.sa_mask);
        textgraph::term::sigaction(15, &action, std::ptr::null_mut()); // 15 is SIGTERM
        textgraph::term::sigaction(2, &action, std::ptr::null_mut()); //  2 is SIGINT
        textgraph::term::sigaction(9, &action, std::ptr::null_mut()); //  9 is SIGKILL
        textgraph::term::sigaction(20, &action, std::ptr::null_mut()); // 20 is SIGSTP
        textgraph::term::sigaction(19, &action, std::ptr::null_mut()); // 19 is SIGSTOP
    }
}

/// Build a graph text string, based on values and a OptsBuilder
///
/// # Arguments
///
/// * `opts` -  textgraph::parseopts::OptBuilder
fn build_graph(x_values: &Vec<f64>, y_values: &Vec<f64>, opts: &OptsBuilder) -> String {
    let opts = opts.clone().build();

    let mut gb = GraphBuilder::new(&x_values, &y_values, opts.width, opts.height);
    gb.color(opts.color);
    gb.axis(!opts.silent);
    gb.graph_type(opts.graph_type.clone());
    if opts.cut {
        gb.cut_overflow(true);
    } else if let Some(n) = opts.last_n {
        gb.keep_tail(n as usize);
    }

    gb.build()
}

/// Will graph what comes in through stdin,
/// For each new line, the graph will be re-drawn.
///
/// # Arguments
///
/// * `opts` -  textgraph::parseopts::OptBuilder
fn filter(opts: OptsBuilder) {
    #[cfg(all(feature = "libc", feature = "ansi"))]
    {
        set_filter_signalhandler();
        print!("\x1b[?1049h");
        print!("\x1b[?25l");
    }

    let mut x_values: Vec<f64> = Vec::new();
    let mut y_values: Vec<f64> = Vec::new();
    let mut i = 0.0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        i += 1.0;
        let line = line.expect("Could not read...");

        let y = f64::from_str(line.as_str());
        if let Err(_) = y {
            print!("Could not parse line as f64.");
            continue;
        }
        if let Ok(y) = y {
            y_values.push(y);
            x_values.push(i);
        }

        #[cfg(feature = "ansi")]
        print!("\x1B[2J\x1B[H");
        println!("{}", build_graph(&x_values, &y_values, &opts));
    }
}

/// Will graph the contents of a file
/// This assumes opts.in_file is Some, or it will panic!
/// Currently this only supports a single column, with no x-values
///
/// # Arguments
///
/// * `opts` -  textgraph::parseopts::OptBuilder
fn graph_file(opts: OptsBuilder) {
    let raw_y_values = std::fs::read_to_string(opts.in_file.clone().unwrap()).expect("TG6");

    let mut y_values: Vec<f64> = Vec::new();
    let mut x_values: Vec<f64> = Vec::new();
    for (i, line) in raw_y_values.lines().enumerate() {
        y_values.push(f64::from_str(line).expect("TG7"));
        x_values.push(i as f64);
    }

    println!("{}", build_graph(&x_values, &y_values, &opts));
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
