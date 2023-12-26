use std::ops::{Index, IndexMut};
use array_macro::array;
use bitvec::bitvec;
use bitvec::prelude::BitVec;
use itertools::{Itertools};
use aoc2023::common::read_input_lines;
use rand::{Rng};

#[inline]
fn idx(s: &str) -> u16 {
    let s = s.as_bytes();
    let mut result = 0;
    for c in s[0..3].iter() {
        result *= 26;
        result += (*c - b'a') as u16;
    }
    result
}

#[inline]
fn s(i: u16) -> String {
    format!(
        "{}{}{}",
        ((i / (26 * 26)) as u8 + b'a') as char,
        (((i / 26) % 26) as u8 + b'a') as char,
        ((i % (26)) as u8 + b'a') as char,
    )
}

// struct Node {
//     count: usize,
//     edges: Vec<Edge>,
// }
// struct Edge {
//     dest: usize,
//     count: usize
// }

const SIZE: usize = 26 * 26 * 26;
// type Graph = [BitVec; SIZE];
// type Graph = Vec<[u16; SIZE]>;

#[derive(Clone, Debug)]
struct Graph {
    idx_to_pos: [u16; SIZE],
    pos_to_idx: Vec<u16>,
    nodes: Vec<u16>,
    nodes_exist: BitVec,
    adj: Vec<Vec<u16>>,
}

impl Index<(u16, u16)> for Graph {
    type Output = u16;

    fn index(&self, index: (u16, u16)) -> &Self::Output {
        &self.adj[index.0 as usize][index.1 as usize]
        // &self.adj[self.idx_to_pos[index.0] as usize][self.idx_to_pos[index.1] as usize]
    }
}

impl IndexMut<(u16, u16)> for Graph {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        &mut self.adj[index.0 as usize][index.1 as usize]
        // &mut self.adj[self.idx_to_pos[index.0] as usize][self.idx_to_pos[index.1] as usize]
    }
}

// #[inline]
// fn add_edge(graph: &mut Graph, src: usize, dest: usize) {
//     graph[src].set(dest, true);
//     graph[dest].set(src, true);
// }

fn add_node(graph: &mut Graph, node: u16) -> u16 {
    if !graph.nodes_exist[node as usize] {
        let new = graph.pos_to_idx.len() as u16;
        graph.idx_to_pos[node as usize] = new;
        graph.pos_to_idx.push(node);
        graph.nodes.push(1);
        graph.adj.push(vec![0; graph.adj.len()]);
        for edges in graph.adj.iter_mut() {
            edges.push(0)
        }
        graph.nodes_exist.set(node as usize, true);
        new
    } else {
        graph.idx_to_pos[node as usize]
    }
}

fn create_edge(graph: &mut Graph, src: u16, dest: u16) {
    let src = add_node(graph, src);
    let dest = add_node(graph, dest);
    add_edges(graph, src, dest, 1);
}

fn add_edges(graph: &mut Graph, src: u16, dest: u16, count: u16) {
    graph[(src, dest)] += count;
    graph[(dest, src)] += count;
    // graph[src].count = 1;
    // graph[src].edges.push(Edge{dest, count: 1});
    // graph[dest].count = 1;
    // graph[dest].edges.push(Edge{dest: src, count: 1});
}

fn delete_edges(graph: &mut Graph, src: u16, dest: u16) {
    graph[(src, dest)] = 0;
    graph[(dest, src)] = 0;
}

#[inline]
fn contract_edge(graph: &mut Graph, u: u16, v: u16) -> u16 {
    // returns number of edges deleted
    // v will be deleted, u will be the new combined node

    let result = graph[(u, v)];

    let nodes = graph.adj[v as usize]
        .iter()
        .cloned()
        .enumerate()
        .filter(|(i, c)| *c > 0)
        .collect_vec();
    for (node, count) in nodes.into_iter() {
        delete_edges(graph, v, node as u16);
        if node as u16 != u {
            add_edges(graph, u, node as u16, count);
        }
    }
    graph.nodes[u as usize] += graph.nodes[v as usize];
    graph.nodes[v as usize] = 0;

    result
}

