use clap::{Parser, ValueEnum};
use std::{
    fs,
    process::{Command, Output, exit},
};

const NUM_STEPS: u8 = 2;

mod msg {
    use colored::Colorize;

    use crate::NUM_STEPS;

    pub fn step(text: &str) {
        static mut CUR_STEP: u8 = 0;
        unsafe { CUR_STEP += 1 };
        let step = unsafe { CUR_STEP };
        eprintln!(
            "{} {}",
            format!("[{}/{}]", step, NUM_STEPS).dimmed(),
            text.bold()
        );
    }

    pub fn success(text: &str) {
        eprintln!("{} {}", "✓".green().bold(), text.green());
    }

    #[allow(dead_code)]
    pub fn warn(text: &str) {
        eprintln!("{} {}", "⚠".yellow().bold(), text.yellow());
    }

    pub fn error(code: &str, text: &str, hint: Option<&str>) {
        eprintln!(
            "{} {}{} {}",
            "✗".red().bold(),
            "error".red().bold(),
            format!("[{}]:", code).red(),
            text
        );
        if let Some(h) = hint {
            eprintln!("  {} {}", "hint:".cyan(), h);
        }
    }

    #[allow(dead_code)]
    pub fn info(text: &str) {
        eprintln!("{} {}", "→".blue().bold(), text);
    }
}

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

fn default_opencode_prompt(args: &Cli, command_res: &Output) -> Vec<String> {
    let skill_str = match get_skill("debugging.md") {
        Some(s) => s,
        None => {
            msg::error(
                "S001",
                "cannot locate skill",
                Some("ensure skills directory exists at src/skills/"),
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
        "Fix the issues according to the level, and provide a summary of what you did, dont stop till its fixed."
            .to_string(),
    ]
}

fn fix(args: Cli, command_res: Output) {
    msg::step("Applying fix...");
    let res = default_opencode_prompt(&args, &command_res);
    let output = Command::new("opencode").args(res).output();

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
            eprint!("{}", stdout);
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

    msg::step("Compiling...");
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

    if output.stderr.is_empty() {
        msg::success("compiled successfully");
        return;
    }

    msg::warn("compilation failed, attempting fix...");

    if let Ok(stderr) = std::str::from_utf8(&output.stderr) {
        eprint!("{}", stderr);
    }

    fix(args, output);
}
