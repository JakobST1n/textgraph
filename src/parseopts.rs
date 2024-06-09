use crate::graph::GraphOptions;
use std::str::FromStr;

/// Available options for how the graph should look
pub enum GraphType {
    /// Use only * symbols 
    Star,
    /// Use pretty characters from the ascii range
    Ascii,
}

/// Struct containing command line options
pub struct Opts {
    /// Desired width of graph, if None, it should be automatically determined
    pub width: Option<u64>,
    /// Desired height of graph, if None, it should be automatically determined
    pub height: Option<u64>,
    /// Which type of graph it should be, ascii, star
    pub graph_type: GraphType,
    /// Wether to always interpolate, even if not nesecarry
    pub interpolate: bool,
    /// Enable axis on the resulting graph, makes it a bit prettier
    pub axis: bool,
    /// Specify if it is used as a filter, and you only want to look at the last N samples
    pub last_n: Option<u64>,
    /// Read from the specified file, instead of reading continously from stdin 
    pub in_file: Option<String>,
}

impl From<&Opts> for GraphOptions {
    /// Convert from CLIOpts to GraphOptions,
    /// This will do some magic, like find the terminal size if not specified, etc.
    fn from(opts: &Opts) -> Self {
        GraphOptions {
            width: opts.width.unwrap_or_else(|| {
                if let Ok((width, _)) = crate::term::get_terminal_size() {
                    width as u64
                } else {
                    println!("Could not determine TTY columns, specify with -r");
                    std::process::exit(1);
                }
            }),
            height: opts.height.unwrap_or_else(|| {
                if let Ok((_, height)) = crate::term::get_terminal_size() {
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

/// Simple convenience macro for printing usage of the program and exiting without a stacktrace.
/// For some reason, having this as a function didn't always make the compiler recognize that
/// the program exited.
macro_rules! parseopts_panic {
    ($progname:expr) => {
        println!(
            "Usage: {} [-h|--height <height>] [-w|--width <width>] [-t <star|ascii>]",
            $progname
        );
        std::process::exit(1);
    };
}

/// Parse a single named option/argument, and update the Opts struct accordingly
///
/// # Arguments
///
/// * `opts` - The opts struct to modify
/// * `arg` - The name of the option/argument to read (without the -)
/// * `value` - Optionally the value of the option/argument. This function will panic if not
///             provided when it is required.
/// * `progname` - The first argument of the program, this is used for error messages.
pub fn parseopt(opts: &mut Opts, arg: &str, value: Option<String>, progname: &str) {
    match arg {
        "interpolate" => {
            opts.interpolate = true;
        }
        "t" => {
            let Some(graph_type) = value else {
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
                        "Unknown type \"{}\", valid options are \"star\", \"ascii\".",
                        t
                    );
                    parseopts_panic!(progname);
                }
            }
        }
        "h" | "height" => {
            let Some(height) = value else {
                println!("Missing value for {}", arg);
                parseopts_panic!(progname);
            };
            let Ok(height) = u64::from_str(&height) else {
                println!("Cannot parse integer from \"{}\"", height);
                parseopts_panic!(progname);
            };
            opts.height = Some(height);
        }
        "l" | "last-n" => {
            let Some(last_n) = value else {
                println!("Missing value for {}", arg);
                parseopts_panic!(progname);
            };
            let Ok(last_n) = u64::from_str(&last_n) else {
                println!("Cannot parse integer from \"{}\"", last_n);
                parseopts_panic!(progname);
            };
            opts.last_n = Some(last_n);
        }
        "a" | "axis" => {
            opts.axis = true;
        }
        "w" | "width" => {
            let Some(width) = value else {
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

/// Parse command line options passed to binary
/// Very rudimentary argument parser, which allows for the most part the standard convention
/// of unix style command line arguments.
/// This function is specialised for the TextGraph program,
/// but is easily adaptable for other programs as well.
pub fn parseopts() -> Opts {
    let mut opts = Opts {
        width: None,
        height: None,
        graph_type: GraphType::Star,
        interpolate: false,
        axis: false,
        last_n: None,
        in_file: None,
    };

    let mut it = std::env::args();
    let progname = it.next().expect("TG1");

    let mut pos_arg = 0;

    while let Some(mut arg) = it.next() {
        if arg.starts_with("--") {
            arg.remove(0);
            arg.remove(0);

            let arg_name;
            let mut arg_value = None;
            if arg.contains('=') {
                let mut ita = arg.splitn(2, '=');
                arg_name = ita.next().expect("TG4").to_string();
                arg_value = Some(ita.next().expect("TG5").to_string());
            } else {
                arg_name = arg.clone();
                match arg_name.as_str() {
                    "widht" | "height" | "last-n" => {
                        arg_value = it.next();
                    }
                    _ => ()
                }
            }
            parseopt(&mut opts, &arg_name, arg_value, &progname);
        } else if arg.starts_with("-") {
            arg.remove(0);
            for arg_name in arg.chars() {
                match arg_name {
                    'h' | 't' | 'w' | 'l' => {
                        parseopt(&mut opts, &arg_name.to_string(), it.next(), &progname);
                    }
                    _ => {
                        parseopt(&mut opts, &arg_name.to_string(), None, &progname);
                    }
                }
            }
        } else {
            match pos_arg {
                0 => {
                    opts.in_file = Some(arg);
                },
                _ => {
                    println!("No positional argument expected at position {} (\"{}\")", pos_arg, arg);
                    parseopts_panic!(progname);
                }
            }
            pos_arg += 1;
        }

    }


    return opts;
}
