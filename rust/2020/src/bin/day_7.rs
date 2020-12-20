use std::io::{stdin, BufRead};

use petgraph::{
    graph::{Graph, NodeIndex},
    visit::{EdgeRef, IntoEdges, IntoNeighbors},
};

use std::collections::{HashMap, LinkedList};

peg::parser! {
  grammar edge_parser() for str {
    rule number() -> u32
      = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub rule bag() -> Bag
      = adj:$(['a'..='z']+) " " color:$(['a'..='z']+) " " ("bags"/"bag"){ Bag(adj.into(), color.into()) }

    rule bag_link() -> (Bag, u32)
      = number:number() " " bag:bag() ", "? {(bag, number)}

    rule bag_empty() -> BagEdges
      = b1:bag() " contain no other bags." {BagEdges(b1, vec![])}

    rule bag_non_empty() -> BagEdges
      = b1:bag() " contain " l:bag_link()* "." {BagEdges(b1, l)}

    pub rule bag_edge() -> BagEdges
      = edge:(bag_empty()/bag_non_empty()) {edge}
  }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Bag(String, String);

#[derive(Debug, PartialEq)]
pub struct BagEdges(Bag, Vec<(Bag, u32)>);

fn lines_to_graph(lines: Vec<String>) -> (HashMap<Bag, NodeIndex>, Graph<Bag, u32>) {
    let mut bag_graph = Graph::<Bag, u32>::new();
    let mut bag_hashmap: HashMap<Bag, NodeIndex> = HashMap::new();
    let _ = lines
        .into_iter()
        // Pour chaques lignes, on parse
        .map(|line| edge_parser::bag_edge(line.as_str()).unwrap())
        // On met en mémoire le noeud
        .map(|edges| {
            let node_index: NodeIndex = bag_graph.add_node(edges.0.clone());
            bag_hashmap.insert(edges.0, node_index);
            (node_index, edges.1)
        })
        // On le fait pour tous les noeuds avant d'aller plus loin
        .collect::<LinkedList<_>>()
        .iter()
        // On rajoute les fils
        .map(|(node_index, edges)| {
            let _ = edges
                .iter()
                .map(|(bag, weight)| {
                    let son_index: &NodeIndex = bag_hashmap.get(&bag).unwrap();
                    bag_graph.add_edge(*node_index, *son_index, *weight);
                })
                .collect::<Vec<_>>();
        })
        // On collecte le tout
        .collect::<Vec<_>>();
    (bag_hashmap, bag_graph)
}

fn exercise_1(target_node: NodeIndex, bag_graph: &Graph<Bag, u32>) -> i32 {
    use petgraph::visit::DfsEvent;

    let reversed_graph = petgraph::visit::Reversed(bag_graph);
    let mut count = -1;
    petgraph::visit::depth_first_search(&reversed_graph, Some(target_node), |event| {
        if let DfsEvent::Discover(_, _) = event {
            count += 1;
        }
    });
    count
}
fn exercise_2(begin_node: NodeIndex, bag_graph: &Graph<Bag, u32>) -> u32 {
    let mut number_bags: HashMap<NodeIndex, u32> = HashMap::new();

    let mut dfs_postorder = petgraph::visit::DfsPostOrder::new(bag_graph, begin_node);

    while let Some(current_index) = dfs_postorder.next(&bag_graph) {
        let mut current_number_bags: u32 = 1;
        for edge in bag_graph.edges(current_index) {
            let number_bags_neight = number_bags.get(&edge.target()).unwrap();
            current_number_bags += edge.weight() * number_bags_neight;
        }
        number_bags.insert(current_index, current_number_bags);
    }

    *number_bags.get(&begin_node).unwrap() - 1
}

fn main() {
    let (bag_hashmap, bag_graph) =
        lines_to_graph(stdin().lock().lines().map(Result::unwrap).collect());
    // On le fait pour tous les noeuds avant d'aller plus loin
    // On collecte le tout

    let target_node = bag_hashmap
        .get(&Bag("shiny".into(), "gold".into()))
        .unwrap();

    println!("{}", exercise_1(*target_node, &bag_graph));
    println!("{}", exercise_2(*target_node, &bag_graph));
}

#[test]
fn test_exo_1() {
    let (bag_hashmap, bag_graph) = lines_to_graph(vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".into(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".into(),
        "bright white bags contain 1 shiny gold bag.".into(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".into(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".into(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".into(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".into(),
        "faded blue bags contain no other bags.".into(),
        "dotted black bags contain no other bags.".into(),
    ]);
    let target_node = bag_hashmap
        .get(&Bag("shiny".into(), "gold".into()))
        .unwrap();
    assert_eq!(exercise_1(*target_node, &bag_graph), 4);
}

#[test]
fn test_exo_2() {
    let (bag_hashmap, bag_graph) = lines_to_graph(vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".into(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".into(),
        "bright white bags contain 1 shiny gold bag.".into(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".into(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".into(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".into(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".into(),
        "faded blue bags contain no other bags.".into(),
        "dotted black bags contain no other bags.".into(),
    ]);
    let target_node = bag_hashmap
        .get(&Bag("shiny".into(), "gold".into()))
        .unwrap();

    assert_eq!(exercise_2(*target_node, &bag_graph), 32);

    let (bag_hashmap, bag_graph) = lines_to_graph(vec![
        "shiny gold bags contain 2 dark red bags.".into(),
        "dark red bags contain 2 dark orange bags.".into(),
        "dark orange bags contain 2 dark yellow bags.".into(),
        "dark yellow bags contain 2 dark green bags.".into(),
        "dark green bags contain 2 dark blue bags.".into(),
        "dark blue bags contain 2 dark violet bags.".into(),
        "dark violet bags contain no other bags.".into(),
    ]);
    let target_node = bag_hashmap
        .get(&Bag("shiny".into(), "gold".into()))
        .unwrap();
    assert_eq!(exercise_2(*target_node, &bag_graph), 126);
}

#[test]
fn test_bag_reader() {
    assert_eq!(
        edge_parser::bag("light orange bag").unwrap(),
        Bag("light".into(), "orange".into())
    );
    assert_eq!(
        edge_parser::bag("light orange bags").unwrap(),
        Bag("light".into(), "orange".into())
    );
}

#[test]
fn test_edge_reader() {
    assert_eq!(
        edge_parser::bag_edge(
            "dim chartreuse bags contain 2 wavy plum bags, 4 wavy teal bags, 1 dark crimson bag."
        )
        .unwrap(),
        BagEdges(
            Bag("dim".into(), "chartreuse".into()),
            vec![
                (Bag("wavy".into(), "plum".into()), 2),
                (Bag("wavy".into(), "teal".into()), 4),
                (Bag("dark".into(), "crimson".into()), 1),
            ]
        )
    );
    assert_eq!(
        edge_parser::bag_edge("dim chartreuse bags contain 2 wavy plum bags.").unwrap(),
        BagEdges(
            Bag("dim".into(), "chartreuse".into()),
            vec![(Bag("wavy".into(), "plum".into()), 2),]
        )
    );
    assert_eq!(
        edge_parser::bag_edge("bright fuchsia bags contain no other bags.").unwrap(),
        BagEdges(Bag("bright".into(), "fuchsia".into()), vec![])
    );
}
