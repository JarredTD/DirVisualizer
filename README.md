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
