# Mycro

Simple text macros for Linux.

Mycro should also work on macOS and Windows, but these are currently untested.

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
# this means that typing `|>hw` will trigger the macro and replace it with `\>hello, world`.

email=user@mail.com
workemail=user@work.com
```

## TODO list

- [ ] Add missing keys (shift, control, alt, del, home, etc...)
- [ ] Setup CI/CD

