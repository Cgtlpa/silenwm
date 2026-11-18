#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub fn master_stack(
    area: Rect,
    window_count: usize,
    master_ratio: f32,
    outer_gap: u32,
    inner_gap: u32,
) -> Vec<Rect> {
    if window_count == 0 {
        return Vec::new();
    }

    let gap = outer_gap.min(area.width / 2).min(area.height / 2);

    let usable = Rect {
        x: area.x + gap as i32,
        y: area.y + gap as i32,
        width: area.width.saturating_sub(gap * 2),
        height: area.height.saturating_sub(gap * 2),
    };

    if window_count == 1 {
        return vec![usable];
    }

    let ratio = master_ratio.clamp(0.1, 0.9);

    let master_width = ((usable.width.saturating_sub(inner_gap) as f32) * ratio).round() as u32;

    let stack_width = usable.width.saturating_sub(master_width + inner_gap);

    let stack_count = window_count - 1;
    let total_gaps = inner_gap.saturating_mul((stack_count - 1) as u32);

    let available_height = usable.height.saturating_sub(total_gaps);

    let base_height = available_height / stack_count as u32;
    let remainder = available_height % stack_count as u32;

    let mut windows = Vec::new();

    // Master window
    windows.push(Rect {
        x: usable.x,
        y: usable.y,
        width: master_width,
        height: usable.height,
    });

    // Stack windows
    let mut current_y = usable.y;

    for i in 0..stack_count {
        let height = base_height + u32::from((i as u32) < remainder);

        windows.push(Rect {
            x: usable.x + (master_width + inner_gap) as i32,
            y: current_y,
            width: stack_width,
            height,
        });

        current_y += (height + inner_gap) as i32;
    }

    windows
}
