use std::{fs::read_to_string, time::Duration};

use color_eyre::{eyre::eyre, eyre::Context, Result};
use day14::{
    cave::{Cave, CavePos, Element},
    parser::rock_formations,
};
use nom::{combinator::all_consuming, Finish};

const SAND_START: CavePos = CavePos { x: 500, y: 0 };

struct CaveApp {
    cave: Cave,
    total_sand: u32,
    paused: bool,
    speed: u8,
}
impl eframe::App for CaveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(1.0);
            ui.horizontal(|ui| {
                let paused = self.paused;
                ui.toggle_value(&mut self.paused, if paused { "▶" } else { "⏸" });
                ui.label("Speed: ");
                ui.add(egui::Slider::new(&mut self.speed, 1..=10).prefix("x"));
            });
            ui.heading(format!("Fallen Sand: {}", self.total_sand));

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.monospace(format!("{}", self.cave));
            });
            if !self.paused {
                for _ in 0..self.speed {
                    if !self.update_state() {
                        self.paused = true;
                    }
                }
            }
        });
        ctx.request_repaint_after(Duration::from_millis(25));
    }
}
impl CaveApp {
    pub fn new(cave: Cave) -> Self {
        CaveApp {
            cave: cave,
            total_sand: 0,
            paused: true,
            speed: 1,
        }
    }
    fn update_state(&mut self) -> bool {
        if self.cave.drop_sand(&SAND_START).is_some() {
            self.total_sand += 1;
            return true;
        }
        return false;
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    let input = read_to_string("input.txt").wrap_err("reading input.txt")?;

    let rocks = match all_consuming(rock_formations)(input.as_str())
        .finish()
        .map(|r| r.1)
    {
        Ok(ok) => ok,
        Err(e) => return Err(eyre!("Error while reading rocks: {}", e)),
    };

    let width = rocks.iter().map(|p| p.x).max().unwrap() + 1;
    let height = rocks.iter().map(|p| p.y).max().unwrap() + 1;
    let mut cave = Cave::new(height, width);
    for rock in &rocks {
        cave.set(rock, Element::Rock);
    }

    eframe::run_native(
        "Cave",
        eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(800.0, 600.0)),
            ..Default::default()
        },
        Box::new(|_cc| Box::new(CaveApp::new(cave))),
    );

    Ok(())
}
