use std::{collections::HashMap, fs};

type Wires = HashMap<String, u16>;

#[derive(Clone)]
enum Signal {
    Wire(String),
    Value(u16),
}

impl Signal {
    fn parse(s: &str) -> Signal {
        if let Ok(value) = s.parse::<u16>() {
            Signal::Value(value)
        } else {
            Signal::Wire(s.to_string())
        }
    }

    fn get_value(&self, wires: &Wires) -> Option<u16> {
        match self {
            Signal::Wire(id) => wires.get(id).copied(),
            Signal::Value(value) => Some(*value),
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Assign {
        source: Signal,
        target: String,
    },
    Gate {
        left: Signal,
        right: Signal,
        op: String,
        target: String,
    },
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        let mut splits = s.split(" -> ");
        let expr = splits.next().unwrap();
        let target = splits.next().unwrap().to_owned();

        match expr.split_whitespace().collect::<Vec<_>>()[..] {
            [source] => Instruction::Assign {
                source: Signal::parse(source),
                target,
            },
            [left, op, right] => Instruction::Gate {
                left: Signal::parse(left),
                right: Signal::parse(right),
                op: op.to_owned(),
                target,
            },
            ["NOT", source] => Instruction::Gate {
                left: Signal::Value(0),
                op: "NOT".to_owned(),
                right: Signal::parse(source),
                target,
            },
            _ => panic!("Invalid instruction: {}", s),
        }
    }

    fn exec(&self, wires: &mut Wires) -> Option<()> {
        match self {
            Instruction::Assign { source, target } => {
                let value = source.get_value(wires)?;
                wires.insert(target.to_owned(), value);
            }
            Instruction::Gate {
                left,
                right,
                op,
                target,
            } => {
                let left = left.get_value(wires)?;
                let right = right.get_value(wires)?;
                let value = match op.as_str() {
                    "AND" => left & right,
                    "OR" => left | right,
                    "LSHIFT" => left << right,
                    "RSHIFT" => left >> right,
                    "NOT" => !right,
                    _ => panic!("Invalid operator: {}", op),
                };
                wires.insert(target.to_owned(), value);
            }
        }
        Some(())
    }
}

fn get_signal_a(mut insts: Vec<Instruction>) -> u16 {
    let mut wires = Wires::new();
    while !insts.is_empty() {
        insts.retain(|inst| inst.exec(&mut wires).is_none())
    }
    wires["a"]
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut insts = input.lines().map(Instruction::parse).collect::<Vec<_>>();

    let signal_a_1 = get_signal_a(insts.clone());
    let pos = insts
        .iter()
        .position(|inst| matches!(inst, Instruction::Assign { target, .. } if target == "b"))
        .unwrap();
    insts[pos] = Instruction::Assign {
        source: Signal::Value(signal_a_1),
        target: "b".to_owned(),
    };

    println!("Part 1: {}", signal_a_1);
    println!("Part 2: {}", get_signal_a(insts));
}
