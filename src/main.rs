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

    let mut a_mult = MultiSet::from_vec(vec![3, 2, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mut b_mult = MultiSet::from_vec(vec![0, 1, 4, 0, 0, 0, 0, 0, 0, 0]);

    a_mult.elements[0] = 3;
    a_mult.elements[1] = 2;
    b_mult.elements[1] = 1;
    b_mult.elements[2] = 4;

    println!("Set A:");
    a.display();
    println!("Set B:");
    b.display();

    let not_a = a.complement();
    println!("Not A:");
    not_a.display();

    let a_union_b = a.union(&b);
    println!("A union B:");
    a_union_b.display();

    let a_intersection_b = a.intersection(&b);
    println!("A intersection B:");
    a_intersection_b.display();

    let a_difference_b = a.difference(&b);
    println!("A difference B:");
    a_difference_b.display();

    let a_symmetric_difference_b = a.symmetric_difference(&b);
    println!("A symmetric difference B:");
    a_symmetric_difference_b.display();

    println!("MultiSet A:");
    a_mult.display();
    println!("MultiSet B:");
    b_mult.display();

    let a_union_b = a_mult.union(&b_mult);
    println!("A union B:");
    a_union_b.display();

    let a_intersection_b = a_mult.intersection(&b_mult);
    println!("A intersection B:");
    a_intersection_b.display();

    let a_difference_b = a_mult.difference(&b_mult);
    println!("A difference B:");
    a_difference_b.display();

    println!("Sum of elements in A: {}", a_mult.sum());
    println!("Sum of elements in B: {}", b_mult.sum());
}
