const NUM_STEPS: u8 = 2;
use colored::Colorize;
use spinners::{Spinner, Spinners};
static mut CUR_STEP: u8 = 0;

pub fn step(text: &str) -> Spinner {
    unsafe { CUR_STEP += 1 };
    let step = unsafe { CUR_STEP };

    Spinner::new(
        Spinners::SimpleDotsScrolling,
        format!(
            "{} {}",
            format!("[{}/{}]", step, NUM_STEPS).dimmed(),
            text.purple().bold()
        ),
    )
}

pub fn success(text: &str) {
    eprintln!("\n{} {}", "✓".green().bold(), text.green());
}

#[allow(dead_code)]
pub fn warn(text: &str) {
    eprintln!("\n{} {}", "⚠".yellow().bold(), text.yellow());
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
        eprintln!("\n  {} {}", "hint:".cyan(), h);
    }
}

#[allow(dead_code)]
pub fn info(text: &str) {
    eprintln!("{} {}", "→".blue().bold(), text);
}
