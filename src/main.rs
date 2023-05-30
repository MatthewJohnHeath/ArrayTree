use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;
use std::ops::Not;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {Left, Right}
impl Not for Direction{
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    } 
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NodeColour{Red, Black}
#[derive(Debug, PartialEq, Eq, Default)]
enum RBTreeOrBreadcrumb<'a, ContentType>{
    #[default]
    Empty,
    Node{ 
        value:ContentType, 
        left: &'a Self, 
        right: &'a Self,
        parent: &'a Self,
        colour: NodeColour
    },
    Trail( &'a RBTreeOrBreadcrumb<'a, ContentType>)
} 
impl<'a, ContentType> RBTreeOrBreadcrumb<'a, ContentType>{
    fn child(&self, dir:Direction)->&Self{
        match (self, dir){
            (Self::Node{value:_, left: child, right:_, parent:_, colour:_}, Direction::Left) 
            | (Self::Node{value:_, left: _, right:child, parent:_, colour:_}, Direction::Right) 
                => &child,
            _ => &Self::Empty
        }
    }
}
struct ArrayTree<'a, ContentType, const N:usize>{
    all_nodes : [RBTreeOrBreadcrumb<'a, ContentType>; N],
    root : &'a RBTreeOrBreadcrumb<'a, ContentType>,
    recycling : &'a RBTreeOrBreadcrumb<'a, ContentType>,
    unused : core::slice::Iter<'a, RBTreeOrBreadcrumb<'a, ContentType>>
}

impl<'a, ContentType,  const N: usize> ArrayTree<'a, ContentType,  N> {
    
    fn new() -> Self {
        let mut uninit :MaybeUninit< Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();
        unsafe{ addr_of_mut!((*ptr).all_nodes).write([();N].map(|_| RBTreeOrBreadcrumb::Empty)); }
        unsafe{ addr_of_mut!((*ptr).root).write(&RBTreeOrBreadcrumb::Empty) ;}
        unsafe{ addr_of_mut!((*ptr).recycling).write(&RBTreeOrBreadcrumb::Empty) ;}
        unsafe{ addr_of_mut!((*ptr).unused).write((*ptr).all_nodes.iter()) ;}
        unsafe { uninit.assume_init() }
    }
}
impl<'a, ContentType,  const N: usize> ArrayTree<'a, ContentType,  N> {
    fn obtain_next (&mut self)->&RBTreeOrBreadcrumb< 'a, ContentType>{
        if let RBTreeOrBreadcrumb::Trail(next) = self.recycling{
            let got : &'a RBTreeOrBreadcrumb< 'a, ContentType> = self.recycling; 
            self.recycling = next;
            got
        }
        else{
            self.unused.next()
            .unwrap_or(&RBTreeOrBreadcrumb::Empty)
        }
    }
}
fn main() {
    let tree = ArrayTree::<i32, 100>::new();



}
