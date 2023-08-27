use std::collections::HashMap;

use egui::Key;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq)]
enum MainViewMode {
    TIMELINE = 0,
    CODE = 1,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone)]
pub struct Clip {
    start_time: i64,
    length: i64,
    path: String,
}

impl Clip {
    fn new(start_time: i64, path: String) -> Self {
        return Self {
            start_time: start_time,
            length: 5,
            path: path,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    pub paths: Vec<String>,
    timeline: Vec<Clip>,
    code: String,
}

fn add_clip( timeline:&mut Vec<Clip>, path: String) {
    let mut new_start_time: i64 = 0;
    if let Some(last_clip) = timeline.last() {
        new_start_time = last_clip.start_time + last_clip.length;
    }

    timeline.push(Clip::new(new_start_time, path))
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VideoEditor {
    pub project: Project,
    side_panel_shown: bool,
    view_mode: MainViewMode,

    #[serde(skip_serializing, skip_deserializing)]
    pub textures: HashMap<String,egui::TextureHandle>,
}

impl Default for VideoEditor {
    fn default() -> Self {
        return Self {
            project: Project {
                paths: [].to_vec(),
                timeline: [].to_vec(),
                code: "".to_string()
            },
            side_panel_shown: true,
            textures: HashMap::new(),
            view_mode: MainViewMode::TIMELINE
        };
    }
}

impl VideoEditor {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

}

pub fn load_texture(map: &mut HashMap<String,egui::TextureHandle>, path: &str, ctx: &egui::Context)
{
    map.insert(path.to_string(),
        // Load the texture only once.
        ctx.load_texture(
            "my-image",
            load_image_from_path(std::path::Path::new(&path)).unwrap(),
            Default::default()
        )
    );
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    return Ok(egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
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

                    if ui.add_enabled(MainViewMode::TIMELINE == self.view_mode, egui::Button::new("Code")).clicked() {
                        self.view_mode = MainViewMode::CODE;
                    }

                    if ui.add_enabled(MainViewMode::CODE == self.view_mode, egui::Button::new("Timeline")).clicked() {
                        self.view_mode = MainViewMode::TIMELINE;
                    }

                })
            });
        });

        if self.side_panel_shown {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Project Explorer");

                if ui.button("Add file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let path: String =path.to_str().unwrap().to_string();
                        load_texture(&mut self.textures, &path, ui.ctx());
                        self.project.paths.push(path);
                    }
                }

                egui::ScrollArea::vertical()
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        for item in self.project.paths.iter_mut() {
                            ui.vertical(|ui| {
                                let texture = self.textures.get(item);
                                if let Some(t) = texture {
                                    let image = ui.image(&*t, egui::Vec2::new(100.0, 100.0));
                                    let image_drag = image.interact(egui::Sense::drag());

                                    if image_drag.drag_released() {
                                        let s = image.ctx.input(|i| i.pointer.interact_pos()).unwrap();
                                        // todo: replace with check if it's inside the timeline
                                        if s.x > 200.0 {
                                            add_clip(&mut self.project.timeline, item.to_string())
                                        }
                                        println!("Dropped at: {:?}", s);
                                    }

                                    ui.label(item.to_string());
                                }
                            });
                        }
                    });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view_mode {
                MainViewMode::TIMELINE => {
                    ui.heading("Timeline");

                    ui.horizontal(|ui| {
                        for item in self.project.timeline.iter() {
                            // Render thumbnail
                            let texture = self.textures.get(&item.path);
                            if let Some(t) = texture {
                                ui.image(&*t, egui::Vec2::new(100.0, 100.0));
                            }

                            ui.label(format!("Start Time: {}", item.start_time));
                            ui.label(format!("Length: {}", item.length));
                        }
                    });
                },
                MainViewMode::CODE => {
                    ui.heading("Code");

                    if ui.text_edit_multiline(&mut self.project.code).lost_focus() {
                        println!("Code: {}", self.project.code);
                        println!("{:?}", rslint_parser::parse_text(&self.project.code, 0).syntax());
                    }
                }
            }

        });
    }
}
