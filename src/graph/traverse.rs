//! Graph traversing structs.

use crate::graph::edge::Direction::Outgoing;
use crate::graph::edge::{CompactDirection, Direction, EdgeType};
use crate::graph::node::NodeTrait;
use crate::graph::Undirected;
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
