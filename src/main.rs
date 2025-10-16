use clap::Parser;
use colored::Colorize;
use dialoguer::Input;
use std::path::Path;
use std::process::{Command, ExitCode};

/// A Git automation CLI that handles add, commit, and push operations
#[derive(Parser, Debug)]
#[command(name = "ga")]
#[command(version = "0.1.0")]
#[command(about = "Automates Git add, commit, and push workflow", long_about = None)]
struct Args {
    /// Commit message (if not provided, you'll be prompted)
    #[arg(short, long)]
    message: Option<String>,

    /// The branch origin to push to (if not provided, code is pushed to main)
    #[arg(short, long)]
    origin: Option<String>,

    /// Print verbose output from git commands
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // Check if we're in a git repository
    if !Path::new(".git").exists() {
        eprintln!(
            "{} {}",
            "Error:".red().bold(),
            "Not a git repository. No .git directory found."
        );
        return ExitCode::FAILURE;
    }

    // Step 1: git add .
    if let Err(e) = run_git_add(args.verbose) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return ExitCode::FAILURE;
    }

    // Step 2: Get commit message
    let message = match get_commit_message(args.message) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            return ExitCode::FAILURE;
        }
    };

    // Step 3: git commit
    if let Err(e) = run_git_commit(&message, args.verbose) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return ExitCode::FAILURE;
    }

    // Step 4: Determine branch and push
    let branch = args.origin.unwrap_or_else(|| "main".to_string());
    if let Err(e) = run_git_push(&branch, args.verbose) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return ExitCode::FAILURE;
    }

    println!(
        "\n{} {}",
        "✓".green().bold(),
        "Successfully pushed the code!".green()
    );

    ExitCode::SUCCESS
}

fn run_git_add(verbose: bool) -> Result<(), String> {
    println!("{} {}", "→".blue().bold(), "Running git add .".cyan());

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .map_err(|e| format!("Failed to execute git add: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "git add failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    if verbose && !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    println!("{} {}", "✓".green().bold(), "Staged all changes".green());
    Ok(())
}

fn get_commit_message(message: Option<String>) -> Result<String, String> {
    match message {
        Some(msg) => {
            if msg.trim().is_empty() {
                return Err("Commit message cannot be empty".to_string());
            }
            Ok(msg)
        }
        None => {
            println!(
                "{} {}",
                "→".blue().bold(),
                "Commit message required".cyan()
            );
            Input::<String>::new()
                .with_prompt("Enter commit message")
                .interact_text()
                .map_err(|e| format!("Failed to read input: {}", e))
                .and_then(|msg| {
                    if msg.trim().is_empty() {
                        Err("Commit message cannot be empty".to_string())
                    } else {
                        Ok(msg)
                    }
                })
        }
    }
}

fn run_git_commit(message: &str, verbose: bool) -> Result<(), String> {
    println!(
        "{} {}",
        "→".blue().bold(),
        format!("Committing with message: \"{}\"", message).cyan()
    );

    let output = Command::new("git")
        .arg("commit")
        .arg("-s")
        .arg("-m")
        .arg(message)
        .output()
        .map_err(|e| format!("Failed to execute git commit: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Check if there's nothing to commit
        if stderr.contains("nothing to commit") {
            return Err("Nothing to commit, working tree clean".to_string());
        }
        return Err(format!("git commit failed: {}", stderr));
    }

    if verbose {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    println!("{} {}", "✓".green().bold(), "Commit created".green());
    Ok(())
}

fn run_git_push(branch: &str, verbose: bool) -> Result<(), String> {
    println!(
        "{} {}",
        "→".blue().bold(),
        format!("Pushing to origin/{}", branch).cyan()
    );

    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(branch)
        .output()
        .map_err(|e| format!("Failed to execute git push: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "git push failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    if verbose {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stdout.is_empty() {
            println!("{}", stdout);
        }
        if !stderr.is_empty() {
            println!("{}", stderr);
        }
    }

    println!(
        "{} {}",
        "✓".green().bold(),
        format!("Pushed to origin/{}", branch).green()
    );
    Ok(())
}
