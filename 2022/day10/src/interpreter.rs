use std::{collections::VecDeque, fmt::Display, vec};

use log::debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Command {
    Noop,
    Addx(i32),
}
impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Command::Noop => "NOP".to_string(),
            Command::Addx(x) => format!("ADDX {}", x),
        };
        write!(f, "{}", out)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Interpreter {
    cpu: Cpu,
    input: VecDeque<Command>,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter::with_input(vec![].as_slice())
    }

    pub fn with_input(input: &[Command]) -> Self {
        Interpreter {
            cpu: Cpu::new(),
            input: input.iter().copied().collect::<VecDeque<_>>(),
        }
    }

    pub fn x(&self) -> i32 {
        self.cpu.x
    }

    pub fn push_command(&mut self, cmd: Command) {
        self.input.push_back(cmd);
    }

    /// Tick the CPU forward by one clock cycle. Returns true when the program has finished.
    pub fn tick(&mut self) -> bool {
        if self.input.is_empty() {
            debug!("Program has finished executing");
            return true;
        }
        if !self.cpu.busy() {
            self.cpu
                .set_command(self.input.pop_front().unwrap())
                .unwrap();
        } else {
        }
        self.cpu.tick();
        false
    }
}
impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum CpuError {
    Busy,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Cpu {
    x: i32,
    current_cmd: Command,
    cycles_left: u32,
}
impl Cpu {
    fn new() -> Self {
        Cpu {
            x: 1,
            current_cmd: Command::Noop,
            cycles_left: 0,
        }
    }

    fn set_command(&mut self, cmd: Command) -> Result<(), CpuError> {
        if self.cycles_left > 0 {
            return Err(CpuError::Busy);
        }
        self.current_cmd = cmd;
        self.cycles_left = match cmd {
            Command::Noop => 1,
            Command::Addx(_) => 2,
        };
        debug!(
            "New CPU command ({} cycles): {}",
            self.cycles_left, self.current_cmd
        );
        Ok(())
    }

    fn tick(&mut self) {
        self.cycles_left -= 1;
        if self.cycles_left == 0 {
            debug!("Command {} has finished", self.current_cmd);
            match self.current_cmd {
                Command::Noop => {}
                Command::Addx(y) => self.x += y,
            }
        } else {
            debug!("Command {} still processing", self.current_cmd);
        }
    }

    fn busy(&self) -> bool {
        self.cycles_left > 0
    }
}
