pub mod alloc;
pub mod sample;
pub mod report;

use argh::FromArgs;

#[global_allocator]
pub static mut ALLOCATOR: alloc::Tracing = alloc::Tracing::new();

#[derive(FromArgs)]
#[argh(description = "My arguments")]
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Sample(sample::Sample),
    Report(report::Report),
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
    argh::from_env::<Args>().subcommand.run();
}
