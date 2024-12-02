pub fn display_help_message() {
    println!(
        "Usage:\n./nsp-cli --help\n\
        \t--list <path>: Lists all nsp or xci files in a given directory or nsp parts\n\
        \t--split <path>: Splits an nsp file into fat32-compatible chunks given the path to it\n\
        \t--combine <path>: Combines multiple nsp file parts into one singular resulting nsp file given a path to a folder containing nsp parts\n\
        \nNote: all <path> will default to current working directory if none is supplied."
    );
}


pub fn list_possible_nsp_given_path(string_path: &str) {
    let paths = std::fs::read_dir(string_path).unwrap();
    let mut found_count: usize = 0;

    for raw_path in paths {
        let entry = raw_path.unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        let relative_path = entry.path().display().to_string();

        // Check if the file ends with .nsp or .xci
        if relative_path.ends_with(".nsp") || relative_path.ends_with(".xci") {
            println!("{}", relative_path);
            found_count += 1;
        } else {
            // Check if the filename is numeric
            if file_name.parse::<u32>().is_ok() {
                println!("{}", relative_path);
                found_count += 1;
            }
        }
    }

    if found_count == 0 {
        println!("No nsp files or numeric files found in directory `{}`", string_path);
    }
}