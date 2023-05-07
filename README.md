# Mycro

(wip)

Simple macros for Linux.

```
# replace |>mail for user@email.com:
mail=user@email.com
```

## Usage

Just add a `mycro` file to `$HOME/.config/` and run the program.

Example file:

```
# starter is a reserved word used to declare the
# sequence of keys that start a macro.
starter=|>

# a macro is declared as macro=result:
wl=hello, world
# this means that typing `|>hw` will trigger the macro and replace it with `\>hello. world`.

email=user@mail.com
workemail=user@work.com
```

## TODO

- [x] Read config file
- [x] Read from not hardcoded path
- [x] Emit keyboard events (write)
- [ ] Add all possible keyboard keys
  - [x] Uppercase
  - [ ] Tab, ctrl/cmd, super, shit, enter etc...
- [ ] Add instalation steps to README
- [ ] test on macOS and Windows
