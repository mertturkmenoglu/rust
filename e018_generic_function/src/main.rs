struct Container<T> {
    value: T
}

impl Container<i32> {
    fn print_value(self) {
        println!("{}", self.value.to_owned());
    }
}

fn main() {
    let a = Container { value: 5 };
    a.print_value();
}