use std::cmp::max;
use bit_set::BitSet;
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

fn dfs(map: &Grid<Tile>, pos: Pt<isize>, target: Pt<isize>, mut hist: PointSet<usize>, dist: usize) -> usize {
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
            Tile::Floor | Tile::Ice(_) => { Some(dfs(&map, next, target, hist.clone(), dist+1)) }
            // Tile::Ice(next_dir) => {
            //     if *dir == next_dir {
            //         Some(dfs(&map, next, target,  hist.clone(), dist+1))
            //     } else {
            //         None
            //     }
            // }
        }
    }).max().unwrap_or(0)
}

#[derive(Clone, Debug)]
struct Edge {
    weight: usize,
    pos: Pt<usize>,
    to: usize,
}

type Graph = Vec<Vec<Edge>>;

fn contract(
    map: &Grid<Tile>,
    pos: Pt<usize>,
    from: usize,
    dist: usize,
    graph: &mut Graph,
    // pt --> (node index, dist from node)
    map_to_graph: &mut Grid<Option<(usize, usize)>>,
) {
    if let Some((node, d_dist)) = map_to_graph[pos] {
        println!("arrived at {pos} from {from} and it belongs to {node}");
        if node == from {
            println!("    cycle");
            return;
        }
        update_connection(pos, from, d_dist +dist+1, graph, node);
        return;
    } else {
        map_to_graph[pos] = Some((from, dist));
    }
    let neighbours = pos
        .neighbours4();
    let neighbours = neighbours
        .iter()
        .filter(
            |&&p|
                map.contains(p) &&
                map[p] != Tile::Wall // &&
                // !visited.contains(**p)
        )
        .collect::<Vec<_>>();

    if neighbours.len() == 1 {
        // the only dead ends are the start and end and we make sure we don't go back to the start
        // so this is the end
        let node = graph.len();
        graph[from].push(Edge{weight: dist+1, pos, to: node});
        graph.push(vec![Edge{weight: dist+1, pos, to: from }]);
    } else if neighbours.len() == 2 {
        // part of previous corridor

        for &&neighbour in neighbours.iter() {
            let next_node = map_to_graph[neighbour];
            if next_node.is_none() {
                contract(map, neighbour, from, dist + 1, graph, map_to_graph);
            } else {
                continue;
            }
        }
    } else {
        // junction

        let node = graph.len();
        graph.push(vec![Edge{weight: dist+1, pos, to: from }]);
        graph[from].push(Edge{weight: dist+1, pos, to: node});

        for neighbour in neighbours.iter() {
            contract(map, **neighbour, node, 0, graph, map_to_graph)
        }
    }
}

fn update_connection(pos: Pt<usize>, from: usize, dist: usize, graph: &mut Graph, next_node: usize) {
    if let Some(existing_edge) = graph[next_node]
        .iter_mut()
        .find(|edge| edge.to == from)
    {
        println!("    updating existing weight {} to {}", existing_edge.weight, dist + 1);
        existing_edge.weight = max(existing_edge.weight, dist + 1);
        graph[from]
            .iter_mut()
            .find(|edge| edge.to == next_node)
            .unwrap()
            .weight = max(existing_edge.weight, dist + 1);
    } else {
        println!("    adding edge with weight {}", dist + 1);
        // although we have visited this position already, we haven't drawn a connection to
        // the "from" node
        graph[next_node].push(Edge { weight: dist + 1, pos, to: from });
        graph[from].push(Edge { weight: dist + 1, pos, to: next_node });
    }
}

fn dfs2(graph: &Graph, cur: usize, target: usize, mut hist: BitSet, dist: usize) -> usize {
    if hist.contains(cur) {
        return 0
    }
    hist.insert(cur);
    if cur == target {
        return dist;
    }

    graph[cur].iter().map(|edge| {
        dfs2(graph, edge.to, target, hist.clone(), dist + edge.weight)
    }).max().unwrap_or(0)
}


fn main() {
    let input = read_input_lines().unwrap();
    let map = Grid::map_from_lines(
        input.map(|line| line.as_bytes().to_owned()), Tile::parse
    );
    let start = Pt(1, 0);
    let target = Pt(map.width - 2, map.height - 1);
    let mut graph = vec![];
    graph.push(vec![]);
    let mut map_to_graph = Grid::new(map.width, map.height);
    map_to_graph[start] = Some((0, 0));
    contract(&map, Pt(1, 1), 0, 1, &mut graph, &mut map_to_graph);
    // let hist = PointSet::new(map.width);
    // println!("{}", dfs(&map, start, target.into(), hist, 0))
    let hist = BitSet::new();
    // println!("{:#?}", graph);
    println!("{}", dfs2(&graph, 0, graph.len() - 1, hist, 0) - 1);
}