mod graph_stuff;
mod diff; 

fn main() {
    // let mut node = graph_stuff::Node::new("/Users/home/Documents/musical-fortnight/testing/first.txt"); 
    // let mut node2 = graph_stuff::Node::new("/Users/home/Documents/musical-fortnight/testing/second.txt"); 
    // graph_stuff::append_node(&mut node, node2); 
    // node.print_node(); 

    diff::diff("/Users/home/Documents/musical-fortnight/testing/cow1.txt", "/Users/home/Documents/musical-fortnight/testing/cow2.txt"); 
}
