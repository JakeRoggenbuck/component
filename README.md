![Component Logo](./images/Component_dark_mode.png#gh-dark-mode-only)
![Component Logo](./images/Component_light_mode.png#gh-light-mode-only)
<br>
A math language using postfix notation.

## About

Component is the follow-up language to [Basis](https://github.com/JakeRoggenbuck/basis) which is also a math language. Component was built with insights learned from writing Basis. The main difference between the two is that Basis has a more complicated syntax which included types and data structures whereas Component uses postfix notation with the addition of functions as operations.

Note that the symbol `>` is used in code example to show that these expressions are being put into a [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) and evaluated. These symbols at the start of the expression are not apart of the syntax.

The code for Component and Basis use the same lexer. Component is mainly different because it has a different syntax. The lexer for basis was written generally enough that this code can be shared. It also helps that Basis is a math language inspired by postfix notation but not exclusively postfix notation.

## Interactive interpreter
![image](https://github.com/user-attachments/assets/3d1978d6-c3b7-4954-9ba5-6b8958e396b1)

## Basic Math Operations
Add two numbers together.

```
> 1 1 +
-> 2
```

This also works with `-` for subtraction, `*` for multiplication, and `/` for division.

## Vector Operations (Coming Soon)

```
> 1 2 3 > 5 6 7 > x
-> 4 -8 4
```

## Variables

Assign the value 2 to variable `a`.

```
> 2 a =
-> 2
```

Use the variable `a`.
```
a 4 *
-> 8
```

Variables are statically typed in Component. Here is an example usage of a variable.

![image](https://github.com/user-attachments/assets/f20443c1-3a83-4336-9b01-2309e2bc0af9)

## Function (Coming Soon)
Create an addition function called `foo`.

```
> x y + func foo

> 1 2 foo
-> 3
```

## Type Conversion

```
> 4 5 /
-> 0.8

> 4 5 / int
-> 0
```

```
> 2 dec
-> 1
```

## Error Handling

![image](https://github.com/user-attachments/assets/993e8eb3-0ca2-4a20-8b30-37dd405992bd)

