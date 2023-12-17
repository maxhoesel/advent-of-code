use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Arc,
    },
};

use anyhow::anyhow;
use day16::{Direction, Element, Position, SparseGrid};
use itertools::Itertools;
use tokio::{
    sync::{
        mpsc::Sender,
        mpsc::{self},
    },
    task::JoinHandle,
};
use tracing::{debug, info, trace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirror {
    SplitterLeftRight,
    SplitterUpDown,
    TopLeftDownRight,
    DownLeftTopRight,
}
impl TryFrom<char> for Mirror {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Mirror::SplitterUpDown),
            '-' => Ok(Mirror::SplitterLeftRight),
            '\\' => Ok(Mirror::TopLeftDownRight),
            '/' => Ok(Mirror::DownLeftTopRight),
            e => Err(anyhow!("Not a mirror: {e}")),
        }
    }
}
impl FromStr for Mirror {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Mirror::SplitterUpDown),
            "-" => Ok(Mirror::SplitterLeftRight),
            "\\" => Ok(Mirror::TopLeftDownRight),
            "/" => Ok(Mirror::DownLeftTopRight),
            e => Err(anyhow!("Not a mirror: {e}")),
        }
    }
}
impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mirror::SplitterLeftRight => "-",
                Mirror::SplitterUpDown => "|",
                Mirror::TopLeftDownRight => "\\",
                Mirror::DownLeftTopRight => "/",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MazeResult {
    pub launch_locations: HashSet<BeamLocation>,
    pub visited_fields: HashSet<BeamLocation>,
}

#[derive(Debug)]
pub struct MazeRunner {
    runner: JoinHandle<MazeResult>,
}
impl MazeRunner {
    pub fn new(grid: Arc<SparseGrid<Mirror>>, start: BeamLocation) -> MazeRunner {
        debug_assert!(!grid.is_empty());
        let runner = tokio::spawn(async move {
            let (update_sender, mut update_recv) = mpsc::channel(500);
            let next_id = AtomicUsize::new(1);
            let mut visited_fields = HashSet::new();
            let mut launch_locations = HashSet::new();
            let mut active_beams: HashMap<usize, JoinHandle<()>> = HashMap::new();

            // Spawn the first beam
            launch_locations.insert(start);
            let first_beam = BeamWalker {
                id: next_id.fetch_add(1, Relaxed),
                beam_tip: start,
                grid: Arc::clone(&grid),
                update_sender: update_sender.clone(),
            };
            active_beams.insert(
                first_beam.id,
                tokio::spawn(async move {
                    first_beam.run().await;
                }),
            );

            loop {
                let msg = update_recv.recv().await;
                trace!("Loop {:?}", msg);
                match msg {
                    Some(BeamUpdate::Visitation { id, location }) => {
                        if !active_beams.contains_key(&id) {
                            continue;
                        }
                        if visited_fields.contains(&location) {
                            debug!("Beam at {location:?} has reached known ground, stopping");
                            if let Some(handle) = active_beams.remove(&id) {
                                handle.abort();
                                let _ = handle.await;
                            };
                        } else {
                            visited_fields.insert(location);
                        }
                    }
                    Some(BeamUpdate::Finished { id }) => {
                        info!("Beam {id} has stopped");
                        active_beams.remove(&id);
                    }
                    Some(BeamUpdate::NewBeam { start, parent }) => {
                        if let Some(id) = parent {
                            if !active_beams.contains_key(&id) {
                                continue;
                            }
                        }

                        if launch_locations.contains(&start) {
                            debug!("Not launching at {start:?} because we already sent it out");
                            continue;
                        }

                        let id = next_id.fetch_add(1, Relaxed);
                        let runner = BeamWalker {
                            id,
                            beam_tip: start,
                            grid: Arc::clone(&grid),
                            update_sender: update_sender.clone(),
                        };
                        active_beams.insert(
                            id,
                            tokio::spawn(async move {
                                runner.run().await;
                            }),
                        );
                        launch_locations.insert(start);
                        debug!("launched beam {id} at {start:?}");
                    }
                    None => {
                        return MazeResult {
                            launch_locations,
                            visited_fields,
                        }
                    }
                };
                if active_beams.is_empty() {
                    info!("All beams have stopped, processing remaining messages...");
                    drop(update_sender);
                    drop(active_beams);
                    while let Some(msg) = update_recv.recv().await {
                        debug!("{:?}", msg);
                        match msg {
                            BeamUpdate::Visitation { id: _, location } => {
                                if visited_fields.contains(&location) {
                                    continue;
                                } else {
                                    visited_fields.insert(location);
                                }
                            }
                            _ => continue,
                        }
                    }
                    debug!("Done!");
                    return MazeResult {
                        launch_locations,
                        visited_fields,
                    };
                }
            }
        });

        MazeRunner { runner }
    }

