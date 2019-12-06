use petgraph::algo::{dijkstra};
use petgraph::graphmap::UnGraphMap;

pub fn first(contents: &String) -> i32 {
    let gr = UnGraphMap::<_, f32>::from_edges(
        contents.lines()
            .map(|s| s.trim())
            .map(|s| s.split(')').collect::<Vec<_>>())
            .map(|x| (x[0], x[1])),
    );
    dijkstra(&gr, "COM", None, |_| 1).values().sum::<i32>()
}

pub fn second(contents: &String) -> i32 {
    let gr = UnGraphMap::<_, f32>::from_edges(
        contents.lines()
            .map(|s| s.trim())
            .map(|s| s.split(')').collect::<Vec<_>>())
            .map(|x| (x[0], x[1])),
    );
    dijkstra(&gr, "YOU", Some("SAN"), |_| 1)["SAN"] - 2
}