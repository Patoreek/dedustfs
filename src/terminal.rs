use std::io::{self, Write};

/// Clear the visible terminal and move the cursor to the top-left.
pub fn clear_screen() {
    // CSI 2 J: erase display; CSI H: cursor to home (1,1).
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}
