#[derive(Debug, Clone)]
pub enum Args {
    Version,
    Help,
    Perform {
        width: u32,
        height: u32,
        directory: String,
        output: String,
    },
}

fn error_message(message: &str) -> String {
    format!(
        "{}\nUse `sprite-binpacker help` for more information.",
        message
    )
}

impl Args {
    pub const HELP: &'static str = "USAGE: sprite-binpacker <WIDTH> <HEIGHT> <DIRECTORY> <OUTPUT>
       sprite-binpacker help
       sprite-binpacker version

COMMANDS:
    help      Display this help message
    version   Display the version of the program

ARGUMENTS:
    width     The width of the texture atlas in pixels
    height    The height of the texture atlas in pixels
    directory The directory containing the sprite images
    output    The directory to output the sprite sheet and metadata";
    pub const VERSION: &'static str =
        concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));
}

impl TryFrom<Vec<String>> for Args {
    type Error = String;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
        if args.len() == 1 {
            return Err(error_message("no arguments provided"));
        }

        if args[1] == "version" {
            Ok(Args::Version)
        } else if args[1] == "help" {
            Ok(Args::Help)
        } else {
            if args.len() != 5 {
                return Err(error_message("incorrect number of arguments"));
            }

            let width = args[1]
                .parse::<u32>()
                .map_err(|_| error_message("invalid width"))?;
            let height = args[2]
                .parse::<u32>()
                .map_err(|_| error_message("invalid height"))?;
            let directory = args[3].to_string();
            let output = args[4].to_string();

            Ok(Args::Perform {
                width,
                height,
                directory,
                output,
            })
        }
    }
}
