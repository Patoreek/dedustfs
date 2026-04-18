use crate::theme;
use dialoguer::Select;
use owo_colors::OwoColorize;

use dialoguer::Input;
use std::io::{self};
use std::time::{Duration, SystemTime};
/// Use walkdir to iterate over the files in the directory
use walkdir::WalkDir;

const DELETE_OLDER_THAN_30_DAYS: &str = "Delete all screenshots that are older than 30 days";
const DELETE_OLDER_THAN_60_DAYS: &str = "Delete all screenshots that are older than 60 days";
const DELETE_OLDER_THAN_90_DAYS: &str = "Delete all screenshots that are older than 90 days";
const CUSTOM_DAYS: &str = "Custom days";
const CANCEL: &str = "Cancel";

pub fn delete_screenshots_menu() {
    let menu_theme = theme::menu_theme();
    let options: Vec<&str> = vec![
        DELETE_OLDER_THAN_30_DAYS,
        DELETE_OLDER_THAN_60_DAYS,
        DELETE_OLDER_THAN_90_DAYS,
        CUSTOM_DAYS,
        CANCEL,
    ];
    println!("{}", "What screenshots would you like to delete?".cyan());
    let selection = Select::with_theme(&menu_theme)
        .items(&options)
        .interact()
        .unwrap();

    match options[selection] {
        DELETE_OLDER_THAN_30_DAYS => {
            delete_screenshots_older_than(30);
        }
        DELETE_OLDER_THAN_60_DAYS => {
            delete_screenshots_older_than(60);
        }
        DELETE_OLDER_THAN_90_DAYS => {
            delete_screenshots_older_than(90);
        }
        CUSTOM_DAYS => {
            let custom_days = get_custom_days_from_user();
            delete_screenshots_older_than(custom_days);
        }
        CANCEL => {
            return;
        }
        _ => {
            println!("Invalid selection");
            return;
        }
    }

    fn get_custom_days_from_user() -> u32 {
        let custom_days: u32 = Input::with_theme(&theme::menu_theme())
            .with_prompt("Enter the number of days")
            .interact()
            .unwrap();
        custom_days
    }

    fn delete_screenshots_older_than(days: u32) {
        println!("Deleting screenshots older than {} days...", days);
        //TODO: Add progress bar
        //TODO: show files that are being deleted above the progress bar

        let home = dirs::home_dir().unwrap();
        let roots = [
            ("Downloads", home.join("Downloads")),
            ("Desktop", home.join("Desktop")),
            ("Documents", home.join("Documents")),
        ];
        let mut files = Vec::new();
        let mut file_count_by_root = [0usize; 3];
        for (i, (_, root)) in roots.iter().enumerate() {
            for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    file_count_by_root[i] += 1;
                    files.push(entry.path().to_path_buf());
                }
            }
        }

        // Files older than `days` have modified time before (now - days).
        let now = SystemTime::now();
        let cutoff_date = now
            .checked_sub(Duration::from_secs(days as u64 * 86400))
            .unwrap();

        // Width of summary separator lines (after the two leading spaces).
        const SUMMARY_RULE_LEN: usize = 44;

        let mut files_checked = 0usize;
        let mut files_deleted = 0usize;
        let mut deleted_count_by_root = [0usize; 3];

        for file in files {
            files_checked += 1;
            let file_name: String = file.file_name().unwrap().to_string_lossy().into_owned();
            let file_date = file.metadata().unwrap().modified().unwrap();

            if file_date < cutoff_date {
                if file_name.contains("Screenshot") {
                    files_deleted += 1;
                    if let Some(i) = roots
                        .iter()
                        .enumerate()
                        .find(|(_, (_, r))| file.starts_with(r))
                        .map(|(i, _)| i)
                    {
                        deleted_count_by_root[i] += 1;
                    }
                    println!("Deleting {} ...\n", file_name);
                    std::fs::remove_file(file).unwrap();
                    println!("Deleted {} ...\n", file_name.red().bold());
                }
            }
            // let file_path = file.to_path_buf();

            // if file_name.contains("Screenshot") && file_date > days as u64 {
            //     println!("Deleting {} ...\n", file_name);
            // }
        }

        println!();
        println!("  {}", "=".repeat(SUMMARY_RULE_LEN).cyan());
        println!(
            "  {}",
            format!("{:^width$}", "Summary", width = SUMMARY_RULE_LEN)
                .cyan()
                .bold()
        );
        println!("  {}", "=".repeat(SUMMARY_RULE_LEN).cyan());
        for (i, (dir_name, _)) in roots.iter().enumerate() {
            println!(
                "    {:<26} {}",
                format!("Files in {dir_name}"),
                format!("{:>8}", file_count_by_root[i]).bold()
            );
        }
        println!(
            "    {:<26} {}",
            "Files checked (total)",
            format!("{:>8}", files_checked).bold()
        );
        for (i, (dir_name, _)) in roots.iter().enumerate() {
            println!(
                "    {:<26} {}",
                format!("Deleted in {dir_name}"),
                format!("{:>8}", deleted_count_by_root[i]).green().bold()
            );
        }
        println!(
            "    {:<26} {}",
            "Screenshots deleted (total)",
            format!("{:>8}", files_deleted).green().bold()
        );
        println!("  {}", "=".repeat(SUMMARY_RULE_LEN).cyan());
        println!();
        println!("Press Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new()).unwrap();
    }
}
