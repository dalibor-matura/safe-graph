//! Directed Graph representation.
//!
//! The Graph and its components are inspired and mostly copied and refactored from `petgraph` crate
//! https://crates.io/crates/petgraph.

pub use crate::node::NodeTrait;

use crate::edge::{AllEdges, CompactDirection, Direction, EdgeType, Edges, IntoWeightedEdge};
use crate::node::Nodes;
use crate::traverse::{Neighbors, NeighborsDirected};
use indexmap::IndexMap;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;

/// Marker type for a directed graph.
#[derive(Copy, Debug)]
pub enum Directed {}
copyclone!(Directed);

/// Marker type for an undirected graph.
#[derive(Copy, Debug)]
pub enum Undirected {}
copyclone!(Undirected);

/// A `Graph` with undirected edges.
///
/// For example, an edge between *1* and *2* is equivalent to an edge between
/// *2* and *1*.
pub type UndirectedGraph<N, E> = Graph<N, E, Undirected>;

/// `Graph<N, E, Ty>` is a graph datastructure using an associative array
/// of its node weights `N`.
///
/// It uses an combined adjacency list and sparse adjacency matrix
/// representation, using **O(|V| + |E|)** space, and allows testing for edge
/// existance in constant time.
///
/// # `Graph` is parameterized over:
///
/// - Associated data `N` for nodes and `E` for edges, called *weights*.
/// - The node weight `N` must implement `Copy` and will be used as node
/// identifier, duplicated into several places in the data structure.
/// It must be suitable as a hash table key (implementing `Eq + Hash`).
/// The node type must also implement `Ord` so that the implementation can
/// order the pair (`a`, `b`) for an edge connecting any two nodes `a` and `b`.
/// - `E` can be of arbitrary type.
/// - Edge type `Ty` that determines whether the graph edges are directed or
/// undirected.
///
/// You can use the type alias `UndirectedGraph` for convenience.
///
/// `Graph` does not allow parallel edges, but self loops are allowed.
#[derive(Clone)]
pub struct Graph<N, E, Ty = Directed> {
    nodes: IndexMap<N, Vec<(N, CompactDirection)>>,
    edges: IndexMap<(N, N), E>,
    ty: PhantomData<Ty>,
}

impl<N: Eq + Hash + fmt::Debug, E: fmt::Debug, Ty: EdgeType> fmt::Debug for Graph<N, E, Ty> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.nodes.fmt(f)
    }
}

