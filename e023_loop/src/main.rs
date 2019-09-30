fn main() {
    let mut index = 0;

    let variable = loop {
        if index >= 10 {
            break index;
        }

        index += 1;
    };

    println!("{}", variable);
}
