use crate::computer::Computer;
use crate::infra::Problem;
use petgraph::algo::dijkstra;
use petgraph::graphmap::UnGraphMap;

pub struct Day15;

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Problem<String, String, i64, i64> for Day15 {
    fn day() -> u8 {
        15
    }
    fn first(contents: String) -> i64 {
        let (target, graph) = map_ship(&contents);
        let costs = dijkstra(&graph, (0, 0), Some(target), |_| 1);
        costs[&target]
    }
    fn second(contents: String) -> i64 {
        let (target, graph) = map_ship(&contents);
        let costs = dijkstra(&graph, target, None, |_| 1);
        costs.into_iter().map(|(_, v)| v).max().unwrap()
    }
}

fn map_ship(contents: &str) -> ((i64, i64), UnGraphMap<(i64, i64), ()>) {
    let mut cpu = Computer::from_str(contents);
    let mut p = (0, 0);
    let mut d = Dir::North;
    let mut target = (0, 0);
    let mut graph = UnGraphMap::new();

    cpu.run();
    loop {
        cpu.run_with_input(match d {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        });
        match (&mut cpu).next() {
            Some(0) => {
                d = ccw(d);
            }
            Some(1) => {
                graph.add_edge(p, next(p, d), ());
                p = next(p, d);
                d = cw(d);
            }
            Some(2) => {
                graph.add_edge(p, next(p, d), ());
                p = next(p, d);
                target = p;
                d = cw(d);
            }
            _ => panic!(),
        };
        if p == (0, 0) && graph.edge_count() != 0 {
            break;
        }
    }
    (target, graph)
}

fn ccw(d: Dir) -> Dir {
    match d {
        Dir::North => Dir::West,
        Dir::West => Dir::South,
        Dir::South => Dir::East,
        Dir::East => Dir::North,
    }
}

fn cw(d: Dir) -> Dir {
    match d {
        Dir::North => Dir::East,
        Dir::East => Dir::South,
        Dir::South => Dir::West,
        Dir::West => Dir::North,
    }
}

fn next(p: (i64, i64), d: Dir) -> (i64, i64) {
    match d {
        Dir::North => (p.0, p.1 - 1),
        Dir::South => (p.0, p.1 + 1),
        Dir::West => (p.0 - 1, p.1),
        Dir::East => (p.0 + 1, p.1),
    }
}
