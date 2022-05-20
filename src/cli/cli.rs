use clap::Parser;

// work in progress..

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct RunnerArgs {
    #[clap(short, long)]
    name: String,
}

pub fn run() {
    let args = RunnerArgs::parse();
    println!("Hello {}!", args.name);
}
