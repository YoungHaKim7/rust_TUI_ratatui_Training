use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Span},
    DefaultTerminal, Frame,
};

fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(50); 2]).spacing(1);
    let [top, main] = vertical.areas(frame.area());
    let [left, right] = horizontal.areas(main);

    let title = Line::from_iter([
        Span::from("Calender Widget").bold(),
        Span::from(" ( Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);
    //frame.render_widget(frame, left);
    //frame.render_widget(frame, right);
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}
