use colored::Colorize;
use std::time::Duration;
#[derive(Debug, Clone)]
#[doc = "Struct documentation added by AI refactor"]
pub struct StepSummary {
    pub label: String,
    pub state: StepState,
}
#[derive(Debug, Clone)]
pub enum StepState {
    Success(Duration),
    Failed(Duration, String),
    Skipped,
}
impl StepSummary {
    #[doc = "Method documentation added by AI refactor"]
    pub fn success(label: impl Into<String>, duration: Duration) -> Self {
        Self {
            label: label.into(),
            state: StepState::Success(duration),
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn failed(label: impl Into<String>, duration: Duration, msg: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: StepState::Failed(duration, msg.into()),
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn skipped(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: StepState::Skipped,
        }
    }
}
#[doc = "Function documentation added by AI refactor"]
pub fn print_step_table(title: &str, steps: &[StepSummary], total: Duration) {
    println!("\n{}", format!("┌──── {title} ─────────────────┐").dimmed());
    for step in steps {
        match &step.state {
            StepState::Success(dur) => println!(
                "{} {} {:<22} {:>6.2}s",
                "│".dimmed(),
                "✔".green(),
                step.label,
                dur.as_secs_f64()
            ),
            StepState::Skipped => println!(
                "{} {} {:<22} {}",
                "│".dimmed(),
                "•".yellow(),
                step.label,
                "skipped".dimmed()
            ),
            StepState::Failed(dur, msg) => println!(
                "{} {} {:<22} {:>6.2}s  {}",
                "│".dimmed(),
                "✖".red(),
                step.label,
                dur.as_secs_f64(),
                truncate(msg, 30).red()
            ),
        }
    }
    println!("{} Total {:>27.2}s", "│".dimmed(), total.as_secs_f64());
    println!("{}", "└───────────────────────────────┘".dimmed());
}
#[doc = "Function documentation added by AI refactor"]
fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        value.to_string()
    } else {
        value
            .chars()
            .take(max.saturating_sub(1))
            .collect::<String>()
            + "…"
    }
}
