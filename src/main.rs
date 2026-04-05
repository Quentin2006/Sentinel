use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    // NOTE: this is how you add a flag
    // the flags value will auto bind
    #[arg(
        short = 'f',
        long = "flag",
        default_value = "this is the flag default value"
    )]
    flag: String,

    // this will simply hold the the rest of the cmd, i.e. the compilation command
    compiler: String,
    compilation_args: Vec<String>,
}

fn main() {
    // exstract args
    let args = Cli::parse();

    // ensure we have a compilation command
    if args.compiler.is_empty() {
        panic!("ERROR: Cannot run without compilation cmd")
    }

    println!(
        "{:?} {:?} {:?}",
        args.compiler, args.compilation_args, args.flag
    );

    // run the compilation command
    let output = Command::new(args.compiler.clone())
        .args(args.compilation_args.clone())
        .output()
        .expect("failed to execute process");

    println!("{:#?}", output)
}
