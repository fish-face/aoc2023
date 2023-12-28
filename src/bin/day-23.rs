use std::cmp::max;
use bit_set::BitSet;
use itertools::Itertools;
use aoc2023::common::read_input_lines;
use aoc2023::coord::{Pt, Dir, PointSet};
use aoc2023::grid::Grid;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Wall, Floor, Ice(Dir)
}

impl Tile {
    fn parse(c: u8) -> Self {
        match c {
            b'#' => Tile::Wall,
            b'.' => Tile::Floor,
            b'^' => Tile::Ice(Dir::N),
            b'>' => Tile::Ice(Dir::E),
            b'v' => Tile::Ice(Dir::S),
            b'<' => Tile::Ice(Dir::W),
            _ => Tile::Floor,
        }
    }
}

fn part1(map: &Grid<Tile>, pos: Pt<isize>, target: Pt<isize>, mut hist: PointSet<usize>, dist: usize) -> usize {
    if hist.contains(pos.into()) {
        return 0
    }
    hist.insert(pos.into());
    if pos == target {
        return dist;
    }

    [Dir::N, Dir::E, Dir::S, Dir::W].iter().filter_map(|dir| {
        let next = pos.walk(*dir, 1);
        if !map.contains_isize(next) {
            return None;
        }

        match map[next.into()] {
            Tile::Wall => { None }
            Tile::Floor => { Some(part1(&map, next, target, hist.clone(), dist+1)) }
            Tile::Ice(next_dir) => {
                if *dir == next_dir {
                    Some(part1(&map, next, target,  hist.clone(), dist+1))
                } else {
                    None
                }
            }
        }
    }).max().unwrap_or(0)
}

#[derive(Clone, Debug)]
struct Edge {
    weight: usize,
    to: usize,
}

type Graph = Vec<Vec<Edge>>;

fn contract(
    map: &Grid<Tile>,
    pos: Pt<isize>,
    from: usize,
    dist: usize,
    forward: Option<bool>,
    graph: &mut Graph,
    digraph: &mut Graph,
    // pt --> (node index, dist from node)
    map_to_graph: &mut Grid<Option<(usize, usize)>>,
) {
    if let Some((node, d_dist)) = map_to_graph[pos.into()] {
        // println!("arrived at {pos} from {from} and it belongs to {node}");
        if node == from {
            // println!("    cycle");
            return;
        }
        update_connection(from, d_dist + dist+1, forward.unwrap(), graph, digraph, node);
        return;
    } else {
        map_to_graph[pos.into()] = Some((from, dist));
    }
    let neighbours = [Dir::N, Dir::E, Dir::S, Dir::W].iter().map(|dir|
        (dir, pos.walk(*dir, 1))).collect_vec();
    // let neighbours = pos
    //     .neighbours4();
    let neighbours = neighbours
        .iter()
        .filter(
            |(dir, p)|
                map.contains_isize(*p) &&
                map[(*p).into()] != Tile::Wall
        )
        .collect::<Vec<_>>();

    if neighbours.len() == 1 {
        // the only dead ends are the start and end and we make sure we don't go back to the start
        // so this is the end
        let node = graph.len();
        graph[from].push(Edge{weight: dist+1, to: node});
        graph.push(vec![Edge{weight: dist+1, to: from }]);
        digraph[from].push(Edge{weight: dist+1, to: node});
        digraph.push(vec![]);
    } else if neighbours.len() == 2 {
        // part of previous corridor

        for (dir, neighbour) in neighbours.iter() {
            let next_node = map_to_graph[(*neighbour).into()];
            if next_node.is_none() {
                contract(map, *neighbour, from, dist + 1, forward, graph, digraph, map_to_graph);
            } else {
                continue;
            }
        }
    } else {
        // junction

        let node = graph.len();
        graph.push(vec![Edge{weight: dist+1, to: from }]);
        graph[from].push(Edge{weight: dist+1, to: node});

        let forward = forward.expect("found junction without determining a direction");
        if forward {
            digraph.push(vec![]);
            digraph[from].push(Edge{weight: dist+1, to: node});
        } else {
            digraph.push(vec![Edge{weight: dist+1, to: from }]);
            // digraph[from].push(Edge{weight: dist+1, to: node});
        }

        for (dir, neighbour) in neighbours.iter() {
            if let Tile::Ice(tile_dir) = map[(*neighbour).into()] {
                let forward = tile_dir == **dir;
                // println!("{} {}", neighbour, forward);
                contract(map, *neighbour, node, 0, Some(forward), graph, digraph, map_to_graph)
            } else {
                panic!("bad direction at {neighbour}: {:?}", map[(*neighbour).into()]);
            }
        }
    }
}

