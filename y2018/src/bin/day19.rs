use aoc_macro::{generator, solution};

aoc_macro::day!();

use y2018::device::{Device, Registers};

type Out = Device;

#[generator]
fn generator(input: &str) -> Out {
    Device::load(input)
}

fn simulate(mut device: Device, r0: u32) -> u32 {
    let mut registers = Registers::default();
    registers[0] = r0;
    while device.step(&mut registers) {}
    registers[0]
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 6,
    expect = 3224)]
fn part1(device: Out) -> u32 {
    simulate(device, 0)
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 6,
    expect = 32188416)]
fn part2(device: Out, example: bool) -> u32 {
    if example {
        return simulate(device, 1);
    }
    let mut r0 = 0;
    let r3 = 10551408;

    for r4 in 1..=r3 {
        if r3 % r4 != 0 {
            continue;
        }
        r0 += r4;
    }

    r0
}

static EXAMPLE: &str = "
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";
static INPUT: &str = "
#ip 1
addi 1 16 1
seti 1 4 4
seti 1 1 2
mulr 4 2 5
eqrr 5 3 5
addr 5 1 1
addi 1 1 1
addr 4 0 0
addi 2 1 2
gtrr 2 3 5
addr 1 5 1
seti 2 4 1
addi 4 1 4
gtrr 4 3 5
addr 5 1 1
seti 1 1 1
mulr 1 1 1
addi 3 2 3
mulr 3 3 3
mulr 1 3 3
muli 3 11 3
addi 5 7 5
mulr 5 1 5
addi 5 18 5
addr 3 5 3
addr 1 0 1
seti 0 7 1
setr 1 3 5
mulr 5 1 5
addr 1 5 5
mulr 1 5 5
muli 5 14 5
mulr 5 1 5
addr 3 5 3
seti 0 7 0
seti 0 6 1
";

/*
 0: r[1] = r[1] + 16        // goto 17
 1: r[4] = 1
 2: r[2] = 1

 3: r[5] = r[4] * r[2]
 4: r[5] = (r[4] * r[2]) == r[3]
 5: r[1] = r[5] + r[1]      // goto r[5] + 6 (either 6 or 7)
 6: r[1] = r[1] + 1         // goto instr=8

 7: r[0] = r[4] + r[0]

 8: r[2] = r[2] + 1
 9: r[5] = r[2] > r[3]
10: r[1] = r[1] + r[5]      // goto r[5] + 11 (either 11 or 12)
11: r[1] = 2                // goto 3

12: r[4] = r[4] + 1
13: r[5] = r[4] > r[3]
14: r[1] = r[5] + r[1]      // goto r[5] + 15 (either 15 or 16)
15: r[1] = 1                // goto 2

16: r[1] = r[1] * r[1]      // exit

17: r[3] = r[3] + 2
18: r[3] = r[3] * r[3]
19: r[3] = 19 * r[3]
20: r[3] = r[3] * 11
21: r[5] = r[5] + 7
22: r[5] = r[5] * r[1]
23: r[5] = r[5] + 18
24: r[3] = r[3] + r[5]
25: r[1] = r[1] + r[0]      // goto r[0] + 26
26: r[1] = 0                // goto 1
27: r[5] = r[1]
28: r[5] = r[5] * r[1]
29: r[5] = r[1] + r[5]
30: r[5] = r[1] * r[5]
31: r[5] = r[5] * 14
32: r[5] = r[5] * r[1]
33: r[3] = r[3] + r[5]
34: r[0] = 0
35: r[1] = 0                // goto 1

boils down to:

let r0 = 0; // 0
let r3 = 10551408; // 3

for r4 in 1..=r3 {
    if (r3 % r4 != 0) {
        continue;
    }
    r0 += r4;
}
*/
