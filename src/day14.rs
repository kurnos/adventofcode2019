use crate::infra::Problem;
use std::collections::HashMap;

use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;

pub struct Day14;

impl Problem<String, String, u64, u64> for Day14 {
    fn day() -> u8 {
        14
    }
    fn first(contents: String) -> u64 {
        let (order, costs) = parse_reactions(&contents);
        required_ore(&costs, &order, 1)
    }
    fn second(contents: String) -> u64 {
        let (order, costs) = parse_reactions(&contents);

        crate::utils::bisect(
            |n| required_ore(&costs, &order, n),
            1,
            1_000_000_000_000u64,
            1_000_000_000_000u64,
        )
    }
}

#[allow(clippy::type_complexity)]
fn parse_reactions(contents: &str) -> (Vec<&str>, HashMap<&str, (u64, Vec<(u64, &str)>)>) {
    let mut costs = HashMap::new();
    let mut graph = DiGraphMap::<&str, ()>::new();

    for line in contents.lines() {
        let mut s = line.split("=>");
        let reagents = s.next().unwrap().split(',').map(parse_reaction);
        let (prod_amount, prod_kind) = parse_reaction(s.next().unwrap());
        for (req_amount, req_kind) in reagents {
            costs
                .entry(prod_kind)
                .or_insert_with(|| (prod_amount, Vec::new()))
                .1
                .push((req_amount, req_kind));
            graph.add_edge(req_kind, prod_kind, ());
        }
    }

    (toposort(&graph, None).unwrap().into_iter().collect(), costs)
}

fn parse_reaction(r: &str) -> (u64, &str) {
    let mut x = r.trim().split_whitespace();
    let n = x.next().unwrap().parse().unwrap();
    (n, x.next().unwrap())
}

#[allow(clippy::type_complexity)]
fn required_ore<'a, 'b>(
    costs: &'a HashMap<&'b str, (u64, Vec<(u64, &'b str)>)>,
    production_order: &'a [&'b str],
    fuel_amount: u64,
) -> u64 {
    let mut needed = HashMap::new();
    needed.insert("FUEL", fuel_amount);

    for kind in production_order.iter().skip(1).rev() {
        let required = needed.remove(kind).unwrap();
        let (produced, sources) = &costs[kind];
        let productions = ((required as f64) / (*produced as f64)).ceil() as u64;
        for (a, s) in sources {
            *needed.entry(s).or_default() += a * productions;
        }
    }
    needed["ORE"]
}
