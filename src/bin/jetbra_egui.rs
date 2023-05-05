#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use eframe::egui::Ui;
use eframe::{egui, Theme};

use jetbra::app::{App, Apps};
use jetbra::install::{InstallArgs, Installer};
use jetbra::jetbrains;
use jetbra::uninstall::{UninstallArgs, Uninstaller};

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 480.0)),
        default_theme: Theme::Light,
        ..Default::default()
    };
    let app = Jetbra::new()?;
    eframe::run_native("Jetbra", options, Box::new(|_cc| Box::from(app))).unwrap();
    Ok(())
}

struct Jetbra {
    apps: Vec<AppCheckbox>,
    installer: Installer,
    uninstaller: Uninstaller,
}

struct AppCheckbox {
    app: Apps,
    name: String,
    selected: bool,
}

impl Jetbra {
    fn new() -> Result<Self> {
        let jetbrains_dir = jetbrains::path()?;
        Ok(Self {
            apps: Apps::all()
                .iter()
                .map(|&app| AppCheckbox {
                    app,
                    name: App::from(app).name,
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
            self.apps.iter_mut().for_each(|app| {
                app.selected = all;
            });
        }
        self.apps.iter_mut().for_each(|app| {
            ui.checkbox(&mut app.selected, &app.name);
        });
    }

    fn install(&self, ui: &mut Ui) -> Result<()> {
        if ui.button("Install").clicked() {
            self.installer.install(&InstallArgs {
                app: self.selected_apps(),
            })?;
        }
        Ok(())
    }

    fn uninstall(&self, ui: &mut Ui) -> Result<()> {
        if ui.button("Uninstall").clicked() {
            self.uninstaller.uninstall(&UninstallArgs {
                app: self.selected_apps(),
            })?;
        }
        Ok(())
    }

    fn selected_apps(&self) -> Option<Vec<Apps>> {
        if self.selected_all() {
            return None;
        }
        let apps: Vec<Apps> = self
            .apps
            .iter()
            .filter(|app| app.selected)
            .map(|app| app.app)
            .collect();
        Some(apps)
    }

    fn selected_all(&self) -> bool {
        self.apps.iter().filter(|app| app.selected).count() == self.apps.len()
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
