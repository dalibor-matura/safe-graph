//! Graph Edge related constructs.

use self::Direction::{Incoming, Outgoing};
use crate::graph::{Directed, Graph, Undirected};
use crate::node::NodeTrait;
use crate::traverse::Neighbors;
use indexmap::map::Iter as IndexMapIter;
use indexmap::IndexMap;
use std::marker::PhantomData;

/// A graph's edge type determines whether is has directed edges or not.
pub trait EdgeType {
    fn is_directed() -> bool;
}

impl EdgeType for Directed {
    #[inline]
    fn is_directed() -> bool {
        true
    }
}

impl EdgeType for Undirected {
    #[inline]
    fn is_directed() -> bool {
        false
    }
}

pub struct Edges<'a, N, E: 'a, Ty>
where
    N: 'a + NodeTrait,
    Ty: EdgeType,
{
    from: N,
    edges: &'a IndexMap<(N, N), E>,
    iter: Neighbors<'a, N, Ty>,
}

impl<'a, N, E, Ty> Edges<'a, N, E, Ty>
where
    N: 'a + NodeTrait,
    Ty: EdgeType,
{
    pub fn new(from: N, edges: &'a IndexMap<(N, N), E>, iter: Neighbors<'a, N, Ty>) -> Self {
        Self { from, edges, iter }
    }
}

impl<'a, N, E, Ty> Iterator for Edges<'a, N, E, Ty>
where
    N: 'a + NodeTrait,
    E: 'a,
    Ty: EdgeType,
{
    type Item = (N, N, &'a E);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(b) => {
                let a = self.from;
                match self.edges.get(&Graph::<N, E, Ty>::edge_key(a, b)) {
                    None => unreachable!(),
                    Some(edge) => Some((a, b, edge)),
                }
            }
        }
    }
}

pub struct AllEdges<'a, N, E: 'a, Ty> {
    inner: IndexMapIter<'a, (N, N), E>,
    ty: PhantomData<Ty>,
}

impl<'a, N, E, Ty> AllEdges<'a, N, E, Ty>
where
    N: 'a + NodeTrait,
{
    pub fn new(inner: IndexMapIter<'a, (N, N), E>, ty: PhantomData<Ty>) -> Self {
        Self { inner, ty }
    }
}

impl<'a, N, E, Ty> Iterator for AllEdges<'a, N, E, Ty>
where
    N: 'a + NodeTrait,
    E: 'a,
    Ty: EdgeType,
{
    type Item = (N, N, &'a E);
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some((&(a, b), v)) => Some((a, b, v)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner
            .nth(n)
            .map(|(&(n1, n2), weight)| (n1, n2, weight))
    }

    fn last(self) -> Option<Self::Item> {
        self.inner
            .last()
            .map(|(&(n1, n2), weight)| (n1, n2, weight))
    }
}

impl<'a, N, E, Ty> DoubleEndedIterator for AllEdges<'a, N, E, Ty>
where
    N: 'a + NodeTrait,
    E: 'a,
    Ty: EdgeType,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner
            .next_back()
            .map(|(&(n1, n2), weight)| (n1, n2, weight))
    }
}

/// Convert an element like `(i, j)` or `(i, j, w)` into
/// a triple of source, target, edge weight.
///
/// For `Graph::from_edges` and `Graph::from_edges`.
pub trait IntoWeightedEdge<E> {
    type NodeId;
    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E);
}

impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix)
where
    E: Default,
{
    type NodeId = Ix;

    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (s, t) = self;
        (s, t, E::default())
    }
}

impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix, E) {
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        self
    }
}

impl<'a, Ix, E> IntoWeightedEdge<E> for (Ix, Ix, &'a E)
where
    E: Clone,
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (a, b, c) = self;
        (a, b, c.clone())
    }
}

impl<'a, Ix, E> IntoWeightedEdge<E> for &'a (Ix, Ix)
where
    Ix: Copy,
    E: Default,
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (s, t) = *self;
        (s, t, E::default())
    }
}

