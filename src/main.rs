fn main() {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if args.len() == 0 {
        println!(
            "Usage:\n./nsp-cli --help\n\
            \t--list <path>: Lists all nsp files or part files in the specified directory\n\
            \t--split <path>: Splits an nsp file into fat32-compatible chunks given the path to it\n\
            \t--combine <path>: Combines multiple nsp file parts into one singular resulting nsp file given a path to a folder containing nsp parts\n\
            \nNote: all <path> will default to current working directory if none is supplied."
        );
        return; // Exit program
    }

    println!("{:?}", args);
}
