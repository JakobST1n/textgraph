use std::io::{self, BufRead, Write};
use std::str::FromStr;
use textgraph::graph::GraphBuilder;
use textgraph::parseopts::{parseopts, OptsBuilder};

extern "C" fn handle_sigint(_sig: i32) {
    print!("\x1b[?25h");
    print!("\x1B[?1049l");
    io::stdout().flush().unwrap();
    std::process::exit(0);
}

/// Set a signalhandler for swapping back to the the main screen on sigint
fn set_filter_signalhandler() {
    let mut sig_action: libc::sigaction = unsafe { std::mem::zeroed() };
    sig_action.sa_flags = 0;
    sig_action.sa_sigaction = handle_sigint as usize;

    unsafe {
        let mut signal_set = std::mem::zeroed();
        libc::sigemptyset(&mut signal_set);
        sig_action.sa_mask = signal_set;

        libc::sigaction(libc::SIGINT, &sig_action, std::ptr::null_mut());
        libc::sigaction(libc::SIGKILL, &sig_action, std::ptr::null_mut());
        libc::sigaction(libc::SIGTSTP, &sig_action, std::ptr::null_mut());
        libc::sigaction(libc::SIGSTOP, &sig_action, std::ptr::null_mut());
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
    set_filter_signalhandler();
    print!("\x1b[?1049h");
    print!("\x1b[?25l");

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
