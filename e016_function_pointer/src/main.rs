fn for_each(v: &Vec<i32>, f: fn(&i32)) {
    for a in v {
        f(&a);
    }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    for_each(&v, |x| println!("{}", x+1));
}
