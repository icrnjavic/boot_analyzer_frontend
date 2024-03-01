use eframe::{epi, egui};
use image::{io::Reader as ImageReader, GenericImageView};
use std::io::Cursor;
use std::thread;
use std::time::Duration;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}

struct MyApp {
    name: String,
    greeting: String,
    texture_id: Option<egui::TextureId>,
    image_aspect_ratio: f32,
    content_to_display: String,
    test_result_message: String,
    errorCode_message: String,
}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: String::new(),
            greeting: String::new(),
            texture_id: None,
            image_aspect_ratio: 1.0,
            content_to_display: String::from("Output will be here"),
            test_result_message: String::from(" "), //filled once the test ends
            errorCode_message: String::from(" "), //filled once the test ends
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Gateway Logger V0.1.0"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, frame: &mut epi::Frame, _storage: Option<&dyn epi::Storage>) {
        let image_bytes = include_bytes!("logo.png");
        let image = ImageReader::new(Cursor::new(image_bytes))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode image");

        let size = image.dimensions();
        self.image_aspect_ratio = size.0 as f32 / size.1 as f32;

        let image = image.to_rgba8();
        let pixels = image.into_raw();
        let color_pixels: Vec<egui::Color32> = pixels
            .chunks(4)
            .map(|c| egui::Color32::from_rgba_premultiplied(c[0], c[1], c[2], c[3]))
            .collect();

        let texture_id = frame.tex_allocator().alloc_srgba_premultiplied([size.0 as usize, size.1 as usize].into(), &color_pixels);
        self.texture_id = Some(texture_id);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(texture_id) = self.texture_id {
                    let new_width = 30.0;
                    let new_height = new_width / self.image_aspect_ratio;
                    ui.image(texture_id, [new_width, new_height]);
                }
                ui.heading("Gateway Logger");
            });

            ui.label("__________________________________________________________________________________________________________________________________");
            ui.label(format!("Test Result: {}", self.test_result_message));
            ui.label(format!("Error code: {}", self.errorCode_message));
            ui.label("__________________________________________________________________________________________________________________________________");

            ui.horizontal(|ui| {
                ui.label("Enter pcbid: ");
                ui.add_sized([650.0, 20.0], egui::TextEdit::singleline(&mut self.name));
                if ui.button("Start").clicked() {
                    for i in 1..=50 {
                        self.content_to_display.push_str(&format!("\nline{}", i)); //Simulates gateway output
                    }
                    self.test_result_message = String::from("Check the button and LM for shorts");
                    self.errorCode_message = String::from("GPIO: pin (gpio 11) value is 0");
                }
            });

            if !self.greeting.is_empty() {
                ui.label(&self.greeting);
            }


            let scroll_area_width = 780.0;
            let scroll_area_height = 450.0;
            let frame = egui::Frame::none().margin(egui::Vec2::splat(0.0)).fill(egui::Color32::from_gray(0)); // Customize the frame as needed
            frame.show(ui, |ui| {
                ui.set_min_size(egui::vec2(scroll_area_width, scroll_area_height));
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.content_to_display)
                        .desired_width(scroll_area_width - 4.0)
                        .interactive(false));
                });
            });
        });
    }


}
