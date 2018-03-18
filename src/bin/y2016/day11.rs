// Elevator starts at floor 1.
// Elevator can contain 2 Items. It must contain 1 item.
// Stops on each floor; must have both generator and microchip on floor.
// Goal: Bring all RTGs and microchips to 4th floor.

use std::cmp::{self, Ord, Ordering, PartialOrd};
use std::fmt;
use std::usize;
use std::u16;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

struct Executor {
    min_steps: usize,
    visited: HashSet<Floors>,
    potential_states: BinaryHeap<State>,
}

impl Executor {
    fn new() -> Executor {
        Executor {
            min_steps: usize::MAX,
            visited: HashSet::new(),
            potential_states: BinaryHeap::new(),
        }
    }

    fn push_state(&mut self, state: State) {
        if state.steps > self.min_steps {
            return;
        }
        if state.is_done() {
            self.min_steps = cmp::min(self.min_steps, state.steps);
            return;
        }

        if self.visited.insert(state.floors) {
            self.potential_states.push(state);
        }
    }

    fn next_state(&mut self) -> Option<State> {
        self.potential_states.pop()
    }
}

fn exec(floors: Floors) -> usize {
    let mut executor = Executor::new();
    executor.push_state(State {
        steps: 0,
        floors: floors,
    });
    while let Some(state) = executor.next_state() {
        let i = state.elevator_floor_idx();
        let elevator_floor = state.floors.0[i].0;

        let mut move_two_up = false;
        let mut move_one_down = false;
        for a in elevator_floor.into_iter() {
            if let Some(floors) = state.floors.move_down(i, a, None) {
                move_one_down = true;
                executor.push_state(state.with_floors(floors));
            }
        }
        for (a, b) in elevator_floor.into_iter().tuple_combinations() {
            if let Some(floors) = state.floors.move_up(i, a, Some(b)) {
                move_two_up = true;
                executor.push_state(state.with_floors(floors));
            }

            if !move_one_down {
                if let Some(floors) = state.floors.move_down(i, a, Some(b)) {
                    executor.push_state(state.with_floors(floors));
                }
            }
        }
        if !move_two_up {
            for a in elevator_floor.into_iter() {
                if let Some(floors) = state.floors.move_up(i, a, None) {
                    executor.push_state(state.with_floors(floors));
                }
            }
        }
    }
    println!("visited: {:?}", executor.visited.len());
    executor.min_steps
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
    fn elevator_floor_idx(&self) -> usize {
        self.floors.0.iter().position(|f| f.has_elevator()).unwrap()
    }

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

#[derive(Copy, Clone)]
pub struct Floor(Item);

impl Floor {
    #[inline(always)]
    fn set_elevator(&mut self, e: bool) {
        self.0.set(Item::ELEVATOR, e)
    }
    #[inline(always)]
    fn has_elevator(&self) -> bool {
        self.0.contains(Item::ELEVATOR)
    }

    // -> (paired, unpaired chips, unpaired generators)
    fn to_hashable(self) -> (u8, Item) {
        let generator = self.0 & Item::GENERATOR;
        let microchip = self.0.to_generator();
        let pairs = generator & microchip;
        let paired = pairs.bits.count_ones() as u8;
        let unpaired = self.0 & !(pairs | pairs.to_microchip());

        (paired, unpaired)
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
    fn is_valid(self) -> bool {
        if self.0.is_empty() {
            return true;
        }
        if (self.0 & Item::GENERATOR).is_empty() {
            return true;
        }
        // Shift all microchips to generators, then check that all of those generators are present
        self.0.to_generator().contains(self.0 & Item::GENERATOR)
    }
}

// Space for 4 floors.
#[derive(Copy, Clone)]
struct Floors([Floor; 4]);

impl Hash for Floors {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0[0].to_hashable().hash(state);
        self.0[1].to_hashable().hash(state);
        self.0[2].to_hashable().hash(state);
        self.0[3].to_hashable().hash(state);
    }
}

impl PartialEq for Floors {
    fn eq(&self, other: &Self) -> bool {
        self.0[0].to_hashable() == other.0[0].to_hashable()
            && self.0[1].to_hashable() == other.0[1].to_hashable()
            && self.0[2].to_hashable() == other.0[2].to_hashable()
            && self.0[3].to_hashable() == other.0[3].to_hashable()
    }
}

impl Eq for Floors {}

impl fmt::Debug for Floors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F4: {:?}\n", self.0[3].0)?;
        write!(f, "F3: {:?}\n", self.0[2].0)?;
        write!(f, "F2: {:?}\n", self.0[1].0)?;
        write!(f, "F1: {:?}", self.0[0].0)
    }
}

impl Floors {
    fn new(s: &[Item]) -> Floors {
        debug_assert_eq!(s.len(), 4);
        Floors([Floor(s[0]), Floor(s[1]), Floor(s[2]), Floor(s[3])])
    }

    fn move_down(self, from: usize, a: Item, b: Option<Item>) -> Option<Floors> {
        if from == 0 {
            return None;
        }

        // Don't move things down to empty floors
        if self.0[from - 1].0.is_empty() {
            return None;
        }

        let floors = self.moving(from, from - 1, a, b);

        if floors.0[from - 1].is_valid() {
            Some(floors)
        } else {
            None
        }
    }

    fn move_up(self, from: usize, a: Item, b: Option<Item>) -> Option<Floors> {
        if from + 1 >= self.0.len() {
            return None;
        }

        let floors = self.moving(from, from + 1, a, b);

        if floors.0[from + 1].is_valid() {
            Some(floors)
        } else {
            None
        }
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

impl Item {
    fn into_iter(self) -> ItemIter {
        ItemIter { item: self }
    }

    fn to_microchip(self) -> Item {
        Item::from_bits((self & Item::GENERATOR).bits() << 1).unwrap()
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
