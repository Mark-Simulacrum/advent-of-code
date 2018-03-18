// Elevator starts at floor 1.
// Elevator can contain 2 Items. It must contain 1 item.
// Stops on each floor; must have both generator and microchip on floor.
// Goal: Bring all RTGs and microchips to 4th floor.

use std::cmp::{self, Ord, Ordering, PartialOrd};
use std::usize;
use std::u16;
use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

fn exec(floors: Floors) -> usize {
    let mut min_steps = usize::MAX;
    let mut visited = HashSet::new();
    let mut potential_states = BinaryHeap::new();
    potential_states.push(State {
        steps: 0,
        floors: floors,
    });
    while let Some(state) = potential_states.pop() {
        if state.is_done() {
            min_steps = cmp::min(min_steps, state.steps);
            continue;
        }
        if !visited.insert(state.floors.to_bits() as usize) {
            visited.reserve(potential_states.len());
            continue;
        }
        let i = state
            .floors
            .0
            .iter()
            .position(|f| f.has_elevator())
            .unwrap();
        let elevator_floor = state.floors.0[i].0;

        let mut move_two = false;
        for (a, b) in elevator_floor.iter().tuple_combinations() {
            if state.floors.can_move_up(i, a, Some(b)) {
                move_two = true;
                potential_states.push(state.with_floors(state.floors.moving(i, i + 1, a, Some(b))));
            }
        }
        let mut move_one_down = false;
        for a in elevator_floor.iter() {
            if state.floors.can_move_down(i, a, None) {
                move_one_down = true;
                potential_states.push(state.with_floors(state.floors.moving(i, i - 1, a, None)));
            }

            if !move_two {
                if state.floors.can_move_up(i, a, None) {
                    potential_states.push(state.with_floors(state.floors.moving(
                        i,
                        i + 1,
                        a,
                        None,
                    )));
                }
            }
        }

        if !move_one_down {
            for (a, b) in elevator_floor.iter().tuple_combinations() {
                if state.floors.can_move_down(i, a, Some(b)) {
                    potential_states.push(state.with_floors(state.floors.moving(
                        i,
                        i - 1,
                        a,
                        Some(b),
                    )));
                }
            }
        }
    }
    min_steps
}

pub fn part1(s: &[Item]) -> usize {
    exec(Floors::new(s))
}

pub fn part2(s: &[Item]) -> usize {
    let mut floors = Floors::new(s);
    floors.0[0].0.insert(Item::ELERIUM_GENERATOR);
    floors.0[0].0.insert(Item::ELERIUM_MICROCHIP);
    floors.0[0].0.insert(Item::DILITHIUM_GENERATOR);
    floors.0[0].0.insert(Item::DILITHIUM_MICROCHIP);
    exec(floors)
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    steps: usize,
    floors: Floors,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps).reverse()
    }
}

impl State {
    fn with_floors(&self, floors: Floors) -> Self {
        State {
            steps: self.steps + 1,
            floors: floors,
        }
    }

