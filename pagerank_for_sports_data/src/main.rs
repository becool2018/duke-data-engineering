use textwrap::fill;

struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        let n = graph.len();
        let mut ranks = vec![1.0 / n as f64; n];

        for _ in 0..self.iterations {
            // A new vector to hold the updated ranks after this iteration
            let mut new_ranks = vec![0.0; n];

            // Iterate over each node and its edges in the graph.
            for (node, edges) in graph.iter().enumerate() {
                let contribution = ranks[node] / edges.len() as f64;

                // Distribute the rank contribution to each connected node
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            for rank in &mut new_ranks {
                *rank = self.damping * *rank + (1.0 - self.damping) / n as f64;
            }
            ranks = new_ranks;
        }

        ranks
    }
}

fn main() {
    let graph = vec![
        vec![1, 2], // Node 0 is connected to nodes 1 and 2
        vec![0],    // Node 1 is connected to node 0
        vec![0, 3], // Node 2 is connected to nodes 0 and 3
        vec![0],    // Node 3 is connected to node 0
        vec![0, 1],
    ];

    let names = vec!["ESPN", "NFL", "NBA", "UFC", "MLB"];

    let pagerank = PageRank::new(0.85, 100);
    let ranks = pagerank.rank(&graph);
    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {:.4}", names[i], rank);
    }

    let explanation = "PageRank is a link analysis algorithm used by Google Search that uses the structure of the web to determine the importance of individual pages. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is. The underlying assumption is that more important websites are likely to receive more links from other websites.";
    println!("\n{}", fill(explanation, 80));
}
