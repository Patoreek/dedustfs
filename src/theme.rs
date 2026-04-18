use dialoguer::console::{Style, style};
use dialoguer::theme::ColorfulTheme;

pub fn menu_theme() -> ColorfulTheme {
    let mut theme = ColorfulTheme::default();
    theme.active_item_prefix = style("→ ".to_string()).for_stderr().green();
    theme.active_item_style = Style::new().for_stderr().green().bold();
    theme.inactive_item_prefix = style("  ".to_string()).for_stderr();
    theme
}
