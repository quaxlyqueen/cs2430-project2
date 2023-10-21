mod multiset_operations;
mod set_operations;

use multiset_operations::MultiSet;
use set_operations::Set;

fn main() {
    let a = Set::from_vec(vec![
        true, false, true, false, true, false, true, false, true, false,
    ]);

    let b = Set::from_vec(vec![
        false, true, false, true, false, true, false, true, false, true,
    ]);

    // Multisets, where each element (ie. element 0, 1, 2) has a count
    let a_mult = MultiSet::from_vec(vec![3, 2, 0, 0, 0, 0, 0, 0, 0, 0]);
    let b_mult = MultiSet::from_vec(vec![0, 1, 4, 0, 0, 0, 0, 0, 0, 0]);

    println!("Set A:");
    a.display();
    println!("Set B:");
    b.display();

    // A and B sets together
    let a_union_b = a.union(&b);
    println!("A union B:");
    a_union_b.display();

    // The common elements between sets A and B.
    let a_intersection_b = a.intersection(&b);
    println!("A intersection B:");
    a_intersection_b.display();

    // Elements that are a part of set B are no longer a part of set A.
    let a_difference_b = a.difference(&b);
    println!("A difference B:");
    a_difference_b.display();

    // Remove the common elements from set A and B.
    let a_symmetric_difference_b = a.symmetric_difference(&b);
    println!("A symmetric difference B:");
    a_symmetric_difference_b.display();

    println!("MultiSet A:");
    a_mult.display();
    println!("MultiSet B:");
    b_mult.display();

    // Union of both multisets A and B 
    let a_union_b = a_mult.union(&b_mult);
    println!("A union B:");
    a_union_b.display();

    // Common elements between multisets A and B
    let a_intersection_b = a_mult.intersection(&b_mult);
    println!("A intersection B:");
    a_intersection_b.display();

    // Removing any common elements of set B from set A
    let a_difference_b = a_mult.difference(&b_mult);
    println!("A difference B:");
    a_difference_b.display();

    println!("Sum of elements in A: {}", a_mult.sum());
    println!("Sum of elements in B: {}", b_mult.sum());
}