/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        //创建一个空的队列来存储待访问的节点。
        //将起始节点加入队列，并标记为已访问。
        //从队列中取出一个节点，将其加入访问顺序，并将其未访问的邻居节点加入队列。
        //重复步骤 3，直到队列为空。
		//TODO

        let mut visited = vec![false; self.adj.len()]; // 标记节点是否已访问
        let mut visit_order = Vec::new(); // 存储访问顺序
        let mut queue = VecDeque::new(); // 队列用于BFS

        visited[start] = true; // 将起始节点标记为已访问
        queue.push_back(start); // 将起始节点加入队列

        while let Some(node) = queue.pop_front() { // 循环直到队列为空
            visit_order.push(node); // 将当前节点加入访问顺序

            // 遍历当前节点的邻居节点
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] { // 如果邻居节点未访问过
                    visited[neighbor] = true; // 标记为已访问
                    queue.push_back(neighbor); // 加入队列等待访问
                }
            }
        }

        visit_order
    
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

