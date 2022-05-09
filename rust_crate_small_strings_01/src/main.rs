pub mod sample;
pub mod alloc;
pub mod report;

// As new is 'const fn' we can call this for a static
#[global_allocator]
pub static ALLOCATOR: alloc::Tracing = alloc::Tracing::new();

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "Small string demo")]  // can be a comment starting with ///
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}


#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Sample(sample::Sample),
    Report(report::Report)
}

impl Subcommand {
    fn run(self) {
        match self {
            Subcommand::Sample(x) => x.run(),
            Subcommand::Report(x) => x.run(),
        }
    }
}


fn main() {
    println!("Hello small strings!");
    // argh::from_env::<Args>().subcommand.run();

    let args: Args = argh::from_env();
    let subcommand = args.subcommand;
    subcommand.run();
}