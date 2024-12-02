mod utils;

fn main() {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .collect(); // Collect command line arguments

    if args.len() == 0 {
        utils::display_help_message(); // Display usage message
        return; // Exit program
    } // No arguments were supplied

    println!("{:?}", args); // Display collected arguments for debug purposes
}
