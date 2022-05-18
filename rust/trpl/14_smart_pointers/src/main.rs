mod box_test;
mod drop_test;
mod rc_test;
mod refcell_test;
mod refcell_and_rc;
mod ref_cycle;
mod tree_node;

use box_test::{ cons_list, coerced_deref };
use drop_test::{ print_drop, early_drop };
use rc_test::many_cons_lists;
use refcell_and_rc::multiple_mut;
use ref_cycle::reference_cycle;
use tree_node::tree_child_info;

fn main() {
    println!("This is a showcase of smart pointers!\nCheck out the code.\n");
    cons_list();

    println!("\n");
    coerced_deref();

    println!("\n");
    print_drop();

    println!("\n");
    early_drop();

    println!("\n");
    many_cons_lists();

    println!("\nMultiple mutable borrows example:");
    multiple_mut();

    println!("\nExample of a reference cycle (go to code and uncomment):");
    reference_cycle();

    println!("\nUsing Weak to create a tree with parent info:");
    tree_child_info();
}
