extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

type NodeID = char;

#[derive(Clone)]
struct Graph {
    nodes: HashSet<NodeID>,
    successors: HashMap<NodeID, HashSet<NodeID>>,
    predecessors: HashMap<NodeID, HashSet<NodeID>>,
}

#[derive(PartialEq, Eq)]
struct MinHeapNodeID(NodeID);

impl PartialOrd<MinHeapNodeID> for MinHeapNodeID {
    fn partial_cmp(&self, other: &MinHeapNodeID) -> Option<Ordering> {
        Some(self.0.cmp(&other.0).reverse())
    }
}

impl Ord for MinHeapNodeID {
    fn cmp(&self, other: &MinHeapNodeID) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn parse_input() -> Graph {
    let line_parser =
        Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin\.").unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut nodes: HashSet<NodeID> = HashSet::new();
    let mut successors: HashMap<NodeID, HashSet<NodeID>> = HashMap::new();
    let mut predecessors: HashMap<NodeID, HashSet<NodeID>> = HashMap::new();
    for line in buffer.lines() {
        let caps = line_parser.captures(line).unwrap();
        let from = caps[1].chars().next().unwrap();
        let to = caps[2].chars().next().unwrap();
        nodes.insert(from);
        nodes.insert(to);
        successors.entry(from).or_default().insert(to);
        successors.entry(to).or_default();
        predecessors.entry(from).or_default();
        predecessors.entry(to).or_default().insert(from);
    }

    Graph {
        nodes,
        successors,
        predecessors,
    }
}

fn topological_order(mut graph: Graph) -> Vec<NodeID> {
    let mut order = Vec::new();
    let mut next: BinaryHeap<MinHeapNodeID> = graph
        .nodes
        .iter()
        .filter(|&node| graph.predecessors[node].is_empty())
        .map(|&node| MinHeapNodeID(node))
        .collect();

    while let Some(MinHeapNodeID(u)) = next.pop() {
        order.push(u);
        for &v in graph.successors[&u].iter() {
            graph.predecessors.get_mut(&v).unwrap().remove(&u);
            if graph.predecessors[&v].is_empty() {
                next.push(MinHeapNodeID(v));
            }
        }
        graph.nodes.remove(&u);
        graph.successors.remove(&u);
        graph.predecessors.remove(&u);
    }

    order
}

enum WorkerState {
    Working(NodeID, u32),
    Idle,
}

fn step_length(step: NodeID) -> u32 {
    61 + step.to_digit(36).unwrap() - 'A'.to_digit(36).unwrap()
}

fn step_remaining_work(graph: &Graph, step: NodeID) -> u32 {
    step_length(step)
        + graph.successors[&step]
            .iter()
            .map(|&succ| step_remaining_work(graph, succ))
            .max()
            .unwrap_or(0)
}

fn work(mut graph: Graph, n_workers: usize) -> u32 {
    let mut time = 0;
    let mut workers = Vec::with_capacity(n_workers);
    for _ in 0..n_workers {
        workers.push(WorkerState::Idle);
    }
    let mut next: BinaryHeap<(u32, NodeID)> = graph
        .nodes
        .iter()
        .filter(|&node| graph.predecessors[node].is_empty())
        .map(|&node| (step_remaining_work(&graph, node), node))
        .collect();

    while !graph.nodes.is_empty() {
        for worker in workers.iter_mut() {
            if let WorkerState::Idle = worker {
                if let Some((_, step)) = next.pop() {
                    *worker = WorkerState::Working(step, step_length(step))
                }
            }
        }

        time += 1;


        for worker in workers.iter_mut() {
            *worker = match *worker {
                WorkerState::Working(step, 1) => {
                    for &next_step in graph.successors[&step].iter() {
                        graph
                            .predecessors
                            .get_mut(&next_step)
                            .unwrap()
                            .remove(&step);
                        if graph.predecessors[&next_step].is_empty() {
                            next.push((step_remaining_work(&graph, next_step), next_step));
                        }
                    }
                    graph.nodes.remove(&step);
                    graph.successors.remove(&step);
                    graph.predecessors.remove(&step);
                    WorkerState::Idle
                }
                WorkerState::Working(step, time_remaining) => {
                    WorkerState::Working(step, time_remaining - 1)
                }
                WorkerState::Idle => WorkerState::Idle,
            }
        }
    }

    time
}

fn main() {
    let graph = parse_input();
    let order = topological_order(graph.clone());
    let mut instructions = String::new();
    for node in order {
        instructions.push(node);
    }
    println!("The steps need to be done in this order: {}", instructions);
    let n_workers = 5;
    println!(
        "Using {} workers, the sleigh can be assembled in {} seconds.",
        n_workers,
        work(graph, n_workers)
    );
}
