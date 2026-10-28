use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::{
    io::{self, Write},
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
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    compilation_args: Vec<String>,
}

fn show_compiler_output(output: &Output) {
    let _ = io::stdout().write_all(&output.stdout);
    let _ = io::stderr().write_all(&output.stderr);
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
    // Bundle the instructions so installed binaries work outside this checkout.
    let skill_str = include_str!("skills/debugging.md").to_string();

    vec![
        "-m".to_string(),
        args.model.clone(),
        "--pure".to_string(),
        "run".to_string(),
        skill_str,
        format!(
            "Build status: {}\nstdout:\n{}\nstderr:\n{}",
            command_res.status,
            String::from_utf8_lossy(&command_res.stdout),
            String::from_utf8_lossy(&command_res.stderr)
        ),
        format!(
            "Build executable: {:?}. Arguments: {:?}. Preserve these argument boundaries when running the build.",
            args.compiler,
            args.compilation_args
        ),
        format!(
            "Fix level: {:?}. At 'Errors' level fix ONLY compilation errors (ignore warnings). At 'Warning' level fix errors AND warnings. At 'Logic' level fix errors, warnings, AND logic issues.",
            args.level
        ),
        "Fix the issues according to the level, and provide a summary of what you did, dont stop till its fixed. Output must use one line per result prefixed with ERROR_MESSAGE|, WARNING_MESSAGE|, or LOGIC_MESSAGE|. Do not use ANSI escapes. Do not use markdown styling. Do not add extra commentary."
            .to_string(),
    ]
}

fn fix(args: &Cli, command_res: Output) {
    let mut sp = msg::step("Applying fix");
    let res = default_opencode_prompt(args, &command_res);
    let output = Command::new("opencode").args(res).output();
    sp.stop();

    match output {
        Ok(o) => {
            if !o.status.success() {
                show_compiler_output(&o);
                msg::error(
                    "S002",
                    "fix attempt failed",
                    Some("AI could not resolve the compilation errors"),
                );
                exit(1);
            }

            render_marked_output(&String::from_utf8_lossy(&o.stdout));
            let _ = io::stderr().write_all(&o.stderr);

            // An agent process exiting successfully does not prove the build passed.
            match Command::new(&args.compiler)
                .args(&args.compilation_args)
                .output()
            {
                Ok(verified) => {
                    show_compiler_output(&verified);
                    if verified.status.success() {
                        msg::success("build verified successfully");
                    } else {
                        msg::error("S006", "build still fails after repair", None);
                        exit(1);
                    }
                }
                Err(_) => {
                    msg::error("S005", "failed to execute compiler for verification", None);
                    exit(1);
                }
            }
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

    let should_repair = !output.status.success()
        || (!matches!(args.level, Level::Error) && !output.stderr.is_empty());
    if !should_repair {
        show_compiler_output(&output);
        msg::success("compiled successfully");
        return;
    }

    msg::warn("build diagnostics require attention, attempting fix...");

    if args.verbose {
        show_compiler_output(&output);
    }

    fix(&args, output);
}
