use crate::models::{Pomo, PomoKind};

impl Pomo {
    pub fn next_pomo(self: &mut Pomo) {
        match self.kind {
            PomoKind::Focus => {
                if self.count > 0 && self.count.is_multiple_of(4) {
                    self.kind = PomoKind::LongBreak;
                } else {
                    self.kind = PomoKind::Break
                }
            }
            _ => {
                self.kind = PomoKind::Focus;
                self.count = self.count.saturating_add(1);
            }
        }
    }

    pub fn prev_pomo(self: &mut Pomo) {
        match self.kind {
            PomoKind::Focus => {
                let prev_count = self.count.saturating_sub(1);
                if self.count > 0 && prev_count.is_multiple_of(4) {
                    self.kind = PomoKind::LongBreak;
                    self.count = prev_count;
                } else {
                    self.kind = PomoKind::Break;
                    self.count = prev_count;
                }
            }
            _ => {
                self.kind = PomoKind::Focus;
            }
        }
    }
}
