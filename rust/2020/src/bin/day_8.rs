use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use itertools::Itertools;
use petgraph::{algo::astar, graph::NodeIndex};

enum ProgramResult {
    Loop(i32),
    Finished(i32),
}

fn run_program_until_loop_or_end(program: &Program) -> ProgramResult {
    let mut counter = 0;
    let mut current_instruction: i32 = 0;

    let mut seen_instruction: HashSet<i32> = HashSet::new();

    while current_instruction < program.0.len() as i32 {
        if seen_instruction.contains(&current_instruction) {
            return ProgramResult::Loop(counter);
        }
        seen_instruction.insert(current_instruction);
        let instruction = &program.0[current_instruction as usize];

        match *instruction {
            Instruction::Acc(x) => {
                counter += x;
                current_instruction += 1
            }
            Instruction::Jmp(x) => current_instruction += x,
            _ => current_instruction += 1,
        }
    }
    ProgramResult::Finished(counter)
}

fn exercise_1(program: &Program) -> i32 {
    if let ProgramResult::Loop(x) = run_program_until_loop_or_end(program) {
        x
    } else {
        panic!("It should have looped !");
    }
}

type Graph = petgraph::graph::Graph<i32, ()>;

fn construct_graph_program(
    program: &Program,
) -> (Graph, NodeIndex, NodeIndex, HashMap<NodeIndex, bool>) {
    let n = program.0.len() as i32;

    let instruction_range = 0..n;

    let mut nodes_part_1: HashMap<i32, NodeIndex> = HashMap::new();
    let mut nodes_part_2: HashMap<i32, NodeIndex> = HashMap::new();
    let mut node_to_part: HashMap<NodeIndex, bool> = HashMap::new();

    let mut graph: Graph = Graph::new();

    let initial_node = graph.add_node(-1);
    nodes_part_1.insert(-1, initial_node);
    node_to_part.insert(initial_node, false);
    let final_node = graph.add_node(-2);
    nodes_part_2.insert(-1, final_node);
    node_to_part.insert(final_node, true);

    for index in instruction_range.clone() {
        let node_index_1 = graph.add_node(index);
        let node_index_2 = graph.add_node(index);
        nodes_part_1.insert(index, node_index_1);
        nodes_part_2.insert(index, node_index_2);
        node_to_part.insert(node_index_1, false);
        node_to_part.insert(node_index_2, true);
    }

    graph.add_edge(initial_node, *nodes_part_1.get(&0).unwrap(), ());

    let _ = program
        .0
        .iter()
        .enumerate()
        .map(|(index, instr)| {
            let index = index as i32;
            let node_index_1 = nodes_part_1.get(&index).unwrap();
            let node_index_2 = nodes_part_2.get(&index).unwrap();

            let index_n_plus_one_1 = nodes_part_1.get(&(index + 1)).unwrap_or(&final_node);
            let index_n_plus_one_2 = nodes_part_2.get(&(index + 1)).unwrap_or(&final_node);

            match instr {
                Instruction::Nop(x) => {
                    // On ajoute le "next"
                    graph.add_edge(*node_index_1, *index_n_plus_one_1, ());
                    graph.add_edge(*node_index_2, *index_n_plus_one_2, ());

                    // On saute sur l'autre partie (transformation en jmp)
                    let node_target = nodes_part_2.get(&(index + x)).unwrap_or(&final_node);
                    graph.add_edge(*node_index_1, *node_target, ());
                }
                Instruction::Acc(_) => {
                    // On ajoute le "next"
                    graph.add_edge(*node_index_1, *index_n_plus_one_1, ());
                    graph.add_edge(*node_index_2, *index_n_plus_one_2, ());
                }
                Instruction::Jmp(x) => {
                    // On ajoute le jump
                    let node_target_1 = nodes_part_1.get(&(index + x)).unwrap_or(&final_node);
                    graph.add_edge(*node_index_1, *node_target_1, ());
                    let node_target_2 = nodes_part_2.get(&(index + x)).unwrap_or(&final_node);
                    graph.add_edge(*node_index_2, *node_target_2, ());

                    // On saute à la suivant dans l'autre partie (transformation en Nop)
                    graph.add_edge(*node_index_1, *index_n_plus_one_2, ());
                }
            }
        })
        .collect::<Vec<_>>();
    (graph, initial_node, final_node, node_to_part)
}