impl<N, E, Ty> Graph<N, E, Ty>
where
    N: NodeTrait,
    Ty: EdgeType,
{
    /// Create a new `Graph` instance.
    ///
    /// # Examples
    /// ```
    /// use safe_graph::Graph;
    ///
    /// let graph: Graph<i32, f32> = Graph::new();
    ///
    /// // Test nodes and edges count immediately after graph creation.
    /// assert_eq!(graph.node_count(), 0);
    /// assert_eq!(graph.edge_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `Graph` with estimated capacity.
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Self {
            nodes: IndexMap::with_capacity(nodes),
            edges: IndexMap::with_capacity(edges),
            ty: PhantomData,
        }
    }

    /// Return the current node and edge capacity of the graph.
    pub fn capacity(&self) -> (usize, usize) {
        (self.nodes.capacity(), self.edges.capacity())
    }

    /// Use their natural order to map the node pair (a, b) to a canonical edge id.
    #[inline]
    pub fn edge_key(a: N, b: N) -> (N, N) {
        if Ty::is_directed() || a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }

    /// Whether the graph has directed edges.
    pub fn is_directed(&self) -> bool {
        Ty::is_directed()
    }

    /// Create a new `Graph` from an iterable of edges.
    ///
    /// Node values are taken directly from the list.
    /// Edge weights `E` may either be specified in the list,
    /// or they are filled with default values.
    ///
    /// Nodes are inserted automatically to match the edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use safe_graph::Graph;
    ///
    /// // Create a new directed Graph.
    /// // Use a type hint to have `()` be the edge weight type.
    /// let gr = Graph::<_, ()>::from_edges(&[
    ///     (0, 1), (0, 2), (0, 3),
    ///     (1, 2), (1, 3),
    ///     (2, 3),
    /// ]);
    /// ```
    pub fn from_edges<I>(iterable: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoWeightedEdge<E, NodeId = N>,
    {
        Self::from_iter(iterable)
    }

    /// Return the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Remove all nodes and edges
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    /// Add node `n` to the graph.
    pub fn add_node(&mut self, n: N) -> N {
        self.nodes.entry(n).or_insert(Vec::new());
        n
    }

    /// Return `true` if the node is contained in the graph.
    pub fn contains_node(&self, n: N) -> bool {
        self.nodes.contains_key(&n)
    }

    /// Add an edge connecting `a` and `b` to the graph, with associated
    /// data `weight`. For a directed graph, the edge is directed from `a`
    /// to `b`.
    ///
    /// Inserts nodes `a` and/or `b` if they aren't already part of the graph.
    ///
    /// Return `None` if the edge did not previously exist, otherwise,
    /// the associated data is updated and the old value is returned
    /// as `Some(old_weight)`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create a Graph with directed edges, and add one edge to it
    /// use safe_graph::Graph;
    ///
    /// let mut g: Graph<_, _> = Graph::new();
    /// g.add_edge("x", "y", -1);
    /// assert_eq!(g.node_count(), 2);
    /// assert_eq!(g.edge_count(), 1);
    /// assert!(g.contains_edge("x", "y"));
    /// assert!(!g.contains_edge("y", "x"));
    /// ```
    pub fn add_edge(&mut self, a: N, b: N, weight: E) -> Option<E> {
        if let old @ Some(_) = self.edges.insert(Self::edge_key(a, b), weight) {
            old
        } else {
            // Insert in the adjacency list if it's a new edge.
            self.nodes
                .entry(a)
                .or_insert_with(|| Vec::with_capacity(1))
                .push((b, CompactDirection::Outgoing));

            // Self loops don't have the Incoming entry.
            if a != b {
                self.nodes
                    .entry(b)
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push((a, CompactDirection::Incoming));
            }

            None
        }
    }

    /// Return `true` if the edge connecting `a` with `b` is contained in the graph.
    pub fn contains_edge(&self, a: N, b: N) -> bool {
        self.edges.contains_key(&Self::edge_key(a, b))
    }

    /// Return an iterator over the nodes of the graph.
    ///
    /// Iterator element type is `N`.
    pub fn nodes(&self) -> Nodes<N> {
        Nodes::new(self.nodes.keys().cloned())
    }

    /// Return an iterator of all nodes with an edge starting from `a`.
    ///
    /// - `Directed`: Outgoing edges from `a`.
    /// - `Undirected`: All edges from or to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `N`.
    pub fn neighbors(&self, a: N) -> Neighbors<N, Ty> {
        let iter = match self.nodes.get(&a) {
            Some(neigh) => neigh.iter(),
            None => [].iter(),
        };

        Neighbors::new(iter, self.ty)
    }

    /// Return an iterator of all neighbors that have an edge between them and
    /// `a`, in the specified direction.
    /// If the graph's edges are undirected, this is equivalent to *.neighbors(a)*.
    ///
    /// - `Directed`, `Outgoing`: All edges from `a`.
    /// - `Directed`, `Incoming`: All edges to `a`.
    /// - `Undirected`: All edges from or to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `N`.
    pub fn neighbors_directed(&self, a: N, dir: Direction) -> NeighborsDirected<N, Ty> {
        let iter = match self.nodes.get(&a) {
            Some(neigh) => neigh.iter(),
            None => [].iter(),
        };

        NeighborsDirected::new(iter, dir, self.ty)
    }

    /// Return an iterator of target nodes with an edge starting from `a`,
    /// paired with their respective edge weights.
    ///
    /// - `Directed`: Outgoing edges from `a`.
    /// - `Undirected`: All edges from or to `a`.
    ///
    /// Produces an empty iterator if the node doesn't exist.<br>
    /// Iterator element type is `(N, &E)`.
    pub fn edges(&self, from: N) -> Edges<N, E, Ty> {
        Edges::new(from, &self.edges, self.neighbors(from))
    }

    /// Return a reference to the edge weight connecting `a` with `b`, or
    /// `None` if the edge does not exist in the graph.
    pub fn edge_weight(&self, a: N, b: N) -> Option<&E> {
        self.edges.get(&Self::edge_key(a, b))
    }

    /// Return a mutable reference to the edge weight connecting `a` with `b`, or
    /// `None` if the edge does not exist in the graph.
    pub fn edge_weight_mut(&mut self, a: N, b: N) -> Option<&mut E> {
        self.edges.get_mut(&Self::edge_key(a, b))
    }

    /// Return an iterator over all edges of the graph with their weight in arbitrary order.
    ///
    /// Iterator element type is `(N, N, &E)`
    pub fn all_edges(&self) -> AllEdges<N, E, Ty> {
        AllEdges::new(self.edges.iter(), self.ty)
    }
}

