use crate::constants::LOGO;
use crate::models::App;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Clear, Gauge, List, ListItem, Widget};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let pomo = &self.pomo;
        let main_layout = area.centered(Constraint::Percentage(60), Constraint::Percentage(60));
        let [header_layout, details_layout, progress_layout] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(4),
                Constraint::Length(1),
            ])
            .spacing(1)
            .areas(main_layout);

        Text::raw(LOGO)
            .style(Style::default().blue().bold())
            .centered()
            .render(header_layout, buf);

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

        Gauge::default()
            .style(Modifier::BOLD)
            .gauge_style(Style::new().light_blue().on_black())
            .percent(pomo.progress as u16)
            .render(progress_layout, buf);

        match self.mode {
            crate::models::AppMode::Progress => {}
            crate::models::AppMode::SessionName => {
                let textarea_layout =
                    main_layout.centered(Constraint::Max(30), Constraint::Length(3));

                Clear.render(textarea_layout, buf);
                self.session_name.render(textarea_layout, buf)
            }
        }
    }
}
