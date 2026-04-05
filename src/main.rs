use clap::Parser;
use std::{
    fs,
    process::{Command, Output, exit},
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "opencode/big-pickle")]
    model: String,

    compiler: String,
    compilation_args: Vec<String>,
}

fn get_skill(skill_name: &str) -> Option<String> {
    let skill_path = format!("{}/{}", "src/skills", skill_name);
    fs::read_to_string(skill_path).ok()
}

fn fix(args: Cli, command_res: Output) {
    let skill_str = match get_skill("debugging.md") {
        Some(s) => s,
        None => {
            eprintln!("sentinel: cannot locate skill");
            exit(1);
        }
    };

    let output = Command::new("opencode")
        .args(["-m", &args.model])
        .arg("--pure")
        .args([
            "run",
            format!("{}: {}", "The following is your skill", skill_str.as_str()).as_str(),
        ])
        .arg(
            format!(
                "{}: {:#?}",
                "the following is the compilation results", command_res
            )
            .as_str(),
        )
        .arg(format!(
            "Ensure you use the following compilation command: {} {}",
            args.compiler,
            args.compilation_args.join(" ")
        ))
        .arg("Fix the errors, and provide a summary of what you did, dont stop till its fixed. ")
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                eprintln!("sentinel: fix applied successfully");
            } else {
                eprintln!("sentinel: fix attempt failed");
                exit(1);
            }

            let Ok(stdout) = std::str::from_utf8(&o.stdout) else {
                return;
            };
            eprint!("{}", stdout);
        }
        Err(_) => {
            eprintln!("sentinel: failed to invoke opencode");
            exit(1);
        }
    }
}

fn main() {
    let args = Cli::parse();

    if args.compiler.is_empty() {
        eprintln!("sentinel: no compiler specified");
        exit(1);
    }

    let output = match Command::new(args.compiler.clone())
        .args(args.compilation_args.clone())
        .output()
    {
        Ok(o) => o,
        Err(_) => {
            eprintln!("sentinel: failed to execute compiler");
            exit(1);
        }
    };

    if output.stderr.is_empty() {
        eprintln!("sentinel: compiled successfully");
        return;
    }

    eprintln!("sentinel: compilation failed, attempting fix...\n");

    if let Ok(stderr) = std::str::from_utf8(&output.stderr) {
        eprint!("{}", stderr);
    }

    fix(args, output);
}