    fn is_done(&self) -> bool {
        self.floors.0[0..self.floors.0.len() - 1]
            .iter()
            .all(|f| f.0.is_empty())
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Floor(Item);

impl Floor {
    fn to_bits(self) -> u16 {
        self.0.bits
    }
    fn set_elevator(&mut self, e: bool) {
        self.0.set(Item::ELEVATOR, e)
    }
    fn has_elevator(&self) -> bool {
        self.0.contains(Item::ELEVATOR)
    }

    fn departing_with(self, a: Item, b: Option<Item>) -> Floor {
        let mut f = self;
        f.set_elevator(false);
        f.0.remove(a);
        if let Some(b) = b {
            f.0.remove(b);
        }
        f
    }

    // Either all microchips have a generator, or there are no generators
    fn is_valid_with(&self, a: Item, b: Option<Item>) -> bool {
        let mut item = self.0;
        item.insert(a);
        if let Some(b) = b {
            item.insert(b);
        }
        if item.is_empty() {
            return true;
        }
        if (item & Item::GENERATOR).is_empty() {
            return true;
        }
        // Shift all microchips to generators, then check that all of those generators are present
        item.to_generator().contains(item & Item::GENERATOR)
    }
}

bitflags! {
    pub struct Item: u16 {
        const THULIUM_GENERATOR    = 0b00000000_00000001;
        const THULIUM_MICROCHIP    = 0b00000000_00000010;
        const PLUTONIUM_GENERATOR  = 0b00000000_00000100;
        const PLUTONIUM_MICROCHIP  = 0b00000000_00001000;
        const STRONTIUM_GENERATOR  = 0b00000000_00010000;
        const STRONTIUM_MICROCHIP  = 0b00000000_00100000;
        const PROMETHIUM_GENERATOR = 0b00000000_01000000;
        const PROMETHIUM_MICROCHIP = 0b00000000_10000000;
        const RUTHENIUM_GENERATOR  = 0b00000001_00000000;
        const RUTHENIUM_MICROCHIP  = 0b00000010_00000000;
        const ELERIUM_GENERATOR    = 0b00000100_00000000;
        const ELERIUM_MICROCHIP    = 0b00001000_00000000;
        const DILITHIUM_GENERATOR  = 0b00010000_00000000;
        const DILITHIUM_MICROCHIP  = 0b00100000_00000000;

        const MICROCHIP            = 0b00101010_10101010;
        const GENERATOR            = 0b00010101_01010101;
        const ELEVATOR             = 0b01000000_00000000;
    }
}

// Space for 4 floors.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Floors([Floor; 4]);

impl Floors {
    fn to_bits(self) -> u64 {
        (self.0[0].to_bits() as u64) << (16 * 0) | (self.0[1].to_bits() as u64) << (16 * 1)
            | (self.0[2].to_bits() as u64) << (16 * 2)
            | (self.0[3].to_bits() as u64) << (16 * 3)
    }

    fn new(s: &[Item]) -> Floors {
        assert_eq!(s.len(), 4);
        Floors([Floor(s[0]), Floor(s[1]), Floor(s[2]), Floor(s[3])])
    }

    fn can_move_down(self, from: usize, a: Item, b: Option<Item>) -> bool {
        if from == 0 {
            return false;
        }
        !self.0[from - 1].0.is_empty() && self.0[from - 1].is_valid_with(a, b)
    }

    fn can_move_up(self, from: usize, a: Item, b: Option<Item>) -> bool {
        if from + 1 >= self.0.len() {
            return false;
        }
        self.0[from + 1].is_valid_with(a, b)
    }

    fn moving(self, from: usize, to: usize, a: Item, b: Option<Item>) -> Floors {
        let mut floors = self;
        let new_current_floor = floors.0[from].departing_with(a, b);
        floors.0[from] = new_current_floor;
        floors.0[to].set_elevator(true);
        floors.0[to].0.insert(a);
        if let Some(b) = b {
            floors.0[to].0.insert(b);
        }
        floors
    }
}

#[derive(Copy, Clone)]
struct ItemIter {
    item: Item,
}

impl Iterator for ItemIter {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! remove_return {
            ($($flag:expr),+) => {
                $(
                    {
                        let actual_flag = $flag;
                        if self.item.contains(actual_flag) {
                            self.item.remove(actual_flag);
                            return Some(actual_flag);
                        }
                    }
                )+
            }
        }

        remove_return!(
            Item::THULIUM_GENERATOR,
            Item::THULIUM_MICROCHIP,
            Item::PLUTONIUM_GENERATOR,
            Item::PLUTONIUM_MICROCHIP,
            Item::STRONTIUM_GENERATOR,
            Item::STRONTIUM_MICROCHIP,
            Item::PROMETHIUM_GENERATOR,
            Item::PROMETHIUM_MICROCHIP,
            Item::RUTHENIUM_GENERATOR,
            Item::RUTHENIUM_MICROCHIP,
            Item::ELERIUM_GENERATOR,
            Item::ELERIUM_MICROCHIP,
            Item::DILITHIUM_GENERATOR,
            Item::DILITHIUM_MICROCHIP
        );
        None
    }
}

impl Item {
    fn iter(self) -> ItemIter {
        ItemIter { item: self }
    }

    fn to_generator(self) -> Item {
        Item::from_bits((self & Item::MICROCHIP).bits() >> 1).unwrap()
    }
}

// The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium
// generator, and a strontium generator.
//
// The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
//
// The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium
// generator, and a ruthenium-compatible microchip.
//
// The fourth floor contains nothing relevant.
pub static INPUT: &[Item] = &[
    Item {
        bits: Item::ELEVATOR.bits | Item::THULIUM_GENERATOR.bits | Item::THULIUM_MICROCHIP.bits
            | Item::PLUTONIUM_GENERATOR.bits | Item::STRONTIUM_GENERATOR.bits,
    },
    Item {
        bits: Item::PLUTONIUM_MICROCHIP.bits | Item::STRONTIUM_MICROCHIP.bits,
    },
    Item {
        bits: Item::PROMETHIUM_GENERATOR.bits | Item::PROMETHIUM_MICROCHIP.bits
            | Item::RUTHENIUM_GENERATOR.bits | Item::RUTHENIUM_MICROCHIP.bits,
    },
    Item { bits: 0 },
];
