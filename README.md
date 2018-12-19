# i3-focused-window
Print out the geometry of the currently focused windows on [i3](https://github.com/i3/i3) and [sway](https://github.com/swaywm/sway/).

## Installation
```
~ $ cargo install i3-focused-window
```

## Usage
```
USAGE:
    i3-focused-window [format]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <format>    Output format.
                You can use '%x' and '%y' for offset,
                '%w' and '%h' for dimensions (width and height).
                Example: "%x,%y %wx%h" gives "-1+2 3x4". [default: %wx%h %x%y]
```
