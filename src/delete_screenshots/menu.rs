use dialoguer::Select;
use owo_colors::OwoColorize;

const DELETE_OLDER_THAN_30_DAYS: &str = "Delete all screenshots that are older than 30 days";
const DELETE_OLDER_THAN_60_DAYS: &str = "Delete all screenshots that are older than 60 days";
const DELETE_OLDER_THAN_90_DAYS: &str = "Delete all screenshots that are older than 90 days";
const BACK: &str = "Back";

pub fn delete_screenshots_menu() {
    let options: Vec<&str> = vec![DELETE_OLDER_THAN_30_DAYS, DELETE_OLDER_THAN_60_DAYS, DELETE_OLDER_THAN_90_DAYS, BACK];
    println!("{}", "What screenshots would you like to delete?".cyan());
    let selection = Select::new()
        .items(&options)
        .interact()
        .unwrap();
    println!("You selected: {}", options[selection]);
}
