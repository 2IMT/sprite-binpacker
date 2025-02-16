use sprite_binpacker::Args;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let args = match Args::try_from(args) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Argument error: {}", err);
            std::process::exit(1);
        }
    };

    match args {
        Args::Version => println!("{}", Args::VERSION),
        Args::Help => println!("{}", Args::HELP),
        Args::Perform {
            width,
            height,
            directory,
            output,
        } => {
            println!("Texture atlas dimensions: {width}x{height}");
            println!("Input directory: {}", directory);
            println!("Output directory: {}", output);
        }
    }
}
