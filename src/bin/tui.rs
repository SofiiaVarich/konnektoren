use std::io;

use konnektoren::tui::{init, restore, App};

fn main() -> io::Result<()> {
    let mut terminal = init()?;
    let app_result = App::default().run(&mut terminal);
    restore()?;
    app_result
}
