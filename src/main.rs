use rand::Rng;

const SIZE: usize = 100;

fn main() {
    let mut rng = rand::thread_rng();

    let mut a = Vec::new();
    let mut b = Vec::new();

    let mut count = 0;
    while count < SIZE {
        a.push(rng.gen_range(0..SIZE));
        b.push(rng.gen_range(0..SIZE));
        count = count + 1;
    }

    println!("\n\nunion:");
    let u = union_or_intersection(&a, &b, 0);
    for i in 0..u.len() {
        let x = u[i];
        print!("{x} ");
    }

    println!("\n\nintersection:");
    let inter = union_or_intersection(&a, &b, 1);
    for i in 0..inter.len() {
        let x = inter[i];
        print!("{x} ");
    }

    println!("\n\ndifference:");
    let diff = diff_or_symdiff(&a, &b, 0);
    for i in 0..diff.len() {
        let x = diff[i];
        print!("{x} ");
    }

    println!("\n\nsymmetric difference:");
    let sym_diff = diff_or_symdiff(&a, &b, 1);
    for i in 0..sym_diff.len() {
        let x = sym_diff[i];
        print!("{x} ");
    }
}

// Get the set (represented as a vector) containing the union or intersection of sets a and b (also represented as vectors).
fn union_or_intersection(a: &Vec<usize>, b: &Vec<usize>, u_or_i: u32) -> Vec<usize> {
    let mut x = Vec::new();

    for n in 0..a.len() {
        x.push(a[n]);
    }

    for n in 0..b.len() {
        let mut z = true;
        for j in 0..a.len() {
            if u_or_i == 0 {            // Union of a and b
                if b[n] == a[j] {
                    z = false;
                    break;
                }
            } else if u_or_i == 1 {     // Intersection of a and b
                if b[n] != a[j] {
                    z = false;
                    break;
                }
            }
        }

        if z {
            x.push(b[n]);
        }
    }
    x
}

// Get the set (represented as a vector) containing the difference or symmetric difference of sets a and b (also represented as vectors).
fn diff_or_symdiff(a: &Vec<usize>, b: &Vec<usize>, symmetric: u32) -> Vec<usize> {
    let mut x = Vec::new();

    for i in 0..a.len() {
        let mut z = true;

        for j in 0..b.len() {
            if a[i] == b[j] {
                z = false;
            }
        }
        if z {
            x.push(a[i]);
        }
    }

    if symmetric == 1 {                 // If we're getting the symmetric difference
        for i in 0..b.len() {
            let mut z = true;

            for j in 0..a.len() {
                if b[i] == a[j] {
                    z = false;
                }
            }
            if z {
                x.push(b[i]);
            }
        }
    }
    x
}
