fn main() {
    let name = "Diana";
    let msg = match name {
        "Emily" => "Emily is the best",
        "Diana" => {
            println!("First this will be printed");
            "Diana is good"
        },
        _ => "I don't know"
    };

    println!("{}", msg);
}
