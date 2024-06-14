# DirVisualizer
[![Check](https://github.com/JarredTD/DirVisualizer/actions/workflows/check.yml/badge.svg)](https://github.com/JarredTD/DirVisualizer/actions/workflows/check.yml)
[![Format](https://github.com/JarredTD/DirVisualizer/actions/workflows/fmt.yml/badge.svg)](https://github.com/JarredTD/DirVisualizer/actions/workflows/fmt.yml)
[![Lint](https://github.com/JarredTD/DirVisualizer/actions/workflows/clippy.yml/badge.svg)](https://github.com/JarredTD/DirVisualizer/actions/workflows/clippy.yml)

Tool for visualizing directories and their children

## Usage

```bash
DirVisualizer [OPTIONS] [STARTING_DIR]
```

Arguments:
- **[STARTING_DIR]**
  Default: `.`
Options:
- **-e, --exclude \[\<EXCLUDE>...]**
- **-h, --help**
  Print help
- **-V, --version**
  Print version

### Example
`DirVisualizer --exclude target`
```txt
visualizer
	|____Cargo.toml
	|____LICENSE
	|____test
	|____README.md
	|____.gitignore
	|src
		|____main.rs
```

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the [LICENSE](LICENSE) file for details. AGPL-3.0 is a free, copyleft license designed specifically for software and other kinds of works, offering the freedom to run, study, share, and modify the software. It's similar to the GPL-3.0 but with an additional term to cover the use of the software over a network.

For more details on the AGPL-3.0 License, please refer to [gnu.org/licenses/agpl-3.0](https://www.gnu.org/licenses/agpl-3.0).
