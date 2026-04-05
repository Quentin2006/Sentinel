use clap::Parser;
use std::process::{Command, Output};

#[derive(Parser)]
struct Cli {
    // NOTE: this is how you add a flag
    // the flags value will auto bind
    #[arg(short, long, default_value = "opencode/big-pickle")]
    model: String,

    // this will simply hold the the rest of the cmd, i.e. the compilation command
    compiler: String,
    compilation_args: Vec<String>,
}

fn fix(args: Cli, _command_res: Output) {
    let output = Command::new("opencode")
        .args(["-m", &args.model])
        .args(["run", "this is a test, just say hello!"])
        .output()
        .expect("ERROR: Failed ask opencode");

    print!("{:#?}", output)
}

fn main() {
    // exstract args
    let args = Cli::parse();

    // ensure we have a compilation command
    if args.compiler.is_empty() {
        panic!("ERROR: Cannot run without compilation cmd")
    }

    // run the compilation command
    let output = Command::new(args.compiler.clone())
        .args(args.compilation_args.clone())
        .output()
        .expect("ERROR: Failed execute compilation commmand");

    println!("{:?}", output);

    // check if we have any errors from this cmd
    // if we do, lets just return as theres is nothing to fix
    // if output.stderr.is_empty() {
    //     return;
    // }

    fix(args, output);
}
