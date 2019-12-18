use crate::computer::Computer;
use crate::infra::Problem;
use crate::utils::{Dir, Point2d};
use petgraph::algo::dijkstra;
use petgraph::graphmap::UnGraphMap;

pub struct Day15;

type Pos = Point2d<i16>;

impl Into<i16> for Dir {
    fn into(self) -> i16 {
        match self {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

impl Problem<String, String, usize, usize> for Day15 {
    fn day() -> u8 {
        15
    }
    fn first(contents: String) -> usize {
        let (target, graph) = map_ship(&contents);
        let costs = dijkstra(&graph, Pos::new(0, 0), Some(target), |_| 1);
        costs[&target]
    }
    fn second(contents: String) -> usize {
        let (target, graph) = map_ship(&contents);
        let costs = dijkstra(&graph, target, None, |_| 1);
        costs.into_iter().map(|(_, v)| v).max().unwrap()
    }
}

fn map_ship(contents: &str) -> (Pos, UnGraphMap<Pos, ()>) {
    let mut cpu = Computer::from_str(contents);
    let mut p = Pos::new(0, 0);
    let mut d = Dir::North;
    let mut target = None;
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
                d = d.ccw();
            }
            Some(1) => {
                graph.add_edge(p, p + d, ());
                p = p + d;
                d = d.cw();
            }
            Some(2) => {
                graph.add_edge(p, p + d, ());
                p = p + d;
                target = Some(p);
                d = d.cw();
            }
            _ => panic!(),
        };
        if p.x == 0 && p.y == 0 && graph.edge_count() != 0 {
            break;
        }
    }
    (target.unwrap(), graph)
}
