mod utils;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect(); // Collect command line arguments

    if args.len() == 0 {
        utils::display_help_message(); // Display usage message
        return; // Exit program
    } // No arguments were supplied

    if args[0] == "--list" {
        if args.len() == 2 {
            utils::list_possible_nsp_given_path(&args[1]);
        } else {
            utils::list_possible_nsp_given_path(
                &std::env::current_dir().unwrap().display().to_string(),
            );
        }
    }

    if args[0] == "--split" {
        if args.len() == 2 {
            let _ = utils::split_file_into_parts(&args[1]);
        } else {
            utils::display_help_message();
        }
    }

    if args[0] == "--combine" {
        if args.len() == 2 {
            let _ = utils::combine_parts_into_file(&args[1]);
        } else {
            utils::display_help_message();
        }
    }

    println!("{:?}", args); // Display collected arguments for debug purposes
}
