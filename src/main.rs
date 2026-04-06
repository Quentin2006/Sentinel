use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::{
    fs,
    process::{Command, Output, exit},
};
mod msg;

#[derive(Clone, ValueEnum, Debug)]
enum Level {
    Error,
    Warning,
    Logic,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "opencode/big-pickle")]
    model: String,

    #[arg(short, long, value_enum, default_value_t= Level::Error)]
    level: Level,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

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

fn render_marked_output(text: &str) {
    for mut line in text.lines() {
        let mut line_level: Option<Level> = None;

        // detect error and remove prefix
        if line.starts_with("ERROR_MESSAGE|") {
            line_level = Some(Level::Error);
            line = line.strip_prefix("ERROR_MESSAGE|").unwrap();
        } else if line.starts_with("WARNING_MESSAGE|") {
            line_level = Some(Level::Warning);
            line = line.strip_prefix("WARNING_MESSAGE|").unwrap();
        } else if line.starts_with("LOGIC_MESSAGE|") {
            line_level = Some(Level::Logic);
            line = line.strip_prefix("LOGIC_MESSAGE|").unwrap();
        }

        match line_level {
            Some(Level::Error) => eprintln!("{}", line.red()),
            Some(Level::Warning) => eprintln!("{}", line.yellow()),
            Some(Level::Logic) => eprintln!("{}", line.green()),
            None => eprintln!("{}", line),
        }
    }
}

fn default_opencode_prompt(args: &Cli, command_res: &Output) -> Vec<String> {
    let skill_str = match get_skill("debugging.md") {
        Some(s) => s,
        None => {
            msg::error(
                "S001",
                "cannot locate skill",
                Some(
                    "ensure skills directory exists at /home/qscheetz/Documents/Sentinel/src/skills",
                ),
            );
            exit(1);
        }
    };

    vec![
        "-m".to_string(),
        args.model.clone(),
        "--pure".to_string(),
        "run".to_string(),
        skill_str,
        format!("{}: {:#?}", "the following is the compilation results", command_res),
        format!(
            "Ensure you use the following compilation command: {} {}",
            args.compiler,
            args.compilation_args.join(" ")
        ),
        format!(
            "Fix level: {:?}. At 'Errors' level fix ONLY compilation errors (ignore warnings). At 'Warning' level fix errors AND warnings. At 'Logic' level fix errors, warnings, AND logic issues.",
            args.level
        ),
        "Fix the issues according to the level, and provide a summary of what you did, dont stop till its fixed. Output must use one line per result prefixed with ERROR_MESSAGE|, WARNING_MESSAGE|, or LOGIC_MESSAGE|. Do not use ANSI escapes. Do not use markdown styling. Do not add extra commentary."
            .to_string(),
    ]
}

fn fix(args: Cli, command_res: Output) {
    let mut sp = msg::step("Applying fix");
    let res = default_opencode_prompt(&args, &command_res);
    let output = Command::new("opencode").args(res).output();
    sp.stop();

    match output {
        Ok(o) => {
            if o.status.success() {
                msg::success("fix applied successfully");
            } else {
                msg::error(
                    "S002",
                    "fix attempt failed",
                    Some("AI could not resolve the compilation errors"),
                );
                exit(1);
            }

            let Ok(stdout) = std::str::from_utf8(&o.stdout) else {
                return;
            };

            eprintln!("{}", stdout);
            eprint!("\n\n");
            render_marked_output(stdout);
        }
        Err(_) => {
            msg::error(
                "S003",
                "failed to invoke opencode",
                Some("ensure opencode is installed and in PATH"),
            );
            exit(1);
        }
    }
}

fn main() {
    let args = Cli::parse();

    if args.compiler.is_empty() {
        msg::error(
            "S004",
            "no compiler specified",
            Some("usage: sentinel <compiler> [args...]"),
        );
        exit(1);
    }

    let mut sp = msg::step("Compiling");
    let output = match Command::new(args.compiler.clone())
        .args(args.compilation_args.clone())
        .output()
    {
        Ok(o) => o,
        Err(_) => {
            msg::error(
                "S005",
                "failed to execute compiler",
                Some(&format!("could not run '{}'", args.compiler)),
            );
            exit(1);
        }
    };

    sp.stop();

    if output.stderr.is_empty() {
        msg::success("compiled successfully");
        return;
    }

    msg::warn("compilation failed, attempting fix...");

    // NOTE: copm
    if args.verbose
        && let Ok(stderr) = std::str::from_utf8(&output.stderr)
    {
        eprint!("{}", stderr);
    }

    fix(args, output);
}
