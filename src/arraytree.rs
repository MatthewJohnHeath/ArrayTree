
//use std::mem::MaybeUninit;
//use std::ptr::addr_of_mut;

pub mod libb{
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
enum RBTreeArrayElement<ContentType>{
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
    element : &'a RBTreeArrayElement<ContentType>,
    nodes : &'a [RBTreeArrayElement< ContentType>; N]
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SearchResult<'a, ContentType>{
    None,
    At(&'a RBTreeArrayElement<ContentType>),
    Before(&'a RBTreeArrayElement< ContentType>),
    After (&'a RBTreeArrayElement< ContentType>)
}

impl<'a, ContentType, const N:usize>  RBTreeNode<'a, ContentType,  N> {
    fn left_child(&self) -> Self {
        let element ={
            if let RBTreeArrayElement::Node{left_index: i,..}  = self.element{
                match self.nodes.get(*i){
                    Some(n)=> n,
                    None => &RBTreeArrayElement::Empty
                }
            }
            else{&RBTreeArrayElement::Empty}
        };
        Self{element, nodes: self.nodes}
    }

    fn right_child(&self) -> Self {
        let element ={
            if let RBTreeArrayElement::Node{right_index: i ,..} = self.element{
                match self.nodes.get(*i){
                    Some(n)=> n,
                    None => &RBTreeArrayElement::Empty
                }
            }
            else{&RBTreeArrayElement::Empty}
        };
        Self{element, nodes: self.nodes}
    }

    fn parent(&self) -> Self {
        let element ={
            if let RBTreeArrayElement::Node{parent_index: i , ..} = self.element{
                match self.nodes.get(*i){
                    Some(n)=> n,
                    None => &RBTreeArrayElement::Empty
                }
            }
            else{&RBTreeArrayElement::Empty}
        };
        Self{element, nodes: self.nodes}
    }

    fn value(&self) -> Option<&ContentType>{
        match self.element{
            RBTreeArrayElement::Node{value:v, ..} => Some(v),
            _ => None
        }
    }

    fn colour(&self) -> Option<NodeColour>{
        match self.element{
            RBTreeArrayElement::Node{ colour: c, ..} => Some(*c),
            _ => None
        }
    }

    fn successor(&self)-> Self {
        let element ={
            if let RBTreeArrayElement::NextInList(i) = self.element{
                match self.nodes.get(*i){
                    Some(n)=> n,
                    None => &RBTreeArrayElement::Empty
                }
            }
            else{&RBTreeArrayElement::Empty}
        };
        Self{element, nodes: self.nodes}
    }

    
    
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayTree<ContentType, const N:usize>{
    all_nodes : [RBTreeArrayElement< ContentType>; N],
    root_index : usize,
    never_used_start : usize,
    recycling_start : usize
}

impl<ContentType, const N:usize>  ArrayTree< ContentType,  N>{
    pub fn new()->Self{
        Self { 
            all_nodes: [(); N].map(|_| RBTreeArrayElement::Empty), 
            root_index: 0,
            never_used_start: 0, 
            recycling_start: N //obviously it sucks that we have sentinels
         } 
    }

    fn root(&self)->RBTreeNode<ContentType,  N>{
        RBTreeNode{element:&self.all_nodes[self.root_index], nodes: &self.all_nodes}
    }



    fn add(& mut self, val: ContentType ){
        match &&self.all_nodes[self.root_index]{
            &&RBTreeArrayElement::Empty 
                => {
                    self.all_nodes[self.root_index] =     
                    RBTreeArrayElement::Node{ 
                                        value: val, 
                                        left_index: self.root_index , 
                                        right_index: self.root_index,
                                        parent_index: self.root_index,
                                        colour: NodeColour::Black
                                    }
                },
            _=> todo!(),

        }
    }

}

}
