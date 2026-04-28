use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, LineGauge, List, ListItem, Widget};
use ratatui_textarea::TextArea;

use crate::models::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let pomo = &self.pomo;
        let main_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![Constraint::Min(40), Constraint::Percentage(50)])
            .split(area);
        let main_layout = main_layouts[0];
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(main_layout);

        let header: Line = vec!["POMO".blue().bold()].into();
        header.centered().render(layout[0], buf);

        List::from_iter([
            ListItem::new(format!(" Session: {}", pomo.kind)),
            ListItem::new(format!(" Status: {:?}", pomo.status)),
            ListItem::new(format!(" Rem: {}:{}", pomo.rem / 60, pomo.rem % 60)),
        ])
        .render(layout[1], buf);

        LineGauge::default()
            .filled_style(Style::new().white().on_black().bold())
            .filled_symbol(symbols::line::THICK_HORIZONTAL)
            .ratio(pomo.progress / 100_f64)
            .render(layout[2], buf);

        match self.mode {
            crate::models::AppMode::Progress => {}
            crate::models::AppMode::SessionName => {
                let textarea_layout = Layout::default()
                    .constraints([
                        Constraint::Min(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ])
                    .split(main_layout)[1];

                let mut textarea = TextArea::default();
                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Session Name "),
                );
                textarea.render(textarea_layout, buf);
            }
        }
    }
}
