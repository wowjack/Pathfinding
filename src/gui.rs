use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::grid::GridEvent;


pub fn gui(
    mut ctx: ResMut<EguiContext>,
    mut grid_size: Local<usize>,
    mut grid_event_writer: EventWriter<GridEvent>
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
    ].into();
    style.spacing.slider_width = 350.;
    
    ctx.ctx_mut().set_style(style);
    egui::SidePanel::right("GUI panel")
        .exact_width(410.)
        .show(ctx.ctx_mut(), |ui| {
            ui.heading("Pathfinding");
            ui.small("Jack Kingham");

            ui.vertical_centered(|ui| {
                if ui.button("Solve").clicked() {
                }
                let range_slider = ui.add(egui::Slider::new(&mut *grid_size, 4..=100).step_by(1.));
                if range_slider.drag_started() || range_slider.changed() {
                    grid_event_writer.send(GridEvent::Resize(*grid_size));
                }
            });
        }
    );
}