fn update_connection(from: usize, dist: usize, forward: bool, graph: &mut Graph, digraph: &mut Graph, next_node: usize) {
    if let Some(existing_edge) = graph[next_node]
        .iter_mut()
        .find(|edge| edge.to == from)
    {
        // println!("    updating existing weight {} to {}", existing_edge.weight, dist + 1);
        existing_edge.weight = max(existing_edge.weight, dist + 1);
        graph[from]
            .iter_mut()
            .find(|edge| edge.to == next_node)
            .unwrap()
            .weight = max(existing_edge.weight, dist + 1);
    } else {
        // println!("    adding edge with weight {}", dist + 1);
        // although we have visited this position already, we haven't drawn a connection to
        // the "from" node
        graph[next_node].push(Edge { weight: dist + 1, to: from });
        graph[from].push(Edge { weight: dist + 1, to: next_node });
    }

    if forward {
        if digraph[from]
            .iter_mut()
            .find(|edge| edge.to == next_node).is_none()
        {
            // although we have visited this position already, we haven't drawn a connection to
            // the "from" node
            // println!("    adding edge {}--{} with weight {}", from, next_node, dist + 1);
            digraph[from].push(Edge { weight: dist + 1, to: next_node });
        }
    } else {
        if digraph[next_node]
            .iter_mut()
            .find(|edge| edge.to == from).is_none()
        {
            // println!("    adding edge {}--{} with weight {}", next_node, from, dist + 1);
            digraph[next_node].push(Edge { weight: dist + 1, to: from });
        }
    }
}

fn longest_path(graph: &Graph, cur: usize, target: usize, mut hist: BitSet, dist: usize) -> usize {
    if hist.contains(cur) {
        return 0
    }
    hist.insert(cur);
    if cur == target {
        return dist;
    }

    graph[cur].iter().map(|edge| {
        longest_path(graph, edge.to, target, hist.clone(), dist + edge.weight)
    }).max().unwrap_or(0)
}


fn main() {
    let input = read_input_lines().unwrap();
    let map = Grid::map_from_lines(
        input.map(|line| line.as_bytes().to_owned()), Tile::parse
    );

    let start = Pt(1_usize, 0);
    // let target = Pt(map.width - 2, map.height - 1);
    // let hist = PointSet::new(map.width);

    // println!("{}", part1(&map, start, target.into(), hist, 0));

    let mut graph = vec![];
    graph.push(vec![]);
    let mut digraph = vec![];
    digraph.push(vec![]);
    let mut map_to_graph = Grid::new(map.width, map.height);
    map_to_graph[start.into()] = Some((0, 0));
    contract(&map, Pt(1, 1), 0, 1, Some(true), &mut graph, &mut digraph, &mut map_to_graph);
    let target = digraph.iter().find_position(|edges| edges.len() == 0).unwrap().0;
    let hist = BitSet::new();
    // println!("{:#?}", graph);
    // println!("{:#?}", digraph);

    println!("{}", longest_path(&digraph, 0, target, hist, 0) - 1);

    let hist = BitSet::new();
    println!("{}", longest_path(&graph, 0, target, hist, 0) - 1);
}