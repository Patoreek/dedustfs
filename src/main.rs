use owo_colors::OwoColorize;
use dialoguer::Select;

mod terminal;
mod theme;
mod delete_screenshots;
use delete_screenshots::menu::delete_screenshots_menu;

const DELETE_SCREENSHOTS: &str = "Delete screenshots";
const EXIT: &str = "Exit";
enum MenuOptions {
    DeleteScreenshots,
    Exit,
}


impl MenuOptions {
    fn to_string(&self) -> &str {
        match self {
            MenuOptions::DeleteScreenshots => DELETE_SCREENSHOTS,
            MenuOptions::Exit => EXIT,
        }
    }
}
fn main() {
    let menu_theme = theme::menu_theme();
    let options: Vec<&str> = vec![MenuOptions::DeleteScreenshots.to_string(), MenuOptions::Exit.to_string()];
    loop {
        terminal::clear_screen();
        println!("{}", "Welcome to DedustFS".blue().bold());
        println!("{}", "What would you like to do?".cyan());
        let selection = Select::with_theme(&menu_theme)
            .items(&options)
            .interact()
            .unwrap();
        let selected_option = options[selection];
        match selected_option {
            DELETE_SCREENSHOTS => {
                terminal::clear_screen();
                delete_screenshots_menu();
            }
            EXIT => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid selection");
            }
        }
    }
}