/// Create a new empty `Graph`.
impl<N, E, Ty> Default for Graph<N, E, Ty>
where
    N: NodeTrait,
    Ty: EdgeType,
{
    fn default() -> Self {
        Graph::with_capacity(0, 0)
    }
}

/// Create a new `Graph` from an iterable of edges.
impl<N, E, Ty, Item> FromIterator<Item> for Graph<N, E, Ty>
where
    Item: IntoWeightedEdge<E, NodeId = N>,
    N: NodeTrait,
    Ty: EdgeType,
{
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = Item>,
    {
        let iter = iterable.into_iter();
        let (low, _) = iter.size_hint();
        let mut g = Self::with_capacity(0, low);
        g.extend(iter);
        g
    }
}

/// Extend the graph from an iterable of edges.
///
/// Nodes are inserted automatically to match the edges.
impl<N, E, Ty, Item> Extend<Item> for Graph<N, E, Ty>
where
    Item: IntoWeightedEdge<E, NodeId = N>,
    N: NodeTrait,
    Ty: EdgeType,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = Item>,
    {
        let iter = iterable.into_iter();
        let (low, _) = iter.size_hint();
        self.edges.reserve(low);

        for elt in iter {
            let (source, target, weight) = elt.into_weighted_edge();
            self.add_edge(source, target, weight);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use crate::graph::{Directed, Undirected};

    #[test]
    fn new() {
        let graph: Graph<&str, f32> = Graph::new();

        // Test nodes and edges count immediately after graph creation.
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn new_with_tuple_as_node() {
        let graph: Graph<(&str, &str), f32> = Graph::new();

        // Test nodes and edges count immediately after graph creation.
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn with_capacity() {
        let graph: Graph<&str, f32> = Graph::with_capacity(4, 6);

        // Test nodes and edges count immediately after graph creation.
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn capacity() {
        let nodes_capacity = 4;
        let edges_capacity = 8;

        let graph: Graph<&str, f32> = Graph::with_capacity(nodes_capacity, edges_capacity);
        // Get the allocated capacities.
        let (n, e) = graph.capacity();
        // Test nodes allocated capacity.
        assert!(
            n >= nodes_capacity,
            "Allocated nodes capacity `{}` must be equal or bigger then requested capacity `{}`.",
            n,
            nodes_capacity
        );
        // Test edges allocated capacity.
        assert!(
            e >= edges_capacity,
            "Allocated edges capacity `{}` must be equal or bigger then requested capacity `{}`.",
            e,
            edges_capacity
        );
    }

    #[test]
    fn edge_key() {
        // Test for Directed Graph.
        assert_eq!(Graph::<&str, f32, Directed>::edge_key("a", "b"), ("a", "b"));
        assert_eq!(Graph::<&str, f32, Directed>::edge_key("b", "a"), ("b", "a"));

        // Test for Undirected Graph.
        assert_eq!(
            Graph::<&str, f32, Undirected>::edge_key("a", "b"),
            ("a", "b")
        );
        assert_eq!(
            Graph::<&str, f32, Undirected>::edge_key("b", "a"),
            ("a", "b")
        );
    }

    #[test]
    fn is_directed_true() {
        let graph: Graph<&str, f32, Directed> = Graph::new();

        assert_eq!(graph.is_directed(), true)
    }

    #[test]
    fn is_directed_false() {
        let graph: Graph<&str, f32, Undirected> = Graph::new();

        assert_eq!(graph.is_directed(), false)
    }

    #[test]
    fn from_edges() {
        // Create a new directed Graph.
        // Use a type hint to have `()` be the edge weight type.
        let graph = Graph::<_, _>::from_edges(&[
            (0, 1, 0.12),
            (0, 2, 0.99),
            (0, 3, 0.1),
            (1, 2, 0.9),
            (1, 3, 0.44),
            (2, 3, 0.8),
        ]);

        // Test nodes and edges count.
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 6);

        // Test edges weights.
        assert_eq!(graph.edge_weight(0, 1), Some(&0.12));
        assert_eq!(graph.edge_weight(2, 3), Some(&0.8));
    }

    #[test]
    fn node_count() {
        let mut graph: Graph<&str, f32> = Graph::new();

        // Test nodes count immediately after graph creation.
        assert_eq!(graph.node_count(), 0);

        graph.add_node("a");
        graph.add_node("b");

        // Test nodes count.
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn edge_count() {
        let mut graph: Graph<&str, f32> = Graph::new();

        // Test edges count immediately after graph creation.
        assert_eq!(graph.edge_count(), 0);

        graph.add_edge("a", "b", 2.3);
        graph.add_edge("b", "c", 4.1);

        // Test nodes count.
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn clear() {
        let mut graph: Graph<&str, f32> = Graph::new();

        // Add one edge.
        graph.add_edge("a", "b", 2.3);

        // Test nodes and edges count.
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);

        graph.clear();

        // Test nodes and edges count.
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn add_node() {
        let mut graph: Graph<&str, f32> = Graph::new();

        // Add one node.
        graph.add_node("a");

        // Test nodes count .
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn add_node_as_tuple() {
        let mut graph: Graph<(&str, &str), f32> = Graph::new();

        // Add one node.
        graph.add_node(("s", "a"));

        // Test nodes count.
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn add_node_as_tuple_twide() {
        let mut graph: Graph<(&str, &str), f32> = Graph::new();

        // Add one node twice.
        graph.add_node(("s", "a"));
        graph.add_node(("s", "a"));

        // Test nodes count, it should still be one.
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn add_edge() {
        let mut graph: Graph<&str, f32> = Graph::new();

        // Add one edge.
        graph.add_edge("a", "b", 2.3);

        // Test nodes and edges count.
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn add_edge_with_nodes_as_tuples() {
        let mut graph: Graph<(&str, &str), f32> = Graph::new();

        // Add one edge.
        graph.add_edge(("s", "a"), ("r", "b"), 2.3);

        // Test nodes and edges count.
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn edge_weight() {
        let mut graph: Graph<&str, f32> = Graph::new();

        // Add one edge.
        let edge_weight = 2.4;
        graph.add_edge("a", "b", edge_weight);

        // Test edge weight.
        assert_eq!(graph.edge_weight("a", "b"), Some(&edge_weight));
    }

    #[test]
    fn edge_weight_with_nodes_as_tuples() {
        let mut graph: Graph<(&str, &str), f32> = Graph::new();

        // Add one edge twice.
        let edge_weight = 2.4;
        graph.add_edge(("s", "a"), ("r", "a"), 8.0);
        graph.add_edge(("s", "a"), ("r", "a"), edge_weight);

        // Test edge weight.
        assert_eq!(
            graph.edge_weight(("s", "a"), ("r", "a")),
            Some(&edge_weight)
        );
    }

    #[test]
    fn nodes() {
        let mut graph: Graph<&str, f32> = Graph::new();

        // Prepare a list of node indexes to test with.
        let list = ["a", "b", "c", "d"];

        // Add items from the list as nodes.
        for index in list.iter() {
            graph.add_node(*index);
        }

        // Test iteration over nodes.
        for (i, node) in graph.nodes().enumerate() {
            assert_eq!(list[i], node);
        }
    }

    #[test]
    fn check_nodes_and_edges() {
        let mut graph: Graph<&str, f32> = Graph::with_capacity(4, 6);
        graph.add_edge("a", "b", 2.0);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.contains_edge("a", "b"));
        assert!(!graph.contains_edge("b", "a"));

        graph.add_edge("a", "c", 1.2);
        graph.add_edge("a", "d", 4.2);
        graph.add_edge("b", "c", 0.2);
        graph.add_edge("b", "d", 3.3);
        graph.add_edge("c", "b", 12.2);

        // Check numbers of nodes and edges.
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 6);

        // Check edges weight.
        assert_eq!(graph.edge_weight("a", "b"), Some(&2.0));
        assert_eq!(graph.edge_weight("a", "c"), Some(&1.2));

        // Update and check edge weight.
        graph.add_edge("a", "b", 4.4);

        assert_eq!(graph.edge_weight("a", "b"), Some(&4.4));

        // Try to get edge weight for non-existing edge.
        let weight = graph.edge_weight("c", "d");

        assert_eq!(weight, None);
    }
}
