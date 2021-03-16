use eframe::egui::*;

pub fn knob(ui: &mut Ui, value: &mut f64, min: f64, max: f64) -> bool {
    let mut changed = false;

    let desired_size = ui.spacing().interact_size.y * Vec2::new(2.0, 2.0);

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

    if response.hovered() {
        let speed = if ui.input().modifiers.ctrl { 0.5 } else { 1.0 };

        let prev = *value;

        *value += ui.input().scroll_delta.y as f64 * 0.0025 * (max - min) * speed;
        *value = value.min(max).max(min);

        changed = prev != *value;
    }

    let visuals = ui.style().interact(&response);

    let radius = rect.height() * 0.5;

    ui.painter()
        .circle(rect.center(), radius, visuals.bg_fill, visuals.fg_stroke);

    let angle = ((*value - min) / (max - min) + 0.5) * std::f64::consts::PI * 1.5;

    let point = Vec2::angled(angle as f32) * radius * 0.75;

    ui.painter()
        .circle_filled(rect.center() + point, radius * 0.1, visuals.fg_stroke.color);

    changed
}
