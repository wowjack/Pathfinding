use bevy::prelude::*;
use bevy_egui::{egui::{self, vec2}, EguiContext};

use crate::grid::GridEvent;


pub fn gui(
    mut ctx: ResMut<EguiContext>,
    mut grid_event_writer: EventWriter<GridEvent>,
    mut grid_size: &mut usize,
) {
    use crate::gui::egui::TextStyle::{Heading, Body, Monospace, Small, Button};
    use crate::gui::egui::FontFamily::{Proportional};
    use crate::gui::egui::FontId;
    let mut style = (*ctx.ctx_mut().style()).clone();
    style.text_styles = [
        (Heading, FontId::new(70.0, Proportional)),
        (Body, FontId::new(20.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(30.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ].into();
    style.spacing.slider_width = 350.;
    
    ctx.ctx_mut().set_style(style);
    egui::SidePanel::right("GUI panel")
        .exact_width(410.)
        .show(ctx.ctx_mut(), |ui| {
            ui.heading("Pathfinding");
            ui.small("Jack Kingham");

            ui.add_space(25.);

            ui.vertical_centered(|ui| {
                if ui.button("Solve").clicked() {
                }
                ui.add_space(25.);
                ui.horizontal(|ui| {
                    ui.label("Grid Size: ");
                    let range_slider = ui.add(egui::Slider::new(grid_size, 5..=100).step_by(1.));
                    if range_slider.drag_started() || range_slider.changed() {
                        grid_event_writer.send(GridEvent::Resize(*grid_size));
                    }
                });
            });
        }
    );
}