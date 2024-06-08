use crate::graph::GraphOptions;
use std::str::FromStr;

pub enum GraphType {
    Star,
    Ascii,
}

pub struct Opts {
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub graph_type: GraphType,
    pub interpolate: bool,
    pub axis: bool,
    pub last_n: Option<u64>,
}

impl From<&Opts> for GraphOptions {
    fn from(opts: &Opts) -> Self {
        GraphOptions {
            width: opts.width.unwrap_or_else(|| {
                if let Ok((width, _)) = crate::term::get_terminal_size() {
                    // Here it would maybe be a good idea to keep the size of the graph if it is smaller than
                    // the specified value
                    width as u64
                } else {
                    println!("Could not determine TTY columns, specify with -r");
                    std::process::exit(1);
                }
            }),
            height: opts.height.unwrap_or_else(|| {
                if let Ok((_, height)) = crate::term::get_terminal_size() {
                    // Here it would maybe be a good idea to keep the size of the graph if it is smaller than
                    // the specified value
                    height as u64 - 1
                } else {
                    println!("Could not determine TTY rows, specify with -h");
                    std::process::exit(1);
                }
            }),
            interpolate: opts.interpolate,
            axis: opts.axis,
        }
    }
}

macro_rules! parseopts_panic {
    ($progname:expr) => {
        println!(
            "Usage: {} [-h|--height <height>] [-w|--width <width>] [-t <star|ascii>]",
            $progname
        );
        std::process::exit(1);
    };
}

pub fn parseopts() -> Opts {
    let mut opts = Opts {
        width: None,
        height: None,
        graph_type: GraphType::Star,
        interpolate: false,
        axis: false,
        last_n: None,
    };

    let mut it = std::env::args();
    let progname = it.next().expect("TG1");

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--interpolate" => {
                opts.interpolate = true;
            }
            "-t" => {
                let Some(graph_type) = it.next() else {
                    println!("Missing value for {}", arg);
                    parseopts_panic!(progname);
                };
                match graph_type.as_str() {
                    "star" => {
                        opts.graph_type = GraphType::Star;
                    }
                    "ascii" => {
                        opts.graph_type = GraphType::Ascii;
                    }
                    t => {
                        println!(
                            "Unknown type \"{}\", valid options are \"star\", \"ascii_trailing\".",
                            t
                        );
                        parseopts_panic!(progname);
                    }
                }
            }
            "-h" | "--height" => {
                let Some(height) = it.next() else {
                    println!("Missing value for {}", arg);
                    parseopts_panic!(progname);
                };
                let Ok(height) = u64::from_str(&height) else {
                    println!("Cannot parse integer from \"{}\"", height);
                    parseopts_panic!(progname);
                };
                opts.height = Some(height);
            }
            "-l" | "--last-n" => {
                let Some(last_n) = it.next() else {
                    println!("Missing value for {}", arg);
                    parseopts_panic!(progname);
                };
                let Ok(last_n) = u64::from_str(&last_n) else {
                    println!("Cannot parse integer from \"{}\"", last_n);
                    parseopts_panic!(progname);
                };
                opts.last_n = Some(last_n);
            }
            "-a" | "--axis" => {
                opts.axis = true;
            }
            "-w" | "--width" => {
                let Some(width) = it.next() else {
                    println!("Missing value for {}", arg);
                    parseopts_panic!(progname);
                };
                let Ok(width) = u64::from_str(&width) else {
                    println!("Cannot parse integer from \"{}\"", width);
                    parseopts_panic!(progname);
                };
                opts.width = Some(width);
            }
            opt => {
                println!("Unknown option \"{}\"", opt);
                parseopts_panic!(progname);
            }
        }
    }

    return opts;
}
