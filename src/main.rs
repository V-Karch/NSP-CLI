fn main() {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if args.len() == 0 {
        println!("No arguments supplied"); // Add --help here
        return; // Leave function early
    }

    println!("{:?}", args);
}
