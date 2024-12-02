use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

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
    let paths = fs::read_dir(string_path).unwrap();
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
        println!(
            "No nsp files or numeric files found in directory `{}`",
            string_path
        );
    }
}

/// Splits a file into parts of up to 4GB and saves them with numbered filenames in a new directory.
pub fn split_file_into_parts(file_path: &str) -> io::Result<()> {
    const CHUNK_SIZE: usize = 64 * 1024 * 1024; // 64MB buffer
    const MAX_PART_SIZE: usize = 4 * 1024 * 1024 * 1024; // 4GB

    // Parse the input file path
    let input_path = Path::new(file_path);
    if !input_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Input file does not exist",
        ));
    }

    let file_stem = input_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    // Create output directory
    let output_dir = input_path.with_file_name(format!("{}", file_stem));
    fs::create_dir_all(&output_dir)?;

    // Open the input file
    let mut input_file = File::open(input_path)?;

    let file_size = input_file.metadata()?.len(); // Get the size of the input file
    let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Progress bar error: {}", e)))?
            .progress_chars("#>-"),
    );

    let mut buffer = vec![0; CHUNK_SIZE];
    let mut part_number = 1;
    let mut bytes_written_for_part = 0;
    let mut output_file: Option<File> = None;

    loop {
        // Read a small chunk from the input file
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }

        // If no output file or we've reached the maximum size for the current part, start a new part
        if bytes_written_for_part == 0 || bytes_written_for_part >= MAX_PART_SIZE {
            if let Some(file) = output_file.take() {
                file.sync_all()?; // Ensure all data is written
            }
            let output_file_name = format!("{:02}", part_number);
            let mut output_file_path = PathBuf::from(&output_dir);
            output_file_path.push(output_file_name);
            output_file = Some(File::create(output_file_path)?);
            bytes_written_for_part = 0;
            part_number += 1;
        }

        // Write to the current output file
        if let Some(ref mut file) = output_file {
            file.write_all(&buffer[..bytes_read])?;
            bytes_written_for_part += bytes_read;
            progress_bar.inc(bytes_read as u64);
        }
    }

    // Ensure the last file is finalized
    if let Some(file) = output_file {
        file.sync_all()?;
    }

    progress_bar.finish_with_message("File split completed!");

    println!("File successfully split into parts in: {:?}", output_dir);
    return Ok(());
}

/// Combines split files (e.g., `01`, `02`, `03`) back into the original large file.
/// The output file is named after the input directory with the `.nsp` extension.
pub fn combine_parts_into_file(input_dir_path: &str) -> io::Result<()> {
    let input_dir = Path::new(input_dir_path);
    if !input_dir.exists() || !input_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Input directory does not exist or is not a directory",
        ));
    }

    // Derive the output file name from the directory name
    let output_file_name = format!("{}.nsp", input_dir.file_name().unwrap().to_string_lossy());
    let output_file_path = input_dir
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .join(&output_file_name);

    // Sort the part files (e.g., `01`, `02`, `03`)
    let mut part_files: Vec<PathBuf> = fs::read_dir(input_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.is_file())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| name.chars().all(|c| c.is_digit(10)))
        })
        .collect();

    part_files.sort(); // Ensure files are processed in order (e.g., `01`, `02`, `03`)

    if part_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No part files found in the input directory",
        ));
    }

    // Calculate the total size of all part files
    let total_size: u64 = part_files
        .iter()
        .map(|file| file.metadata().unwrap().len())
        .sum();

    // Create the output file
    let mut output_file = File::create(&output_file_path)?;

    // Set up the progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Progress bar error: {}", e))
            })?
            .progress_chars("#>-"),
    );

    // Handling reading and writing of file parts
    let buffer_size = 64 * 1024; // 64KB buffer for reading
    let mut buffer = vec![0; buffer_size];

    let mut processed_size = 0u64;

    // Loop over the part files and combine them
    for part_file in part_files {
        let mut input_file = File::open(&part_file)?;

        // Read the part file in chunks and write it to the output file
        loop {
            let bytes_read = input_file.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // End of this part file
            }
            output_file.write_all(&buffer[..bytes_read])?;

            // Update the progress bar
            processed_size += bytes_read as u64;
            pb.set_position(processed_size);
        }
    }

    pb.finish_with_message("Done"); // Finish the progress bar with a message
    println!(
        "Files successfully combined into: {}",
        output_file_path.display()
    );
    Ok(())
}
