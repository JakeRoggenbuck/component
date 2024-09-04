![Component Logo](./images/Component_dark_mode.png#gh-dark-mode-only)
![Component Logo](./images/Component_light_mode.png#gh-light-mode-only)
<br>
A math language using postfix notation.

## About

Component is the follow-up language to [Basis](https://github.com/JakeRoggenbuck/basis) which is also a math language. Component was built with insights learned from writing Basis. The main difference between the two is that Basis has a more complicated syntax which included types and data structures whereas Component uses postfix notation with the addition of functions as operations.

Note that the symbol `>` is used in code example to show that these expressions are being put into a [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) and evaluated. These symbols at the start of the expression are not apart of the syntax.

## Basic Math Operations

```
> 1 1 +
2
```

## Vector Operations

```
> 1 2 3 > 5 6 7 > x
4 -8 4
```

## Function

```
> x y + func foo

> 1 2 foo
3
```
