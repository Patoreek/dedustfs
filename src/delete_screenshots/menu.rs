use crate::theme;
use dialoguer::Select;
use owo_colors::OwoColorize;
use std::io::{self, Write};
use std::fs::read_dir;
use std::time::UNIX_EPOCH;

const DELETE_OLDER_THAN_30_DAYS: &str = "Delete all screenshots that are older than 30 days";
const DELETE_OLDER_THAN_60_DAYS: &str = "Delete all screenshots that are older than 60 days";
const DELETE_OLDER_THAN_90_DAYS: &str = "Delete all screenshots that are older than 90 days";
const BACK: &str = "Back";

pub fn delete_screenshots_menu() {
    let menu_theme = theme::menu_theme();
    let options: Vec<&str> = vec![DELETE_OLDER_THAN_30_DAYS, DELETE_OLDER_THAN_60_DAYS, DELETE_OLDER_THAN_90_DAYS, BACK];
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
        BACK => {
            return;
        }
        _ => {
            println!("Invalid selection");
            return;
        }
    }
}

fn delete_screenshots_older_than(days: u32) {
    println!("Deleting screenshots older than {} days...", days);
    //TODO: Add progress bar
    //TODO: show files that are being deleted above the progress bar

    let screenshots = read_dir("/Users/patrick/Documents").unwrap();
    for screenshot in screenshots {
        let screenshot = screenshot.unwrap();
        let screenshot_name: String = screenshot.file_name().to_string_lossy().into_owned();
        let screenshot_path = screenshot.path();
        let screenshot_date = screenshot.metadata().unwrap().modified().unwrap();
        let screenshot_date = screenshot_date.duration_since(UNIX_EPOCH).unwrap();
        let screenshot_date = screenshot_date.as_secs() / 86400;
        
        if screenshot_name.contains("Screenshot") {
            println!("Deleting {} ...\n", screenshot_name);
        } 
    }




    println!("{}","Screenshots deleted!".red().bold());
    println!("Press Enter to continue...");
    let _ = io::stdin().read_line(&mut String::new()).unwrap();
}