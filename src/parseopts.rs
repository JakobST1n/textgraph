use crate::graph::{BrailleType, GraphType};
use std::str::FromStr;

use std::io::IsTerminal;

/// Struct containing command line options
pub struct Opts {
    /// Desired width of graph, if None, it should be automatically determined
    pub width: usize,
    /// Desired height of graph, if None, it should be automatically determined
    pub height: usize,
    /// Which type of graph it should be, ascii, star
    pub graph_type: GraphType,
    /// This will disable distracting elements, such as axis
    pub silent: bool,
    /// Specify if it is used as a filter, and you only want to look at the last N samples
    pub last_n: Option<u64>,
    /// Special case of last_n, which will use window_with as a target.
    pub cut: bool,
    /// Read from the specified file, instead of reading continously from stdin
    pub in_file: Option<String>,
    /// Enable color
    pub color: bool,
}

/// Struct containing command line options
#[derive(Clone)]
pub struct OptsBuilder {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub graph_type: GraphType,
    pub silent: bool,
    pub last_n: Option<u64>,
    pub cut: bool,
    pub in_file: Option<String>,
    pub color: Option<bool>,
}

impl OptsBuilder {
    pub fn build(self) -> Opts {
        Opts {
            width: self.width.unwrap_or_else(|| {
                #[cfg(feature = "libc")]
                if let Ok((width, _)) = crate::term::get_terminal_size() {
                    width as usize
                } else {
                    println!("Could not determine TTY columns, specify with -w");
                    std::process::exit(1);
                }
                #[cfg(not(feature = "libc"))]
                {
                    println!("Not compiled with libc, cannot detect columns, specify with -w");
                    std::process::exit(1);
                }
            }),
            height: self.height.unwrap_or_else(|| {
                #[cfg(feature = "libc")]
                if let Ok((_, height)) = crate::term::get_terminal_size() {
                    height as usize - 1
                } else {
                    println!("Could not determine TTY rows, specify with -h");
                    std::process::exit(1);
                }
                #[cfg(not(feature = "libc"))]
                {
                    println!("Not compiled with libc, cannot detect rows, specify with -h");
                    std::process::exit(1);
                }
            }),
            graph_type: self.graph_type,
            silent: self.silent,
            last_n: self.last_n,
            in_file: self.in_file,
            cut: self.cut,
            color: self
                .color
                .unwrap_or_else(|| std::io::stdout().is_terminal()),
        }
    }
}

