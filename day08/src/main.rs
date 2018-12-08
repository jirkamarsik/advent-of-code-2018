use std::io::Read;

struct Tree {
    children: Vec<Tree>,
    metadata: Vec<u32>,
}

fn parse_tree(inputs: &[&str]) -> (Tree, usize) {
    let mut length = 0;

    let n_children = inputs[0].parse().unwrap();
    let n_metadata = inputs[1].parse().unwrap();
    length += 2;

    let mut children = Vec::with_capacity(n_children);
    let mut metadata = Vec::with_capacity(n_metadata);

    for _ in 0..n_children {
        let (child, child_length) = parse_tree(&inputs[length..]);
        children.push(child);
        length += child_length;
    }

    for _ in 0..n_metadata {
        metadata.push(inputs[length].parse().unwrap());
        length += 1;
    }

    (Tree { children, metadata }, length)
}

fn parse_input() -> Tree {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    parse_tree(&buffer.trim().split(' ').collect::<Vec<_>>()).0
}

fn metadata_sum(tree: &Tree) -> u32 {
    tree.children.iter().map(metadata_sum).sum::<u32>() + tree.metadata.iter().sum::<u32>()
}

fn node_value(tree: &Tree) -> u32 {
    if tree.children.len() == 0 {
        metadata_sum(tree)
    } else {
        tree.metadata
            .iter()
            .filter_map(|&i| tree.children.get((i - 1) as usize))
            .map(node_value)
            .sum()
    }
}

fn main() {
    let tree = parse_input();
    println!(
        "The sum of the metadata entries is {}.",
        metadata_sum(&tree)
    );
    println!("The value of the root node is {}.", node_value(&tree));
}
