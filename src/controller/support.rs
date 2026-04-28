use crate::models::{Pomo, PomoKind};

pub fn next_pomo(pomo: &Pomo) -> Pomo {
    match pomo.kind {
        PomoKind::Focus => {
            if pomo.count > 0 && pomo.count.is_multiple_of(4) {
                Pomo {
                    kind: PomoKind::LongBreak,
                    count: pomo.count,
                    ..Default::default()
                }
            } else {
                Pomo {
                    kind: PomoKind::Break,
                    count: pomo.count,
                    ..Default::default()
                }
            }
        }
        _ => Pomo {
            kind: PomoKind::Focus,
            count: pomo.count + 1,
            ..Default::default()
        },
    }
}

pub fn prev_pomo(pomo: &Pomo) -> Pomo {
    match pomo.kind {
        PomoKind::Focus => {
            let prev_count = pomo.count.saturating_sub(1);
            if pomo.count > 0 && prev_count.is_multiple_of(4) {
                Pomo {
                    kind: PomoKind::LongBreak,
                    count: prev_count,
                    ..Default::default()
                }
            } else {
                Pomo {
                    kind: PomoKind::Break,
                    count: prev_count,
                    ..Default::default()
                }
            }
        }
        _ => Pomo {
            kind: PomoKind::Focus,
            count: pomo.count,
            ..Default::default()
        },
    }
}
