use std::{io::BufRead, collections::HashMap};

use num::integer::lcm;

use super::Solver;

struct Node {
    id: String,
    left: String,
    right: String
}

type Graph = HashMap<String, Node>;

fn parse_graph<Itr: Iterator<Item = std::io::Result<String>>>(lines: Itr) -> Graph {
    let mut graph = HashMap::new();
    for line in lines {
        let line_str = line.unwrap();
        let parts: Vec<&str> = line_str.split(" = ").collect();
        let id = parts[0].to_string();
        let parentheses_stripped = parts[1].trim_matches(|c| c == '(' || c == ')');
        let neighbours: Vec<String> = parentheses_stripped.split(", ").map(|s| s.to_string()).collect();
        let node = Node {
            id: id.clone(),
            left: neighbours[0].clone(),
            right: neighbours[1].clone()
        };
        graph.insert(id, node);
    }
    return graph;
}

fn follow_path<'a>(graph: &'a Graph, start: &'a Node, path: &'a String) -> &'a Node {
    
    path.chars().fold(start, |node, c| {
        match c {
            'L' => graph.get(&node.left).unwrap(),
            'R' => graph.get(&node.right).unwrap(),
            _ => panic!("Invalid path character: {}", c)
        }
    })
}

fn is_start_position(node: &Node) -> bool {
    node.id.contains('A')
}
fn is_end_position(node: &Node) -> bool {
    node.id.contains('Z')
}

fn steps_to_reach_end<'a>(graph: &'a Graph, start: &'a Node, path: &'a String) -> usize {
    let mut steps = 0;
    let mut current = start;
    while !is_end_position(current) {
        steps += path.len();
        current = follow_path(graph, current, path);
    }
    return steps;
}

fn steps_to_reach_end_simultaneously<'a>(graph: &'a Graph, path: &'a String) -> usize {
    let start = graph.values().filter(|node| is_start_position(node));
    
    let individual_steps = start.map(|start_node| steps_to_reach_end(graph, start_node, path));

    return individual_steps.reduce(lcm).expect("Expected at least one start position");
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let mut lines = input.lines();

        let instructions = lines.next().expect("Expected instructions").unwrap();
        lines.next().expect("Expected empty line after instructions").unwrap();

        let graph = parse_graph(lines);

        let steps = steps_to_reach_end_simultaneously(
            &graph, 
            &instructions.to_string()
        );

        println!("Steps: {}", steps);
    }
};