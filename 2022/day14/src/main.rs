use std::{fs::read_to_string, time::Duration};

use color_eyre::{eyre::eyre, eyre::Context, Result};
use day14::{
    cave::{Cave, CavePos, Element},
    parser::rock_formations,
};
use nom::{combinator::all_consuming, Finish};

const SAND_START: CavePos = CavePos { x: 500, y: 0 };

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct CaveApp {
    // Backend
    builder: CaveBuilder,
    cave: Cave,
    drop_location: CavePos,
    // Options
    speed: u8,
    // Count the sand
    total_sand: u32,
    // Toggles
    paused: bool,
    reset: bool,
    reset_with_floor: bool,
}
impl eframe::App for CaveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(1.0);
            ui.horizontal(|ui| {
                let paused = self.paused;
                ui.toggle_value(&mut self.paused, if paused { "▶" } else { "⏸" });
                ui.label("Speed: ");
                ui.add(egui::Slider::new(&mut self.speed, 1..=100).prefix("x"));
            });
            ui.horizontal(|ui| {
                ui.toggle_value(&mut self.reset, "Reset (no Floor)");
                ui.toggle_value(&mut self.reset_with_floor, "Reset (with Floor)");
            });
            ui.heading(format!("Fallen Sand: {}", self.total_sand));

            egui::ScrollArea::horizontal().show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| ui.monospace(format!("{}", self.cave)))
            });
            if self.reset {
                *self = CaveApp::new(self.builder.clone(), false, self.drop_location);
                return;
            }
            if self.reset_with_floor {
                *self = CaveApp::new(self.builder.clone(), true, self.drop_location);
                return;
            }
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
    pub fn new(builder: CaveBuilder, with_floor: bool, drop_location: CavePos) -> Self {
        let cave = if with_floor {
            builder
                .with_floor(2)
                .wrap_err("Constructing new Cave")
                .unwrap()
        } else {
            builder
                .without_floor()
                .wrap_err("Constructing new Cave")
                .unwrap()
        };
        CaveApp {
            builder,
            cave: cave,
            total_sand: 0,
            drop_location,
            paused: true,
            speed: 1,
            reset: false,
            reset_with_floor: false,
        }
    }
    fn update_state(&mut self) -> bool {
        match self.cave.drop_sand(&self.drop_location) {
            Ok(_) => {
                self.total_sand += 1;
                true
            }
            Err(e) => {
                match e {
                    day14::cave::DropError::IntoVoid => {
                        if self.cave.has_floor() {
                            panic!("Sand dropping into the void in a cave with a floor")
                        }
                    }
                    day14::cave::DropError::Occupied(_) => {}
                    day14::cave::DropError::NotInCave(_) => panic!("Drop location invalid"),
                };
                false
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct CaveBuilder {
    input: String,
}
impl CaveBuilder {
    pub fn new(input: String) -> Self {
        CaveBuilder { input }
    }
    pub fn with_floor(&self, offset: usize) -> Result<Cave> {
        let rocks =
            CaveBuilder::parse_rocks(self.input.as_str()).wrap_err("Generating cave with floor")?;
        let width = rocks.iter().map(|p| p.x).max().unwrap() * 2;
        let height = rocks.iter().map(|p| p.y).max().unwrap() + 1;
        let mut cave = Cave::with_floor(height + offset, width);
        for rock in &rocks {
            cave.set(rock, Element::Rock);
        }
        Ok(cave)
    }
    pub fn without_floor(&self) -> Result<Cave> {
        let rocks =
            CaveBuilder::parse_rocks(self.input.as_str()).wrap_err("Generating floorless cave")?;
        let width = rocks.iter().map(|p| p.x).max().unwrap() + 1;
        let height = rocks.iter().map(|p| p.y).max().unwrap() + 1;
        let mut cave = Cave::new(height, width);
        for rock in &rocks {
            cave.set(rock, Element::Rock);
        }
        Ok(cave)
    }

    fn parse_rocks(input: &str) -> Result<Vec<CavePos>> {
        match all_consuming(rock_formations)(input).finish().map(|r| r.1) {
            Ok(ok) => Ok(ok),
            Err(e) => return Err(eyre!("Error while reading rocks: {}", e)),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    let input = read_to_string("input.txt").wrap_err("reading input.txt")?;

    let builder = CaveBuilder::new(input);

    eframe::run_native(
        "Cave",
        eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(800.0, 600.0)),
            ..Default::default()
        },
        Box::new(|_cc| Box::new(CaveApp::new(builder, false, SAND_START))),
    );

    Ok(())
}
