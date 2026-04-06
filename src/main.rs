use clap::{Parser, ValueEnum};
use std::{
    fs,
    process::{exit, Command, Output},
};

#[derive(Clone, ValueEnum, Debug)]
enum Level {
    Errors,
    Warning,
    Logic,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "opencode/big-pickle")]
    model: String,

    #[arg(short, long, value_enum, default_value_t= Level::Errors)]
    level: Level,

    compiler: String,
    compilation_args: Vec<String>,
}

fn get_skill(skill_name: &str) -> Option<String> {
    let skill_path = format!(
        "{}/{}",
        "/home/qscheetz/Documents/Sentinel/src/skills", skill_name
    );
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
        .arg(format!(
            "Fix level: {:?}. At 'Errors' level fix ONLY compilation errors (ignore warnings). At 'Warning' level fix errors AND warnings. At 'Logic' level fix errors, warnings, AND logic issues.",
            args.level
        ))
        .arg("Fix the issues according to the level, and provide a summary of what you did, dont stop till its fixed.")
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
