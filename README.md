![Component Logo](./images/Component_dark_mode.png#gh-dark-mode-only)
![Component Logo](./images/Component_light_mode.png#gh-light-mode-only)
<br>
A math language using postfix notation.

## About

Component is the follow-up language to [Basis](https://github.com/JakeRoggenbuck/basis) which is also a math language. Component was built with insights learned from writing Basis. The main difference between the two is that Basis has a more complicated syntax which included types and data structures whereas Component uses postfix notation with the addition of functions as operations.

The code for Component and Basis use the same lexer. Component is mainly different because it has a different syntax. The lexer for basis was written generally enough that this code can be shared. It also helps that Basis is a math language inspired by postfix notation but not exclusively postfix notation.

## Interactive interpreter
![image](https://github.com/user-attachments/assets/3d1978d6-c3b7-4954-9ba5-6b8958e396b1)

Note that the symbol `>` is used in code example to show that these expressions are being put into a [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) and evaluated. These symbols at the start of the expression are not apart of the syntax.

## Keywords

- `int` converts types into NumericIntLiteral if possible
- `dec` converts types into NumericDecLiteral if possible

## Constants
- `e` Euler's number

## Built-in Functions
- `sqrt` Square root

## Operations
- `+` addition
- `-` subtraction
- `*` multiplication
- `/` division
- `^` exponentiation

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

Errors that occur in the interactive interpreter cause the line being interpreted to crash. When this happens, one of the following error messages will be displayed.

#### Assignment Type Mismatch [E1]

An Assignment Type Mismatch happens when you try to assign a value to an existing variable of a different type. In the following example, the variable `a` is created as a NumericIntLiteral type. Then the value `0.8` is attempted to be assigned the variable `a` but because `0.8` is of the type NumericDecLiteral, it fails with an Assignment Type Mismatch error. This means that variables are statically typed and cannot be changed during runtime.

```
> 1 a =
-> 1

> 4 5 / a =
Error: Assignment Type Mismatch [E1]
0.8 a =
    ^^^ cannot assign value 0.8 of type <NumericDecLiteral> to a variable of type <NumericIntLiteral>
```

#### Wrong Type [E2]

A Wrong Type error happens when you try to call an operation or a function on one or more variables of the wrong type. Here is an example of an identifier being used before it has been assigned. Since it does not get swapped out for a value, since it hasn't been assigned, it has the type Identifier and therefore is the wrong type. In the future, this will also happen for math operations on the type String.

```
> a 1 +
Error: Wrong Type [E2]
a 1 +
^ value is not a <NumericIntLiteral> or <NumericDecLiteral>
```

#### Invalid Type Cast [E3]

An Invalid Type Cast happens when you try to cast from type to type but there is not an operation where this is possible. Similar to the Wrong Type error, the example below shows how an Identifier that has not been assigned a value, is being attempted to cast to both a NumericDecLiteral with the keyword `dec` and a NumericIntLiteral with the keyword `int`. Since there has been no assignment to the variable `a`, it will not get swapped out for a value, and the type cannot be cast to either.

```
> a dec
Error: Invalid Type Cast [E3]
a dec
  ^^^ Cannot convert <Identifier> to <NumericDecLiteral>

> a int
Error: Invalid Type Cast [E3]
a int
  ^^^ Cannot convert <Identifier> to <NumericIntLiteral>
```

#### Stack Empty [E4]
The Stack Empty error happens when the function or operation that has been called requires more arguments than are currently on the stack. This is an indication that not enough variables where provided. In the example below, a single NumericIntLiteral has been added to the stack and then the Addition operation has been called. This gives and error because the Addition operation requires two arguments.

```
> 1 +
Error: Stack Empty [E4]
```

#### Operation Not Implemented [E5]

The Operation Not Implemented error occurs when a non-identifier symbol has been parsed that has not gotten functionality yet. Symbol here meaning one or many characters (e.g. `foo` or `123`). Since symbols that are not keywords or existing identifiers get read as identifiers (so long they follow the identifier rules), there are few things that trigger the Operation Not Implemented error. The one class of symbols that do cause this error are characters like `#` and `$` which have not been assigned any operation.

```
> #
Error: Operation Not Implemented [E5]

> $
Error: Operation Not Implemented [E5]
```

#### Example of errors

Here is an example of what this might look like in the interactive interpreter.

![image](https://github.com/user-attachments/assets/993e8eb3-0ca2-4a20-8b30-37dd405992bd)

