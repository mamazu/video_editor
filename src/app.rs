use std::collections::HashMap;

use egui::{Key,};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    paths: Vec<String>,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VideoEditor {
    project: Project,
    side_panel_shown: bool,

    #[serde(skip_serializing, skip_deserializing)]
    textures: HashMap<String,egui::TextureHandle>,
}

impl Default for VideoEditor {
    fn default() -> Self {
        return Self {
            project: Project {paths: [].to_vec()},
            side_panel_shown: true,
            textures: HashMap::new(),
        };
    }
}

impl VideoEditor {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
            ))
}

impl eframe::App for VideoEditor {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if ctx.input(|i| i.key_pressed(Key::Num1) && i.modifiers.alt) {
            self.side_panel_shown = !self.side_panel_shown;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                    }
                    if ui.button("Save").clicked() {
                    }
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("View", |ui| {
                    if self.side_panel_shown {
                        if ui.button("Hide side menu (Alt+1)").clicked() {
                            self.side_panel_shown = false;
                        }
                    } else {
                        if ui.button("Show side menu (Alt+1)").clicked() {
                            self.side_panel_shown = true;
                        }
                    }
                })
            });
        });

        if self.side_panel_shown {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Project Explorer");

                if ui.button("Add file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let path_str = path.display().to_string();
                        if !self.textures.contains_key(&path_str) {}
                        self.textures.insert(path_str.to_string(),
                            // Load the texture only once.
                            ui.ctx().load_texture(
                                "my-image",
                                load_image_from_path(std::path::Path::new(&path_str)).unwrap(),
                                Default::default()
                            )
                        );
                        self.project.paths.push(path_str);
                    }
                }

                //egui::ScrollArea::vertical()
                    //.drag_to_scroll(false)
                    //.show(ui, |ui| {
                        for item in self.project.paths.iter() {
                            ui.vertical(|ui| {
                                let texture = self.textures.get(item);
                                if let Some(t) = texture {
                                    let mut image = ui.image(&*t, egui::Vec2::new(100.0, 100.0));
                                    image.sense = image.sense.union(egui::Sense::drag());
                                    ui.label(item);

                                    if image.dragged() {
                                        println!("Hey i am being dragged");

                                    }

                                }
                            });
                        }
                    //});
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Timeline");
        });
    }
}
