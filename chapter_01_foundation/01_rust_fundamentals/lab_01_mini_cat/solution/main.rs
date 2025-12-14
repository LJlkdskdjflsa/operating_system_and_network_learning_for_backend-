//! Lab 1 Reference Answer
//!
//! This is one possible implementation, not the only correct answer.
//! If your implementation passes the acceptance criteria, it's a good implementation!

use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Program configuration options
struct Config {
    filename: String,
    pattern: Option<String>,
    show_line_numbers: bool,
}

impl Config {
    /// Parse configuration from command-line arguments
    fn from_args(args: &[String]) -> Result<Self> {
        if args.len() < 2 {
            anyhow::bail!("Usage: mini_cat <filename> [pattern] [-n|--line-numbers]");
        }

        let filename = args[1].clone();
        let mut pattern = None;
        let mut show_line_numbers = false;

        for arg in &args[2..] {
            match arg.as_str() {
                "-n" | "--line-numbers" => show_line_numbers = true,
                _ => {
                    if pattern.is_none() {
                        pattern = Some(arg.clone());
                    }
                }
            }
        }

        Ok(Config {
            filename,
            pattern,
            show_line_numbers,
        })
    }
}

/// Read file and filter output based on configuration
fn run(config: &Config) -> Result<()> {
    let file = File::open(&config.filename)
        .context(format!("Failed to open file: {}", config.filename))?;

    let reader = BufReader::new(file);

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.context("Error reading line")?;

        // Check if line matches pattern
        let should_print = match &config.pattern {
            Some(pattern) => line.contains(pattern),
            None => true,
        };

        if should_print {
            if config.show_line_numbers {
                println!("{:4}: {}", line_num + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args)?;
    run(&config)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_with_filename_only() {
        let args = vec!["mini_cat".to_string(), "test.txt".to_string()];
        let config = Config::from_args(&args).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert!(config.pattern.is_none());
        assert!(!config.show_line_numbers);
    }

    #[test]
    fn test_config_with_pattern() {
        let args = vec![
            "mini_cat".to_string(),
            "test.txt".to_string(),
            "error".to_string(),
        ];
        let config = Config::from_args(&args).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.pattern, Some("error".to_string()));
    }

    #[test]
    fn test_config_with_line_numbers() {
        let args = vec![
            "mini_cat".to_string(),
            "test.txt".to_string(),
            "-n".to_string(),
        ];
        let config = Config::from_args(&args).unwrap();

        assert!(config.show_line_numbers);
    }
}
