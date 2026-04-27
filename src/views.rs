use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{LineGauge, List, ListItem, Widget};

use crate::models::Pomo;

impl Widget for &Pomo {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
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
            ListItem::new(format!(" Session: {}", self.kind)),
            ListItem::new(format!(" Status: {:?}", self.status)),
            ListItem::new(format!(" Rem: {}:{}", self.rem / 60, self.rem % 60)),
        ])
        .render(layout[1], buf);

        LineGauge::default()
            .filled_style(Style::new().white().on_black().bold())
            .filled_symbol(symbols::line::THICK_HORIZONTAL)
            .ratio(self.progress / 100_f64)
            .render(layout[2], buf);
    }
}
