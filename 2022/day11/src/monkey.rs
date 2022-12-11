use std::fmt::Display;

use itertools::Itertools;
use log::info;

pub type ItemWorryLevel = u32;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum MonkeyError {
    ItemNotFound,
    MonkeyNotFound,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InspectValue {
    Input,
    Fixed(u32),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InspectOp {
    Add,
    Mul,
    Sub,
    Div,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Monkey {
    id: usize,
    items: Vec<ItemWorryLevel>,
    inspect_op: InspectOp,
    inspect_value: InspectValue,
    worrytest_value: u32,
    target_worried: usize,
    target_unworried: usize,
    inspected_items: u32,
}
impl Monkey {
    pub fn new(
        id: usize,
        items: Vec<u32>,
        inspect_op: InspectOp,
        inspect_value: InspectValue,
        worrytest_value: u32,
        target_worried: usize,
        target_unworried: usize,
    ) -> Self {
        Monkey {
            id,
            items,
            inspect_op,
            inspect_value,
            worrytest_value,
            target_worried,
            target_unworried,
            inspected_items: 0,
        }
    }

    pub fn monkey_business(
        &mut self,
        monkeys: &mut [Monkey],
        panic_mode: bool,
    ) -> Result<(), MonkeyError> {
        for _ in 0..self.items.len() {
            let item = self.items.remove(0);
            let worried_item = self.inspect(item) / if panic_mode { 1 } else { 3 };
            info!(
                "Monkey {} is done inspecting item {}, worry dropped to {}",
                self.id, item, worried_item
            );
            self.throw_item(worried_item, monkeys)?;
        }
        Ok(())
    }

    fn inspect(&mut self, mut item: ItemWorryLevel) -> ItemWorryLevel {
        self.inspected_items += 1;
        let factor = match self.inspect_value {
            InspectValue::Input => item,
            InspectValue::Fixed(f) => f,
        };

        let worry = match self.inspect_op {
            InspectOp::Add => item + factor,
            InspectOp::Mul => item * factor,
            InspectOp::Sub => item - factor,
            InspectOp::Div => item / factor,
        };
        info!(
            "Monkey {} inspects item {}, worry rises to {}",
            self.id, item, worry
        );
        item = worry;
        item
    }

    fn throw_item(
        &mut self,
        item: ItemWorryLevel,
        monkeys: &mut [Monkey],
    ) -> Result<(), MonkeyError> {
        let target = match self.is_worried_enough(item) {
            true => {
                info!(
                    "Monkey {} thinks you are worried, throws item {} to monkey {}",
                    self.id, item, self.target_worried
                );
                self.target_worried
            }
            false => {
                info!(
                    "Monkey {} thinks you are cool, throws item {} to monkey {}",
                    self.id, item, self.target_unworried
                );
                self.target_unworried
            }
        };
        let reciever = monkeys.get_mut(target).ok_or(MonkeyError::MonkeyNotFound)?;
        reciever.take_item(item);
        Ok(())
    }

    pub fn take_item(&mut self, item: ItemWorryLevel) {
        self.items.push(item);
        info!("Monkey {} received a new item! {}", self.id, item);
    }

    fn is_worried_enough(&self, worry: ItemWorryLevel) -> bool {
        worry % self.worrytest_value == 0
    }

    pub fn inspect_count(&self) -> u32 {
        self.inspected_items
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: ", self.id)?;
        for (i, it) in self.items.iter().enumerate() {
            if i + 1 == self.items.len() {
                write!(f, "{}", it)?;
            } else {
                write!(f, "{},", it)?;
            }
        }
        Ok(())
    }
}
