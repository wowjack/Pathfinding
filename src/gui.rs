use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::grid_update::{UpdateTimer, GridEvent, SlowTileUpdateBuffer};


pub fn gui(
    mut ctx: ResMut<EguiContext>,
    mut grid_event_writer: EventWriter<GridEvent>,
    mut update_buffer: ResMut<SlowTileUpdateBuffer>,
    mut update_timer: ResMut<UpdateTimer>,
    mut millis: Local<u64>
) {
    use crate::gui::egui::TextStyle::{Heading, Body, Monospace, Small, Button};
    use crate::gui::egui::FontFamily::{Proportional};
    use crate::gui::egui::FontId;
    let mut style = (*ctx.ctx_mut().style()).clone();
    style.text_styles = [
        (Heading, FontId::new(70.0, Proportional)),
        (Body, FontId::new(40.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.ctx_mut().set_style(style);
    egui::SidePanel::right("GUI panel")
        .exact_width(410.)
        .show(ctx.ctx_mut(), |ui| {
            ui.heading("Pathfinding");
            ui.small("Jack Kingham");

            ui.vertical_centered(|ui| {
                if ui.button("Solve").clicked() {
                    update_buffer.0.clear();
                    grid_event_writer.send(GridEvent::Clear);
                    grid_event_writer.send(GridEvent::StartSolve);
                }
                let range_slider = ui.add(egui::Slider::new(&mut *millis, 1..=1000));
                if range_slider.drag_started() || range_slider.changed() {
                    update_timer.0.set_duration(std::time::Duration::from_millis(*millis));
                }
                let clear_button = ui.button("Clear");
                if clear_button.clicked() {
                    update_buffer.0.clear();
                    grid_event_writer.send(GridEvent::Clear);
                }
                if clear_button.double_clicked() {
                    update_buffer.0.clear();
                    grid_event_writer.send(GridEvent::Reset);
                }
            });
        }
    );
}