# Sofa Editor

- Intuitive
- Simple
- Vim-like

**Warning**: Still in development. <br>

# Controls

## Switch Between Modes

| Mode                        | Action                  |
|-----------------------------|-------------------------|
| `Insert + Escape` -> `Base` | Switch to `Base` mode   |
| `Base + i` -> `Insert`      | Switch to `Insert` mode |

## Basic Movement

| Mode                 | Action            |
|----------------------|-------------------|
| `Base + h` -> `Base` | Move cursor left  |
| `Base + j` -> `Base` | Move cursor down  |
| `Base + k` -> `Base` | Move cursor up    |
| `Base + l` -> `Base` | Move cursor right |

## Delete Modifier

| Mode                          | Action                   |
|-------------------------------|--------------------------|
| `Base + d` -> `Base (Delete)` | Enable modifier          |
| `Base (Delete) + c` -> `Base` | Delete current character |
| `Base (Delete) + l` -> `Base` | Delete current line      |
| `Base (Delete) + q` -> `Base` | Disable modifier         |

## Go To Modifier

| Mode                          | Action                          |
|-------------------------------|---------------------------------|
| `Base + g` -> `Base (Go to)`  | Enable modifier                 |
| `Base (Go to) + t` -> `Base`  | Go to the top                   |
| `Base (Go to) + b` -> `Base`  | Go to the bottom                |
| `Base (Go to) + s` -> `Base`  | Go to the start of current line |
| `Base (Go to) + e` -> `Base`  | Go to the end of current line   |
| `Base (Create) + q` -> `Base` | Disable modifier                |

## Create Modifier

| Mode                          | Action                   |
|-------------------------------|--------------------------|
| `Base + c` -> `Base (Create)` | Enable modifier          |
| `Base (Create) + a` -> `Base` | Create line above cursor |
| `Base (Create) + b` -> `Base` | Create line below cursor |
| `Base (Create) + q` -> `Base` | Disable modifier         |
