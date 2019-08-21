fn main() {
    // No problem beacuse primitives are copied
    let a = 5;
    let _b = a;
    println!("{}", a);

    // Won't compile
    // let v1 = vec![0, 2, 4, 6, 8];
    // let _v2 = v1;    // Transfer ownership
    // println!("{:?}", v1);
}
