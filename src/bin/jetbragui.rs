#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use eframe::egui::Ui;
use eframe::{egui, Theme};

use jetbra::app::{App, Apps};
use jetbra::install::Installer;
use jetbra::uninstall::Uninstaller;
use jetbra::{install, jetbrains, uninstall};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 480.0)),
        default_theme: Theme::Light,
        ..Default::default()
    };
    let app = Jetbra::new().unwrap();
    eframe::run_native("Jetbra", options, Box::new(|_cc| Box::from(app))).unwrap();
    Ok(())
}

struct Jetbra {
    app_checkboxes: Vec<AppCheckbox>,
    installer: Installer,
    uninstaller: Uninstaller,
}

struct AppCheckbox {
    app: Apps,
    name: &'static str,
    selected: bool,
}

impl Jetbra {
    fn new() -> Result<Self> {
        let jetbrains_dir = jetbrains::path()?;
        Ok(Self {
            app_checkboxes: Apps::all()
                .iter()
                .map(|&app| AppCheckbox {
                    app,
                    name: App::from(&app).name,
                    selected: true,
                })
                .collect(),
            installer: Installer::new(jetbrains_dir.clone()),
            uninstaller: Uninstaller::new(jetbrains_dir),
        })
    }

    fn select_app(&mut self, ui: &mut Ui) {
        let mut all = self.selected_all();
        if ui.checkbox(&mut all, "All").clicked() {
            self.app_checkboxes.iter_mut().for_each(|b| {
                b.selected = all;
            });
        }
        self.app_checkboxes.iter_mut().for_each(|b| {
            ui.checkbox(&mut b.selected, b.name);
        });
    }

    fn install(&self, ui: &mut Ui) -> Result<()> {
        if ui.button("Install").clicked() {
            self.installer.install(&install::Args {
                apps: self.selected_apps(),
            })?;
        }
        Ok(())
    }

    fn uninstall(&self, ui: &mut Ui) -> Result<()> {
        if ui.button("Uninstall").clicked() {
            self.uninstaller.uninstall(&uninstall::Args {
                remove_dependencies: self.selected_all(),
                apps: self.selected_apps(),
            })?;
        }
        Ok(())
    }

    fn selected_apps(&self) -> Vec<App> {
        self.app_checkboxes
            .iter()
            .filter(|b| b.selected)
            .map(|b| App::from(&b.app))
            .collect::<Vec<App>>()
    }

    fn selected_all(&self) -> bool {
        self.app_checkboxes.iter().filter(|b| b.selected).count() == self.app_checkboxes.len()
    }
}

impl eframe::App for Jetbra {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Select app
            self.select_app(ui);
            ui.vertical_centered(|ui| {
                // Install
                if let Err(err) = self.install(ui) {
                    ui.label(err.to_string());
                };
                // Uninstall
                if let Err(err) = self.uninstall(ui) {
                    ui.label(err.to_string());
                }
            });
        });
    }
}
