use aoc_macro::{generator, solution};
use hashbrown::HashSet;
aoc_macro::day!();

use y2018::device::{Device, Registers};

type Out = Device;

#[generator]
fn generator(input: &str) -> Out {
    if input.is_empty() {
        return Device::default();
    }
    Device::load(input)
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 0,
    expect = 16457176)]
fn part1(mut device: Out, example: bool) -> u32 {
    if example {
        return 0;
    }

    let mut registers = Registers::default();
    while device.step(&mut registers) {
        if device.ip == 28 {
            // this is the only instruction that mentions register 0; and the instruction is
            // eqrr; r0 == r5 is the exit condition. As such, whatever the value of register 5 is
            // the correct value to return at this point
            return registers[5];
        }
    }

    unreachable!("did not find instruction #28")
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 0,
    expect = 13625951)]
fn part2(mut device: Out, example: bool) -> u32 {
    if example {
        return 0;
    }
    let mut registers = Registers::default();
    let mut seen = HashSet::new();
    let mut last = registers[5];
    while device.step(&mut registers) {
        // this is the only instruction that mentions register 0; and the instruction is
        // eqrr; r0 == r5 is the exit condition.
        //
        // Presuming there's a cycle, we want the last value in that cycle.
        if device.ip == 28 {
            if seen.insert(registers[5]) {
                last = registers[5];
            } else {
                // seen twice
                return last;
            }
        }
    }
    unreachable!()
}

static EXAMPLE: &str = "";
static INPUT: &str = "
#ip 1
seti 123 0 5
bani 5 456 5
eqri 5 72 5
addr 5 1 1
seti 0 0 1
seti 0 2 5
bori 5 65536 4
seti 3935295 1 5
bani 4 255 2
addr 5 2 5
bani 5 16777215 5
muli 5 65899 5
bani 5 16777215 5
gtir 256 4 2
addr 2 1 1
addi 1 1 1
seti 27 1 1
seti 0 5 2
addi 2 1 3
muli 3 256 3
gtrr 3 4 3
addr 3 1 1
addi 1 1 1
seti 25 0 1
addi 2 1 2
seti 17 7 1
setr 2 2 4
seti 7 6 1
eqrr 5 0 2
addr 2 1 1
seti 5 4 1
";

/*
#include <assert.h>
#include <stdio.h>

int main() {
    int r0 = 0;
    int r2 = 0;
    int r4 = 0;
    int r5 = 0;

    i6: r4 = r5 | 65536;
    r5 = 3935295;
    i8: r2 = r4 & 255;
    r5 = r5 + r2;
    r5 = r5 & 16777215;
    r5 = r5 * 65899;
    r5 = r5 & 16777215;
    if (256 > r4) {
        if (r5 == r0) { // this is instruction 28
            goto exit;
        } else {
            goto i6;
        }
    }

    r2 = 0;
    i18:
    if (((r2 + 1) * 256) > r4) {
        r4 = r2;
        goto i8;
    } else {
        r2 = r2 + 1;
        goto i18;
    }

    return 0;
}
*/
