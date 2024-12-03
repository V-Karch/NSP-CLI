# nsp-cli

`nsp-cli` is a command-line utility written in Rust for working with NSP files. It provides simple and effective commands for listing, splitting, and combining NSP files. This tool can help you manage NSP files directly from the command line.

## Features

- **List NSP Files:** List NSP or XCI files in a given directory or NSP parts.
- **Split NSP Files:** Split a single NSP file into smaller, FAT32-compatible chunks.
- **Combine NSP Parts:** Combine multiple NSP file parts back into a single file.

## Installation

To build and install `nsp-cli` on your system, follow these steps:

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Clone the repository:
  ```bash
   git clone https://github.com/V-Karch/nsp-cli.git
  ```
3. Navigate into the project directory:
  ```bash
   cd nsp-cli
  ```
4. Build the project:
  ```bash
   cargo build --release
  ```
5. Once the build is complete, you can run `nsp-cli` using the generated executable.

## Usage

### List NSP Files

To list all NSP or XCI files in a given directory, use the following command:
```bash
nsp-cli --list <path>
```
If no path is provided, it defaults to the current working directory. This will display all NSP or XCI files, and also numeric files that may be split parts.

### Split NSP File

To split a given NSP file into smaller parts (up to 4GB per part), use:

```bash
nsp-cli --split <path_to_nsp_file>
```

This will split the NSP file into parts and save them in a new directory named after the file. The parts will be stored in a format like `01`, `02`, `03`.

### Combine NSP Parts

To combine previously split NSP parts back into a single file, use:

```bash
nsp-cli --combine <path_to_parts_directory>
```

This will take all the part files in the provided directory (e.g., `01`, `02`, `03`) and combine them into one NSP file.

### Help Message

To display the help message with detailed usage instructions, you can run:
```bash
nsp-cli --help
```

This will provide a description of the commands and options available.

## Examples

```bash
nsp-cli --list # List NSP Files in Current Directory
nsp-cli --list /path/to/directory # List NSP Files in a Specific Directory
nsp-cli --split /path/to/nspfile. # Split a NSP File
nsp-cli --combine /path/to/parts_directory # Combine NSP Parts
```
## Development

If you wish to contribute or make changes to the project, you can fork the repository and submit a pull request. Please ensure that all contributions align with the project's structure and provide proper tests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
