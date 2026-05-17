use crossterm as ct;

fn app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| frame.render_widget("Hello World!", frame.area()))?;
        if ct::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

#[tokio::main]
async fn main() {
    ratatui::run(app).unwrap();
}
