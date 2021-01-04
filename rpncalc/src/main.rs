use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "My program",
    version = "1.0.0",
    author = "horino",
    about = "awesome project"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    println!("verbose status: {}", opts.verbose);
}
