use bevy::prelude::*;
use bevy_egui::egui;
use bevy_framepace::{FramepaceSettings, Limiter};

use super::FPSSetting;
use super::UiState;

pub fn ui_system(
    mut framepace_settings: ResMut<FramepaceSettings>,
    mut ui_state: ResMut<UiState>,
    mut contexts: bevy_egui::EguiContexts,
) {
    let ctx = contexts.ctx_mut();

    egui::panel::SidePanel::right("left panel").show(ctx, |ui| {
        ui.heading("Change Character");
        if ui.button("Mage").clicked() {
            // TODO: Character change logic
            println!("Character Change Mage");
        };
        if ui.button("Brawler").clicked() {
            // TODO: Character change logic
            println!("Character Change Brawler");
        };
        if ui.button("Figher").clicked() {
            // TODO: Character change logic
            println!("Character Change Brawler");
        };

        ui.heading("FPS Settings");
        ui.vertical(|ui| {
            ui.radio_value(&mut ui_state.fps_setting, FPSSetting::Uncapped, "Uncapped");
            ui.radio_value(&mut ui_state.fps_setting, FPSSetting::Auto, "VSync/Auto");
            ui.radio_value(&mut ui_state.fps_setting, FPSSetting::V15, "15 FPS");
            ui.radio_value(&mut ui_state.fps_setting, FPSSetting::V30, "30 FPS");
            ui.radio_value(&mut ui_state.fps_setting, FPSSetting::V60, "60 FPS");
            ui.radio_value(&mut ui_state.fps_setting, FPSSetting::V120, "120 FPS");
            ui.radio_value(&mut ui_state.fps_setting, FPSSetting::V240, "240 FPS");
            ui.radio_value(&mut ui_state.fps_setting, FPSSetting::V360, "360 FPS");
            ui.horizontal(|ui| {
                ui.radio_value(&mut ui_state.fps_setting, FPSSetting::Custom, "Custom");
                ui.add(egui::DragValue::new(&mut ui_state.fps_limit).clamp_range(1.0..=f64::INFINITY));
            });

            framepace_settings.limiter = match ui_state.fps_setting {
                // Not using Limiter::Off since it spams the logs
                FPSSetting::Uncapped => Limiter::Manual(std::time::Duration::ZERO),
                FPSSetting::Auto => Limiter::Auto,
                FPSSetting::V15 => Limiter::Manual(std::time::Duration::from_secs_f64(1.0 / 15.0)),
                FPSSetting::V30 => Limiter::Manual(std::time::Duration::from_secs_f64(1.0 / 30.0)),
                FPSSetting::V60 => Limiter::Manual(std::time::Duration::from_secs_f64(1.0 / 60.0)),
                FPSSetting::V120 => Limiter::Manual(std::time::Duration::from_secs_f64(1.0 / 120.0)),
                FPSSetting::V240 => Limiter::Manual(std::time::Duration::from_secs_f64(1.0 / 240.0)),
                FPSSetting::V360 => Limiter::Manual(std::time::Duration::from_secs_f64(1.0 / 360.0)),
                FPSSetting::Custom => Limiter::Manual(std::time::Duration::from_secs_f64(1.0 / ui_state.fps_limit)),
            }
        });
    });
}