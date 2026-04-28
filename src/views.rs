use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{LineGauge, List, ListItem, Widget};

use crate::models::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let pomo = &self.pomo;
        let [main_layout, _] = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Min(40), Constraint::Percentage(50)])
            .areas(area);
        let [header_layout, details_layout, progress_layout] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Length(1),
            ])
            .spacing(1)
            .areas(main_layout);

        let header: Line = vec!["POMO".blue().bold()].into();
        header.centered().render(header_layout, buf);

        let session_name = match &self.pomo.session {
            None => "NONE",
            Some(name) => name,
        };

        List::from_iter([
            ListItem::new(format!(" Name    : {}", session_name)),
            ListItem::new(format!(" Session : {}", pomo.kind)),
            ListItem::new(format!(" Status  : {:?}", pomo.status)),
            ListItem::new(format!(" Rem     : {}:{}", pomo.rem / 60, pomo.rem % 60)),
        ])
        .render(details_layout, buf);

        LineGauge::default()
            .filled_style(Style::new().white().on_black().bold())
            .filled_symbol(symbols::line::THICK_HORIZONTAL)
            .ratio(pomo.progress / 100_f64)
            .render(progress_layout, buf);

        match self.mode {
            crate::models::AppMode::Progress => {}
            crate::models::AppMode::SessionName => {
                let textarea_layout =
                    main_layout.centered(Constraint::Max(30), Constraint::Length(3));

                self.session_name.render(textarea_layout, buf)
            }
        }
    }
}
