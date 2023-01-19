use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};


pub fn gui(
    mut ctx: ResMut<EguiContext>,
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
            ui.heading("Pathfinding Algorithm Viewer");
            ui.small("Jack Kingham");
        }
    );
}