fn find_changing_node_index(program: &Program) -> usize {
    let (graph, initial_node, final_node, node_to_part) = construct_graph_program(program);
    // use petgraph::dot::Dot;
    // println!("{:?}", Dot::with_config(&graph, &[]));

    let (_, path) = astar(
        &graph,
        initial_node,
        |finish| finish == final_node,
        |_| 1,
        |_| 0,
    )
    .expect("No path between initial and final node");

    let (position, _) = path
        .iter()
        .find_position(|x| *node_to_part.get(&x).unwrap())
        .expect("No jumping");

    let node_to_change = path[position - 1];
    *graph.node_weight(node_to_change).unwrap() as usize
}

fn exercise_2(program: &mut Program) -> i32 {
    let node_to_change_index = find_changing_node_index(program);
    use Instruction::*;

    let new_instruction = match program.0[node_to_change_index] {
        Nop(x) => Jmp(x),
        Jmp(x) => Nop(x),
        _ => panic!("Jump in something else than nop or jmp"),
    };
    program.0[node_to_change_index] = new_instruction;

    if let ProgramResult::Finished(x) = run_program_until_loop_or_end(program) {
        x
    } else {
        panic!("The program do not stop after modification");
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

peg::parser! {
grammar instruction_parser() for str {
    rule base_number() -> i32
        = n:$(['0'..='9']+) { n.parse().unwrap() }
    rule positive_number() -> i32
        = "+" n:base_number() { n }
    rule neg_number() -> i32
        = "-" n:base_number() { -n }
    rule number() -> i32
        = n:(neg_number() / positive_number())

    rule nop() -> Instruction
        = "nop " n:number() {Instruction::Nop(n)}

    rule acc() -> Instruction
        = "acc " n:number() {Instruction::Acc(n)}

    rule jmp() -> Instruction
        = "jmp " n:number() {Instruction::Jmp(n)}

    pub rule instruction() -> Instruction
        = instr:(nop()/acc()/jmp())
    }
}

struct Program(Vec<Instruction>);

fn main() {
    let mut program = Program(
        std::io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .map(|line| instruction_parser::instruction(line.as_str()).unwrap())
            .collect(),
    );

    println!("{}", exercise_1(&program));
    println!("{}", exercise_2(&mut program));
}

#[test]
fn test_parser() {
    assert_eq!(
        instruction_parser::instruction("nop +20").unwrap(),
        Instruction::Nop(20)
    );
    assert_eq!(
        instruction_parser::instruction("nop -99").unwrap(),
        Instruction::Nop(-99)
    );
    assert_eq!(
        instruction_parser::instruction("acc +10").unwrap(),
        Instruction::Acc(10)
    );
    assert_eq!(
        instruction_parser::instruction("acc -20").unwrap(),
        Instruction::Acc(-20)
    );
    assert_eq!(
        instruction_parser::instruction("jmp +42").unwrap(),
        Instruction::Jmp(42)
    );
    assert_eq!(
        instruction_parser::instruction("jmp -1337").unwrap(),
        Instruction::Jmp(-1337)
    );
}

#[test]
fn test_exo_1() {
    let program = Program(
        vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .map(|line| instruction_parser::instruction(line).unwrap())
        .collect(),
    );
    assert_eq!(exercise_1(&program), 5);
}

#[test]
fn test_exo_2() {
    let mut program = Program(
        vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .map(|line| instruction_parser::instruction(line).unwrap())
        .collect(),
    );
    assert_eq!(exercise_2(&mut program), 8);
}
