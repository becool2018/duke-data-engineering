use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}

/*
  THis is a bit like the following Python code:
  class Fighter:
      def __init__(self, name):
          self.name = name
*/
impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}
fn main() {
    let mut graph = UnGraph::new_undirected();
    let fighters = [
        Fighter::new("Dust Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1);
    add_edge(&mut graph, &fighter_nodes, 1, 3);
    add_edge(&mut graph, &fighter_nodes, 3, 0);
    add_edge(&mut graph, &fighter_nodes, 3, 2);
    add_edge(&mut graph, &fighter_nodes, 3, 4);
    add_edge(&mut graph, &fighter_nodes, 0, 4);
    add_edge(&mut graph, &fighter_nodes, 2, 4);

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name: &String = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        match name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the graph",
                name
            ),
            "Dust Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights than Conor McGregor",
                name, closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has a highiest centrality of {:.2}, as they have fought with the least number of fighters",
                name, closeness
            ),
            _ => {}
        }
        println!("-----------------------");
    }
}
