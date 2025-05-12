mod graph_stuff;
mod log_manager; 
mod diff; 

fn main() {
    let mut node = graph_stuff::Node::new("../testing/first.txt"); 
    let mut node2 = graph_stuff::Node::new("../testing/first.txt"); 
    graph_stuff::append_node(&mut node, node2); 
    node.print_node(); 

    let changes = diff::diff("../testing/first.txt", "../testing/first_alt.txt").expect("REASON"); 
    let changed = diff::apply_changes("../testing/first.txt", &changes).expect("REASON"); 
    //let _ = log_manager::init_log("/Users/home/Documents/musical-fortnight/testing/first.txt"); 
    println!("{}", changed); 
}
