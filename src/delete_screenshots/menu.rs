use crate::delete_screenshots::delete_screenshots::{
    delete_screenshots_older_than, get_custom_days_from_user,
};
use crate::theme;
use dialoguer::Select;
use owo_colors::OwoColorize;

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
            let custom_days = get_custom_days_from_user(&menu_theme);
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
}