fn find_verts_of_nth_edge(graph: &Graph, n: u16) -> (u16, u16, u16) {
    let mut count = 0;
    for (u, edges) in graph.adj.iter().enumerate() {
        for (v, ec) in edges.iter().enumerate() {
            if count + ec > n {
                return (u as u16, v as u16, *ec);
            }
            count += ec;
        }
    }
    panic!("did not find edge {n}");
}

fn karger(ggraph: &mut Graph, mut ee_count: u16) {
    let v_count = ggraph.nodes.iter().sum();
    let mut rng = rand::thread_rng();
    loop {
        let graph = &mut ggraph.clone();
        let mut e_count = ee_count;
        for _ in 2..v_count {
            // double because each edge count appears twice in the adjacency matrix

            let edge = rng.gen_range(0..2 * e_count);
            let (u, v, _) = find_verts_of_nth_edge(graph, edge);

            // println!("{edge} = deleting ({} -- {}); {e_count}", s(graph.pos_to_idx[u as usize]), s(graph.pos_to_idx[v as usize]));
            // println!("before {:?} + {:?}",
            //          graph.adj[u as usize]
            //              .iter()
            //              .enumerate()
            //              .filter(|(i, p)| **p > 0)
            //              .map(|(i, c)| (s(graph.pos_to_idx[i]), c))
            //              .collect_vec(),
            //          graph.adj[v as usize]
            //              .iter()
            //              .enumerate()
            //              .filter(|(i, p)| **p > 0)
            //              .map(|(i, c)| (s(graph.pos_to_idx[i]), c))
            //              .collect_vec()
            // );

            // println!("{:?}",
            //          nodes
            //              .iter()
            //              .enumerate()
            //              .filter(|(i, c)| **c > 0)
            //              .map(|(i, c)| (s(i as u16), c))
            //              .collect_vec()
            // );

            e_count -= contract_edge(graph, u, v);

            // println!("after {:?}",
            //          graph.adj[u as usize]
            //              .iter()
            //              .enumerate()
            //              .filter(|(i, p)| **p > 0)
            //              .map(|(i, c)| (s(graph.pos_to_idx[i]), c))
            //              .collect_vec()
            // );

            // nodes[graph.pos_to_idx[u as usize] as usize] += nodes[graph.pos_to_idx[v as usize] as usize];
            // nodes[graph.pos_to_idx[v as usize] as usize] = 0;
        }

        // println!("{e_count}");
        // println!("{:?}", graph.nodes
        //     .iter()
        //     .enumerate()
        //     .filter(|(i, c)| **c > 0)
        //     .map(|(i, c)| (s(graph.pos_to_idx[i]), c))
        //     .collect_vec());

        // break;
        if e_count == 3 {
            break;
        }
    }
}

fn main() {
    let input = read_input_lines().unwrap();
    // let mut graph = array![bitvec![0; SIZE]; SIZE];
    // let mut graph = vec![array![0; SIZE]; SIZE];
    let mut graph = Graph{
        idx_to_pos: array![0; SIZE],
        pos_to_idx: Vec::with_capacity(1500),
        nodes: Vec::with_capacity(1500),
        nodes_exist: bitvec![0; SIZE],
        adj: Vec::with_capacity(1500)
    };
    let mut nodes = array![0; SIZE];

    let mut e_count = 0;
    for line in input {
        let (src, dests) = line.split_once(':').unwrap();
        let dests = dests[1..].split(' ');
        for dest in dests {
            // nodes[idx(src) as usize] = 1;
            // nodes[idx(dest) as usize] = 1;
            create_edge(&mut graph, idx(src), idx(dest));
            // add_edges(&mut graph, idx(src), idx(dest), 1);
            e_count += 1;
        }
    }

    karger(&mut graph, e_count);
    // println!("{:?}", graph[idx("jqt")]);
}