impl<'a, Ix, E> IntoWeightedEdge<E> for &'a (Ix, Ix, E)
where
    Ix: Copy,
    E: Clone,
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        self.clone()
    }
}

/// Edge direction.
#[derive(Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    /// An `Outgoing` edge is an outward edge *from* the current node.
    Outgoing = 0,
    /// An `Incoming` edge is an inbound edge *to* the current node.
    Incoming = 1,
}

copyclone!(Direction);

impl Direction {
    /// Return the opposite `Direction`.
    #[inline]
    pub fn opposite(self) -> Direction {
        match self {
            Outgoing => Incoming,
            Incoming => Outgoing,
        }
    }

    /// Return `0` for `Outgoing` and `1` for `Incoming`.
    #[inline]
    pub fn index(self) -> usize {
        (self as usize) & 0x1
    }
}

// Non-repr(usize) version of Direction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CompactDirection {
    Outgoing,
    Incoming,
}

impl From<Direction> for CompactDirection {
    fn from(d: Direction) -> Self {
        match d {
            Outgoing => CompactDirection::Outgoing,
            Incoming => CompactDirection::Incoming,
        }
    }
}

impl PartialEq<Direction> for CompactDirection {
    fn eq(&self, rhs: &Direction) -> bool {
        (*self as usize) == (*rhs as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::edge::CompactDirection;
    use crate::edge::{EdgeType, Edges};
    use crate::graph::{Directed, Undirected};
    use crate::traverse::Neighbors;
    use indexmap::IndexMap;
    use std::marker::PhantomData;

    #[test]
    fn edge_type_is_directed() {
        assert_eq!(Directed::is_directed(), true);
        assert_eq!(Undirected::is_directed(), false);
    }

    #[test]
    fn edges_new() {
        // Prepare arguments.
        let from: u32 = 1;
        let edges: IndexMap<(u32, u32), f32> = IndexMap::new();
        let node_neighbors: Vec<(u32, CompactDirection)> = vec![];
        let iter = node_neighbors.iter();
        let neighbors: Neighbors<u32, Directed> = Neighbors::new(iter, PhantomData);

        // Test `Edges` struct creation.
        Edges::new(from, &edges, neighbors);
    }

    #[test]
    fn edges_next() {
        // Prepare arguments.
        let from: u32 = 1;
        let mut edges: IndexMap<(u32, u32), f32> = IndexMap::with_capacity(3);
        edges.insert((2, 1), 2.0);
        edges.insert((1, 3), 3.0);
        edges.insert((1, 4), 4.0);
        let node_neighbors: Vec<(u32, CompactDirection)> = vec![
            (2, CompactDirection::Incoming),
            (3, CompactDirection::Outgoing),
            (4, CompactDirection::Outgoing),
        ];
        let neighbors: Neighbors<u32, Directed> =
            Neighbors::new(node_neighbors.iter(), PhantomData);

        // Construct edges from `1`.
        // The edge (2, 1) is being filtered out as the edges are directed.
        let mut edges = Edges::new(from, &edges, neighbors);

        // Test all existing edges from `1`.
        assert_eq!(edges.next(), Some((1, 3, &3.0)));
        assert_eq!(edges.next(), Some((1, 4, &4.0)));

        // Test the end of iteration.
        assert_eq!(edges.next(), None);
    }

    #[test]
    #[should_panic]
    fn edges_next_unreachable() {
        // Prepare arguments.
        let from: u32 = 1;
        let edges: IndexMap<(u32, u32), f32> = IndexMap::new();
        let node_neighbors: Vec<(u32, CompactDirection)> = vec![(2, CompactDirection::Incoming)];
        let neighbors: Neighbors<u32, Directed> =
            Neighbors::new(node_neighbors.iter(), PhantomData);

        // Construct edges.
        let mut edges = Edges::new(from, &edges, neighbors);

        assert_eq!(edges.next(), Some((1, 2, &0.0)));
    }
}
