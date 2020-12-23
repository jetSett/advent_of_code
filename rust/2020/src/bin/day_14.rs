use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, PartialEq, Clone)]
pub struct Mask {
    pub mask1: u64,
    pub mask0: u64,
    pub floating: [bool; 36],
}

impl Mask {
    fn new() -> Self {
        Mask {
            floating: [false; 36],
            mask1: 0,
            mask0: 0xFFFFFFFFFFFFFFFF,
        }
    }
    fn from_str(s: &str) -> Self {
        let mut mask = Mask::new();
        let _ = s
            .chars()
            .rev()
            .enumerate()
            .map(|(n, c)| match c {
                '0' => mask.mask0 ^= 1 << n,
                '1' => mask.mask1 ^= 1 << n,
                'X' => mask.floating[n] = true,
                _ => (),
            })
            .collect::<Vec<_>>();
        mask
    }

    fn apply_value(&self, x: u64) -> u64 {
        x & self.mask0 | self.mask1
    }
    fn apply_mem(&self, real_address: u64) -> Address {
        let mut address = Address([None; 36]);

        let physical_address = real_address | self.mask1;

        for i in 0..36 {
            if !self.floating[i] {
                address.0[i] = Some(physical_address & (1 << i) != 0);
            }
        }

        address
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Address([Option<bool>; 36]);

impl Address {
    fn generate_all_adresses(&self) -> Vec<u64> {
        let mut total_addr: Vec<u64> = vec![];
        let mut stack: Vec<Address> = vec![self.clone()];

        while !stack.is_empty() {
            let current_addr = stack.pop().unwrap();
            let mut finished_addr = true;
            for i in 0..36 {
                if current_addr.0[i].is_none() {
                    finished_addr = false;
                    let mut new_addr1 = current_addr.clone();
                    let mut new_addr2 = current_addr.clone();
                    new_addr1.0[i] = Some(false);
                    new_addr2.0[i] = Some(true);
                    stack.push(new_addr1);
                    stack.push(new_addr2);
                    break;
                }
            }
            if finished_addr {
                total_addr.push(current_addr.to_real_address().unwrap());
            }
        }
        total_addr
    }
    fn to_real_address(&self) -> Option<u64> {
        self.0
            .iter()
            .enumerate()
            .map(|(n, b)| if (*b)? { Some(1 << n) } else { Some(0) })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    MemAssign(u64, u64),
    MaskAssign(Mask),
}

peg::parser! {
grammar instruction_parser() for str {
    rule number() -> u64
      = n:$(['0'..='9']+) { n.parse().unwrap() }
    rule mask() -> Mask
      =  s:$(("0" / "1" / "X")+) {Mask::from_str(s)}
    rule mask_assign() -> Instruction
      = "mask = " m:mask() {Instruction::MaskAssign(m)}
    rule mem_assign() -> Instruction
      = "mem[" addr:number() "] = " val:number() {Instruction::MemAssign(addr, val)}
    pub rule instruction() -> Instruction
        = instr:(mem_assign()/mask_assign()) {instr}
}
}

fn exercise_1(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut current_mask = Mask::new();

    for instr in instructions {
        match instr {
            Instruction::MemAssign(addr, val) => {
                *memory.entry(*addr).or_insert(0) = current_mask.apply_value(*val);
            }
            Instruction::MaskAssign(mask) => current_mask = mask.clone(),
        }
    }

    memory.values().sum()
}

fn exercise_2(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut current_mask = Mask::new();

    for instr in instructions {
        match instr {
            Instruction::MemAssign(addr, val) => {
                for address in current_mask.apply_mem(*addr).generate_all_adresses() {
                    *memory.entry(address).or_insert(0) = *val;
                }
            }
            Instruction::MaskAssign(mask) => current_mask = mask.clone(),
        }
    }

    memory.values().sum()
}

fn main() {
    let instructions = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|x| instruction_parser::instruction(&x).unwrap())
        .collect::<Vec<_>>();

    println!("{}", exercise_1(&instructions));
    println!("{}", exercise_2(&instructions));
}

#[test]
fn test_parser() {
    assert_eq!(
        vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
        .iter()
        .map(|x| instruction_parser::instruction(&x).unwrap())
        .collect::<Vec<_>>(),
        vec![
            Instruction::MaskAssign(Mask {
                mask0: 0xFFFFFFFFFFFFFFFF ^ 0b10,
                mask1: 0b1000000,
                floating: [
                    true, false, true, true, true, true, false, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true
                ]
            }),
            Instruction::MemAssign(8, 11),
            Instruction::MemAssign(7, 101),
            Instruction::MemAssign(8, 0)
        ]
    )
}

#[test]
fn test_address() {
    let address = Address([Some(true); 36]);
    assert_eq!(address.to_real_address().unwrap(), 0xFFFFFFFFF);
    assert_eq!(address.generate_all_adresses().len(), 1);

    let mut address = Address([Some(false); 36]);
    assert_eq!(address.to_real_address().unwrap(), 0);
    assert_eq!(address.generate_all_adresses().len(), 1);

    address.0[8] = None;
    assert_eq!(address.generate_all_adresses().len(), 2);
    assert_eq!(address.generate_all_adresses(), vec![1 << 8, 0]);

    address.0[32] = None;
    assert_eq!(address.generate_all_adresses().len(), 4);
    assert_eq!(
        address.generate_all_adresses(),
        vec![(1 << 8) + (1 << 32), 1 << 8, 1 << 32, 0]
    );
}

#[test]
fn test_apply_mem() {
    let mut mask = Mask::new();
    let mut addr = Address([Some(false); 36]);
    assert_eq!(mask.apply_mem(0), addr);

    mask.floating[4] = true;
    addr.0[4] = None;
    assert_eq!(mask.apply_mem(0), addr);

    mask.mask1 = 1 << 7;
    addr.0[7] = Some(true);
    assert_eq!(mask.apply_mem(0), addr);
}

#[test]
fn test_exo1() {
    assert_eq!(
        exercise_1(&[
            Instruction::MaskAssign(Mask {
                mask0: 0xFFFFFFFFFFFFFFFF ^ 0b10,
                mask1: 0b1000000,
                floating: [
                    true, false, true, true, true, true, false, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true
                ]
            }),
            Instruction::MemAssign(8, 11),
            Instruction::MemAssign(7, 101),
            Instruction::MemAssign(8, 0)
        ]),
        165
    );
}

#[test]
fn test_exo2() {
    let mut floating_1 = [false; 36];
    floating_1[0] = true;
    floating_1[5] = true;

    let mut floating_2 = [false; 36];
    floating_2[0] = true;
    floating_2[1] = true;
    floating_2[3] = true;
    assert_eq!(
        exercise_2(&[
            Instruction::MaskAssign(Mask {
                mask0: 0,
                mask1: 0b10010,
                floating: floating_1,
            }),
            Instruction::MemAssign(42, 100),
            Instruction::MaskAssign(Mask {
                mask0: 0,
                mask1: 0,
                floating: floating_2,
            }),
            Instruction::MemAssign(26, 1),
        ]),
        208
    );
}
