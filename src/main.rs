const i32 set_size = 5;

fn set_intersection(a: [i32; 5], b: [i32; 5]) {
    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i] == b[j] {
                println!("a {} and b {} match! value: {}", i, j, a[i]);
            }
        }
    }
}

fn set_union(a: [i32; 5], b: [i32; 5]) {

}

fn main() {
    let a: [i32; 5] = [ 0, 1, 2, 3, 4 ];
    let b: [i32; 5] = [ 3, 4, 5, 6, 7 ];

    set_intersection(a, b);
}
