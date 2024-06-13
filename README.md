# DirVisualizer

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
