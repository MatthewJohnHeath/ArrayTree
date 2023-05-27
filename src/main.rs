#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
enum Tree<'a>{
    #[default]
    Empty,
    Node{ value:i32, left: &'a Tree<'a>, right: &'a Tree<'a>}
}

fn main() {
    let mut tree_array = [Tree::Empty; 100];
    let mut one = Tree::Node{value:0, left: &Tree::Empty, right: &Tree::Empty};
   
    match & mut one{
     Tree::Node{ value:v, left:_, right:_} => {*v = 1},
    _=>{},
    };
    
    tree_array[0] = one.clone();
    
    match & mut tree_array[0]{
    Tree::Node{ value:_, left:l, right:_} => {*l = &one},
    _=>{},
    };
    
    let two = Tree::Node{value:2, left: &Tree::Empty, right: &Tree::Empty};
    let three_two_one = Tree::Node{ value : 3, left: &one, right:&two};
    println!("{:?}", three_two_one);
}
