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
#[derive(Debug, PartialEq, Eq, Default, Clone)]
enum RBTreeArrayStruct<ContentType>{
    #[default]
    Empty,
    Node{ 
        value:ContentType, 
        left_index: usize, 
        right_index: usize,
        parent_index: usize,
        colour: NodeColour
    },
    NextInList( usize)
} 
#[derive(Debug, PartialEq, Eq, Clone)]
struct RBTreeNode<'a, ContentType, const N:usize>{
    array_struct : &'a RBTreeArrayStruct<ContentType>,
    nodes : &'a [RBTreeArrayStruct< ContentType>; N]
}
//This is bad. Turn all Option<Self> into Self with Empty being the None.
impl<'a, ContentType, const N:usize>  RBTreeNode<'a, ContentType,  N> {
    fn left_child(&self) -> Self {
        let array_struct ={
            if let RBTreeArrayStruct::Node{value:_, left_index: i, right_index: _ , parent_index: _ , colour: _} = self.array_struct{
                match self.nodes.get(*i){
                    Some(n)=> n,
                    None => &RBTreeArrayStruct::Empty
                }
            }
            else{&RBTreeArrayStruct::Empty}
        };
        Self{array_struct, nodes: self.nodes}

    }

    fn right_child(&self) -> Option<Self>{
        if let RBTreeArrayStruct::Node{value:_, left_index: _, right_index: i , parent_index: _ , colour: _} = self.array_struct{
            self.nodes.get(*i)
            .map(|node| Self{array_struct: node, nodes : self.nodes})
        }
        else{None}
    }

    fn parent(&self) -> Option<Self>{
        if let RBTreeArrayStruct::Node{value:_, left_index: _, right_index: _ , parent_index: i , colour: _} = self.array_struct{
            self.nodes.get(*i)
            .map(|node| Self{array_struct: node, nodes : self.nodes})
        }
        else{None}
    }

    fn value(&self) -> Option<&ContentType>{
        match self.array_struct{
            RBTreeArrayStruct::Node{value:v, left_index: _, right_index: _ , parent_index: _ , colour: _} => Some(v),
            _ => None
        }
    }

    fn colour(&self) -> Option<NodeColour>{
        match self.array_struct{
            RBTreeArrayStruct::Node{value:_, left_index: _, right_index: _ , parent_index: _ , colour: c} => Some(*c),
            _ => None
        }
    }

    fn successor(&self) -> Option<Self>{
        if let RBTreeArrayStruct::NextInList(i) = self.array_struct{
            self.nodes.get(*i)
            .map(|node| Self{array_struct: node, nodes : self.nodes})
        }
        else{None}

    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ArrayTree<ContentType, const N:usize>{
    all_nodes : [RBTreeArrayStruct< ContentType>; N],
    root_index : usize,
    never_used_start : usize,
    recycling_start : usize
}

impl<ContentType, const N:usize>  ArrayTree< ContentType,  N>{
    fn new()->Self{
        Self { all_nodes: [(); N].map(|_| RBTreeArrayStruct::Empty), root_index: N, never_used_start: 0, recycling_start: N }
    }
}



fn main() {
    let tree = ArrayTree::<i32, 100>::new();



}