    pub async fn results(self) -> MazeResult {
        match self.runner.await {
            Ok(l) => l,
            Err(e) => panic!("Error retrieving results! {e}"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct BeamLocation {
    pub position: Position,
    pub direction: Direction,
}
impl BeamLocation {
    fn travelling_vertically(&self) -> bool {
        match self.direction {
            Direction::Up | Direction::Down => true,
            Direction::Left | Direction::Right => false,
        }
    }
    fn travelling_horizontally(&self) -> bool {
        !self.travelling_vertically()
    }
}

#[derive(Debug)]
enum BeamUpdate {
    NewBeam {
        start: BeamLocation,
        parent: Option<usize>,
    },
    Visitation {
        id: usize,
        location: BeamLocation,
    },
    Finished {
        id: usize,
    },
}

#[derive(Debug)]

struct BeamWalker {
    id: usize,
    beam_tip: BeamLocation,
    grid: Arc<SparseGrid<Mirror>>,
    update_sender: Sender<BeamUpdate>,
}
impl BeamWalker {
    async fn run(mut self) {
        // process the spawn tile first
        self.beam_tip.direction = match self.grid.get(&self.beam_tip.position) {
            Some(Element { element, pos }) => match element {
                Mirror::SplitterLeftRight if self.beam_tip.travelling_vertically() => {
                    if pos.col > 0 && pos.col < self.grid.width() - 1 {
                        debug!("Splitting Beam {} at {pos:?}", self.id);
                        self.update_sender
                            .send(BeamUpdate::NewBeam {
                                start: BeamLocation {
                                    position: Position {
                                        row: pos.row,
                                        col: pos.col - 1,
                                    },
                                    direction: Direction::Left,
                                },
                                parent: Some(self.id),
                            })
                            .await
                            .expect("Channel closed!");
                    }
                    Direction::Right
                }
                Mirror::SplitterUpDown if self.beam_tip.travelling_horizontally() => {
                    if pos.row > 0 && pos.row < self.grid.height() - 1 {
                        debug!("Splitting Beam {} at {pos:?}", self.id);
                        self.update_sender
                            .send(BeamUpdate::NewBeam {
                                start: BeamLocation {
                                    position: Position {
                                        row: pos.row - 1,
                                        col: pos.col,
                                    },
                                    direction: Direction::Up,
                                },
                                parent: Some(self.id),
                            })
                            .await
                            .expect("Channel closed!");
                    }

                    Direction::Down
                }
                Mirror::SplitterLeftRight | Mirror::SplitterUpDown => {
                    self.beam_tip.direction // irrelevant splitter, no changes needed
                }
                Mirror::TopLeftDownRight => match self.beam_tip.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
                Mirror::DownLeftTopRight => match self.beam_tip.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                },
            },
            None => self.beam_tip.direction, // irrelevant splitter, no changes needed,
        };
        self.update_sender
            .send(BeamUpdate::Visitation {
                id: self.id,
                location: self.beam_tip,
            })
            .await
            .expect("Update channel closed");
        loop {
            let (skipped_fields, beam_tip) = match self
                .grid
                .next_in_direction(&self.beam_tip.position, &self.beam_tip.direction)
            {
                Some(Element {
                    element,
                    pos: mirror_pos,
                }) => {
                    let skipped_fields = match self.beam_tip.direction {
                        Direction::Up => (mirror_pos.row + 1..self.beam_tip.position.row)
                            .map(|row| Position {
                                row,
                                col: mirror_pos.col,
                            })
                            .collect_vec(),
                        Direction::Down => ((self.beam_tip.position.row + 1)..mirror_pos.row)
                            .map(|row| Position {
                                row,
                                col: mirror_pos.col,
                            })
                            .collect_vec(),
                        Direction::Left => ((mirror_pos.col + 1)..self.beam_tip.position.col)
                            .map(|col| Position {
                                row: mirror_pos.row,
                                col,
                            })
                            .collect_vec(),
                        Direction::Right => ((self.beam_tip.position.col + 1)..mirror_pos.col)
                            .map(|col| Position {
                                row: mirror_pos.row,
                                col,
                            })
                            .collect_vec(),
                    };
                    (
                        skipped_fields,
                        Some(match element {
                            Mirror::SplitterLeftRight if self.beam_tip.travelling_vertically() => {
                                if mirror_pos.col > 0 && mirror_pos.col < self.grid.width() - 1 {
                                    debug!("Splitting Beam {} at {mirror_pos:?}", self.id);
                                    self.update_sender
                                        .send(BeamUpdate::NewBeam {
                                            start: BeamLocation {
                                                position: mirror_pos,
                                                direction: Direction::Left,
                                            },
                                            parent: Some(self.id),
                                        })
                                        .await
                                        .expect("Channel closed!");
                                }
                                BeamLocation {
                                    position: mirror_pos,
                                    direction: Direction::Right,
                                }
                            }
                            Mirror::SplitterUpDown if self.beam_tip.travelling_horizontally() => {
                                if mirror_pos.row > 0 && mirror_pos.row < self.grid.height() - 1 {
                                    debug!("Splitting Beam {} at {mirror_pos:?}", self.id);
                                    self.update_sender
                                        .send(BeamUpdate::NewBeam {
                                            start: BeamLocation {
                                                position: mirror_pos,
                                                direction: Direction::Up,
                                            },
                                            parent: Some(self.id),
                                        })
                                        .await
                                        .expect("Channel closed!");
                                }
                                BeamLocation {
                                    position: mirror_pos,
                                    direction: Direction::Down,
                                }
                            }
                            Mirror::SplitterLeftRight | Mirror::SplitterUpDown => BeamLocation {
                                position: mirror_pos,
                                direction: self.beam_tip.direction,
                            },
                            Mirror::TopLeftDownRight => BeamLocation {
                                position: mirror_pos,
                                direction: match self.beam_tip.direction {
                                    Direction::Up => Direction::Left,
                                    Direction::Down => Direction::Right,
                                    Direction::Left => Direction::Up,
                                    Direction::Right => Direction::Down,
                                },
                            },
                            Mirror::DownLeftTopRight => BeamLocation {
                                position: mirror_pos,
                                direction: match self.beam_tip.direction {
                                    Direction::Up => Direction::Right,
                                    Direction::Down => Direction::Left,
                                    Direction::Left => Direction::Down,
                                    Direction::Right => Direction::Up,
                                },
                            },
                        }),
                    )
                }
                None => (
                    match self.beam_tip.direction {
                        Direction::Up => (0..self.beam_tip.position.row)
                            .map(|row| Position {
                                row,
                                col: self.beam_tip.position.col,
                            })
                            .collect_vec(),
                        Direction::Down => ((self.beam_tip.position.row + 1)..self.grid.height())
                            .map(|row| Position {
                                row,
                                col: self.beam_tip.position.col,
                            })
                            .collect_vec(),
                        Direction::Left => (0..(self.beam_tip.position.col))
                            .map(|col| Position {
                                row: self.beam_tip.position.row,
                                col,
                            })
                            .collect_vec(),
                        Direction::Right => ((self.beam_tip.position.col + 1)..self.grid.width())
                            .map(|col| Position {
                                row: self.beam_tip.position.row,
                                col,
                            })
                            .collect_vec(),
                    },
                    None,
                ),
            };
            debug!(
                "Beam {}, Travelled: {:?}. Next: {:?}",
                self.id, skipped_fields, beam_tip
            );
            for field in skipped_fields {
                self.update_sender
                    .send(BeamUpdate::Visitation {
                        id: self.id,
                        location: (BeamLocation {
                            position: field,
                            direction: self.beam_tip.direction,
                        }),
                    })
                    .await
                    .expect("Visited fields channel closed");
            }
            if let Some(tip) = beam_tip {
                self.update_sender
                    .send(BeamUpdate::Visitation {
                        id: self.id,
                        location: tip,
                    })
                    .await
                    .unwrap_or_else(|_| panic!("Visited fields channel closed: {:?}", self));
                self.beam_tip = tip;
            } else {
                // if the close doesn't arrive, whatever thats fine
                let _ = self
                    .update_sender
                    .send(BeamUpdate::Finished { id: self.id })
                    .await;
                return;
            }
        }
    }
}
