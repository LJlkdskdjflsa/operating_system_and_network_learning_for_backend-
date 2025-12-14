//! Mini Cat/Grep - 一個簡單的檔案檢視與過濾工具
//!
//! 用法:
//!   mini_cat <filename>                    # 顯示檔案內容
//!   mini_cat <filename> <pattern>          # 過濾包含 pattern 的行
//!   mini_cat <filename> <pattern> -n       # 加上行號

use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// 程式的設定選項
struct Config {
    filename: String,
    pattern: Option<String>,
    show_line_numbers: bool,
}

impl Config {
    /// 從命令列參數解析設定
    fn from_args(args: &[String]) -> Result<Self> {
        if args.len() < 2 {
            anyhow::bail!("Usage: mini_cat <filename> [pattern] [-n|--line-numbers]");
        }

        let filename = args[1].clone();
        let mut pattern = None;
        let mut show_line_numbers = false;

        // TODO: 解析剩餘的參數
        // 提示：遍歷 args[2..] 來處理 pattern 和 --line-numbers

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

/// 讀取檔案並根據設定過濾輸出
fn run(config: &Config) -> Result<()> {
    // TODO: 實作檔案讀取和過濾邏輯
    //
    // 步驟：
    // 1. 用 File::open 打開檔案
    // 2. 用 BufReader 包裝
    // 3. 遍歷每一行
    // 4. 如果有 pattern，檢查是否包含該字串
    // 5. 如果 show_line_numbers 為 true，輸出行號

    let file = File::open(&config.filename)
        .context(format!("無法開啟檔案: {}", config.filename))?;

    let reader = BufReader::new(file);

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.context("讀取行時發生錯誤")?;

        // 檢查是否符合 pattern
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

// ============================================================
// 測試
// ============================================================

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
            "error".to_string(),
            "-n".to_string(),
        ];
        let config = Config::from_args(&args).unwrap();

        assert!(config.show_line_numbers);
    }

    #[test]
    fn test_config_no_args_should_fail() {
        let args = vec!["mini_cat".to_string()];
        let result = Config::from_args(&args);

        assert!(result.is_err());
    }
}
