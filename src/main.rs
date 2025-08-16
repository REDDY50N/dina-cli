struct CLI {
    command: String,
    target: String,
}

fn main() {
    // // -- Read all cli args into a vector of strings
    // let args: Vec<String> = std::env::args().collect();

    // // -- Print all args
    // println!("{:?}", args);
    //

    let command = std::env::args()
        .nth(1)
        .expect("Missing command! Options: list, init, add, edit, remove, ");
    let target = std::env::args()
        .nth(2)
        .expect("Missing target! Options: recipe, meal-plan, stock, ...");

    let args = CLI { command, target };

    println!("pattern: {:?}, path: {:?}", command, target)
}
