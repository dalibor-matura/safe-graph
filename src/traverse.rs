//! Graph traversing structs.

use crate::edge::{
    CompactDirection,
    Direction::{self, Outgoing},
    EdgeType,
};
use crate::node::NodeTrait;
use crate::Undirected;
use std::marker::PhantomData;
use std::slice::Iter;

pub struct Neighbors<'a, N, Ty = Undirected>
where
    N: 'a,
    Ty: EdgeType,
{
    iter: Iter<'a, (N, CompactDirection)>,
    ty: PhantomData<Ty>,
}

impl<'a, N, Ty> Neighbors<'a, N, Ty>
where
    N: 'a,
    Ty: EdgeType,
{
    pub fn new(iter: Iter<'a, (N, CompactDirection)>, ty: PhantomData<Ty>) -> Self {
        Self { iter, ty }
    }
}

impl<'a, N, Ty> Iterator for Neighbors<'a, N, Ty>
where
    N: NodeTrait,
    Ty: EdgeType,
{
    type Item = N;
    fn next(&mut self) -> Option<N> {
        if Ty::is_directed() {
            (&mut self.iter)
                .filter_map(|&(n, dir)| if dir == Outgoing { Some(n) } else { None })
                .next()
        } else {
            self.iter.next().map(|&(n, _)| n)
        }
    }
}

pub struct NeighborsDirected<'a, N, Ty>
where
    N: 'a,
    Ty: EdgeType,
{
    iter: Iter<'a, (N, CompactDirection)>,
    dir: Direction,
    ty: PhantomData<Ty>,
}

impl<'a, N, Ty> NeighborsDirected<'a, N, Ty>
where
    N: 'a,
    Ty: EdgeType,
{
    pub fn new(iter: Iter<'a, (N, CompactDirection)>, dir: Direction, ty: PhantomData<Ty>) -> Self {
        Self { iter, dir, ty }
    }
}

impl<'a, N, Ty> Iterator for NeighborsDirected<'a, N, Ty>
where
    N: NodeTrait,
    Ty: EdgeType,
{
    type Item = N;
    fn next(&mut self) -> Option<N> {
        if Ty::is_directed() {
            let self_dir = self.dir;
            (&mut self.iter)
                .filter_map(move |&(n, dir)| if dir == self_dir { Some(n) } else { None })
                .next()
        } else {
            self.iter.next().map(|&(n, _)| n)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::edge::{
        CompactDirection,
        Direction::{Incoming, Outgoing},
    };
    use crate::graph::{Directed, Undirected};
    use crate::traverse::{Neighbors, NeighborsDirected};
    use std::marker::PhantomData;

    #[test]
    fn neighbors_new() {
        let nodes: Vec<(u32, CompactDirection)> = vec![];
        let iter = nodes.iter();

        // Test `Neighbors` struct creation.
        let _n: Neighbors<u32, Directed> = Neighbors::new(iter, PhantomData);
    }

    #[test]
    fn neighbors_next() {
        let nodes: Vec<(u32, CompactDirection)> = vec![
            (1, CompactDirection::Incoming),
            (2, CompactDirection::Outgoing),
            (3, CompactDirection::Outgoing),
        ];
        let iter = nodes.iter();

        let mut neighbors_directed: Neighbors<u32, Directed> =
            Neighbors::new(iter.clone(), PhantomData);

        assert_eq!(neighbors_directed.next(), Some(2));
        assert_eq!(neighbors_directed.next(), Some(3));
        assert_eq!(neighbors_directed.next(), None);

        let mut neighbors_undirected: Neighbors<u32, Undirected> =
            Neighbors::new(iter.clone(), PhantomData);

        assert_eq!(neighbors_undirected.next(), Some(1));
        assert_eq!(neighbors_undirected.next(), Some(2));
        assert_eq!(neighbors_undirected.next(), Some(3));
        assert_eq!(neighbors_undirected.next(), None);
    }

    #[test]
    fn neighbors_directed_new() {
        let nodes: Vec<(u32, CompactDirection)> = vec![];
        let iter = nodes.iter();

        // Test `Neighbors` struct creation.
        let _n: NeighborsDirected<u32, Directed> =
            NeighborsDirected::new(iter, Incoming, PhantomData);
    }

    #[test]
    fn neighbors_directed_next() {
        let nodes: Vec<(u32, CompactDirection)> = vec![
            (1, CompactDirection::Incoming),
            (2, CompactDirection::Outgoing),
            (3, CompactDirection::Outgoing),
        ];
        let iter = nodes.iter();

        let mut neighbors_incomming: NeighborsDirected<u32, Directed> =
            NeighborsDirected::new(iter.clone(), Incoming, PhantomData);

        assert_eq!(neighbors_incomming.next(), Some(1));
        assert_eq!(neighbors_incomming.next(), None);

        let mut neighbors_outgoing: NeighborsDirected<u32, Directed> =
            NeighborsDirected::new(iter.clone(), Outgoing, PhantomData);

        assert_eq!(neighbors_outgoing.next(), Some(2));
        assert_eq!(neighbors_outgoing.next(), Some(3));
        assert_eq!(neighbors_outgoing.next(), None);

        let mut neighbors_undirected: NeighborsDirected<u32, Undirected> =
            NeighborsDirected::new(iter, Outgoing, PhantomData);

        assert_eq!(neighbors_undirected.next(), Some(1));
        assert_eq!(neighbors_undirected.next(), Some(2));
        assert_eq!(neighbors_undirected.next(), Some(3));
        assert_eq!(neighbors_undirected.next(), None);
    }
}
