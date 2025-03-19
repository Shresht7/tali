# `tali`

⚠️ Work-in-Progress ⚠️

A command-line application to count the number of lines, words, characters and bytes in a given set of files.

TODO: Add a demo screenshot/gif

## 🌟 Features

- **Scanning**: Scan a set of files (or directories) for the number of lines, words, characters and bytes.
- **Language Detection**: Determines the languages used (based on the file extension)
- **Configurable Output**: Choose from multiple output formats (Table, CSV, TSV, JSON)
- **Colorized**: Defaults to a colorized output when printing to the console.
- **Visualization**: Displays a graphical measure of each file's size relative to the largest file.
- **Group by Language**: Aggregate and group the results by language

TODO: Review the features list

---

## 📦 Installation

TODO

## 📘 Usage

Run `tali` by specifying one or more files or directories to scan:

```sh
tali path/to/directory
```

By default, `tali` outputs the results in a tabular format with colors.

TODO: Add a screenshot of the default output

The output can be configured using the various command-line arguments.

### Options

- `-l, --lines`: Shows the line count
- `-w, --words`: Show the word count
- `-c, --chars`: Shows the character count
- `-b, --bytes`: Show the byte count
- `-e, --language`: Show the corresponding language
- `-v, --visualize`: Show a graphical visualization
- `-f, --format`: Configures the output format (`"table"`, `"json"`, `"plain"`)

Use `--help` to get the full help for more details.

### Examples

TODO: Fill out the examples

#### `tali`

#### `tali README.md --lines`

#### `tali src --language --lines --chars --format json`

#### `tali README.md .gitignore src --lines --format plain`

---

## ⚽ Goals

- Learn Rust
- Build Stuff

## 🖥️ Development

### Project Structure

The cli logic resides in [`main.rs`](./src/main.rs) and the definitions for the command-line arguments are in the [`cli.rs`](./src/cli.rs) file.

The library [`src/lib`](./src/lib/) contains three modules:
- [`scanner`](./src/lib/scanner/): Responsible for walking the file-system and scanning the file metrics
- [`output`](./src/lib/output/): Responsible for formatting and displaying the scan results.
- [`helpers`](./src/lib/helpers/): An amalgamation of helpers and utilities used throughout the project.

### To Do

- [x] Support Reading from STDIN
- [ ] Option to get the max line (column) width
- [ ] Filter
- [ ] Sort
- [ ] Parallelization

### 📕 References

- https://github.com/XAMPPRocky/tokei
- https://boyter.org/posts/sloc-cloc-code/
- https://github.com/cgag/loc
- https://github.com/boyter/scc

---

## 📄 License

This project is licensed under the [MIT LICENSE](./LICENSE). See the [LICENSE](./LICENSE) file for more details.