/// Simple convenience macro for printing usage of the program and exiting without a stacktrace.
/// For some reason, having this as a function didn't always make the compiler recognize that
/// the program exited.
macro_rules! parseopts_panic {
    ($progname:expr) => {
        println!("Usage: {} [OPTIONS] [INPUTFILE]\n", $progname);
        println!("INPUTFILE:");
        println!("  If provided, the data will be read from the file.");
        println!("  If not provided, data will be read from STDIN.\n");
        println!("OPTIONS:");
        println!("  -h, --help          Show this help text");
        println!("                      See also 'man tg'");
        println!("  -s, --silent        Disable distracting elements, such as axis");
        println!("  -b, --braille       Shorthand for -t braille");
        println!("  -a, --ascii         Shorthand for -t ascii");
        println!(
            "  -t           TYPE   Set graph type, valid options are 'star', 'ascii', 'braille', 'braille6' or 'braille8'"
        );
        println!("  -n, --last-n N      Only include N samples in plot");
        println!("  -c, --cut           Special case of --last-n, where N is set to --width");
        println!("  -w, --width  WIDTH  Set desired width of graph");
        println!("  -h, --height HEIGHT Set desired height of graph");
        println!("      --color  ENABLE Enable or disable color, valid options are 'yes' or 'no'");
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
pub fn parseopt(opts: &mut OptsBuilder, arg: &str, value: Option<String>, progname: &str) {
    match arg {
        "t" => {
            let Some(graph_type) = value else {
                println!("Missing value for {}\n", arg);
                parseopts_panic!(progname);
            };
            match graph_type.as_str() {
                "star" => {
                    opts.graph_type = GraphType::Star;
                }
                "ascii" => {
                    opts.graph_type = GraphType::Ascii;
                }
                "braille" | "braille6" => {
                    opts.graph_type = GraphType::Braille(BrailleType::Dot6);
                }
                "braille8" => {
                    opts.graph_type = GraphType::Braille(BrailleType::Dot8);
                }
                t => {
                    println!(
                        "Unknown type \"{}\", valid options are \"star\", \"ascii\" and \"braille\".\n",
                        t
                    );
                    parseopts_panic!(progname);
                }
            }
        }
        "h" | "height" => {
            let Some(height) = value else {
                println!("Missing value for {}\n", arg);
                parseopts_panic!(progname);
            };
            let Ok(height) = usize::from_str(&height) else {
                println!("Cannot parse integer from \"{}\"\n", height);
                parseopts_panic!(progname);
            };
            opts.height = Some(height);
        }
        "n" | "last-n" => {
            let Some(last_n) = value else {
                println!("Missing value for {}\n", arg);
                parseopts_panic!(progname);
            };
            let Ok(last_n) = u64::from_str(&last_n) else {
                println!("Cannot parse integer from \"{}\"\n", last_n);
                parseopts_panic!(progname);
            };
            opts.last_n = Some(last_n);
        }
        "s" | "silent" => {
            opts.silent = true;
        }
        "a" | "ascii" => {
            opts.graph_type = GraphType::Ascii;
        }
        "b" | "braille" => {
            opts.graph_type = GraphType::Braille(BrailleType::Dot6);
        }
        "c" | "cut" => {
            opts.cut = true;
        }
        "color" => {
            let Some(color) = value else {
                println!("Missing value for {}\n", arg);
                parseopts_panic!(progname);
            };
            opts.color = match color.as_str() {
                "yes" => Some(true),
                "no" => Some(false),
                t => {
                    println!(
                        "Unknown type \"{}\", valid options are \"yes\", \"no\".\n",
                        t
                    );
                    parseopts_panic!(progname);
                }
            }
        }
        "w" | "width" => {
            let Some(width) = value else {
                println!("Missing value for {}\n", arg);
                parseopts_panic!(progname);
            };
            let Ok(width) = usize::from_str(&width) else {
                println!("Cannot parse integer from \"{}\"\n", width);
                parseopts_panic!(progname);
            };
            opts.width = Some(width);
        }
        "help" => {
            parseopts_panic!(progname);
        }
        opt => {
            println!("Unknown option \"{}\"\n", opt);
            parseopts_panic!(progname);
        }
    }
}

/// Parse command line options passed to binary
/// Very rudimentary argument parser, which allows for the most part the standard convention
/// of unix style command line arguments.
/// This function is specialised for the TextGraph program,
/// but is easily adaptable for other programs as well.
pub fn parseopts() -> OptsBuilder {
    let mut opts = OptsBuilder {
        width: None,
        height: None,
        graph_type: GraphType::Star,
        silent: false,
        last_n: None,
        cut: false,
        in_file: None,
        color: None,
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
                    "width" | "height" | "last-n" | "color" => {
                        arg_value = it.next();
                    }
                    _ => (),
                }
            }
            parseopt(&mut opts, &arg_name, arg_value, &progname);
        } else if arg.starts_with("-") {
            arg.remove(0);
            for arg_name in arg.chars() {
                match arg_name {
                    'h' | 't' | 'w' | 'n' => {
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
                }
                _ => {
                    println!(
                        "No positional argument expected at position {} (\"{}\")\n",
                        pos_arg, arg
                    );
                    parseopts_panic!(progname);
                }
            }
            pos_arg += 1;
        }
    }

    return opts;
}
