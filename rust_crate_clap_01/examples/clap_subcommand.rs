use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(name = "clap-subcommand")]
#[command(about = "Clap subcommand example", long_about = None)]
pub struct Cli {
    #[arg(short = 'm', long = "message", help = "Message to print")]
    message: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, PartialEq, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Print message to stdout")]
    Stdout,
    #[command(about = "Print message to stderr")]
    Stderr(StderrArgs),
}

#[derive(Debug, Clone, PartialEq, Args)]
pub struct StderrArgs {
    #[arg(short = 'p', long = "prefix", help = "Message prefix")]
    prefix: String,
    #[arg(
        short = 's',
        long = "suffix",
        help = "Message suffix",
        default_value = "2"
    )]
    suffix: Option<u64>,
}

fn main() {
    let cli = Cli::parse();
    // println!("cli: {:?}", cli);

    match cli.command {
        Commands::Stdout => {
            println!("{}", cli.message);
        }
        Commands::Stderr(args_) => {
            let mut final_message = args_.prefix + &" " + &cli.message;
            if let Some(suffix) = args_.suffix {
                final_message += &" ";
                final_message += &suffix.to_string();
            }
            eprintln!("{}", final_message);
        }
    }
}
