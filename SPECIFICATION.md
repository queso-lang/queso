### Overview
Queso is a dynamically-typed, general purpose programming language with a focus on some functional programming primitives.
It promotes the everything-is-an-expression notion, where constructs such as `if`, `while`, as well as blocks, have a value.

This specification is subject to change drastically. Possible additions include a type system, syntactic changes, as well as changes to queso's philosophy.

Below, we present some of the features and characteristics of queso.

### Primitives
```rust
1, -3, 5.6 // arbitrarily large and precise numbers
`singleline`, `
  multiline {foo}
` // immutable, multiline interpolated strings
[], {} // lists, objects
arg -> retVal // functions
false, true // bools
null
```

```
// single-line comments will be supported
```

### Operators
```rust
// arithmetic (numbers only, no implicit conversions)
1 + 2
1 - 2
1 * 2
1 / 2
4 ** 5 // exponentiation
// no modulus, no bitwise operators. Use std functions.

// assignment
foo = 2
foo += 2
foo -= 2
foo *= 2
foo /= 2
foo **= 2

!true
3 > 2
3 >= 2
2 < 3
2 <= 3
2 != 3
3 == 3

`foo` ++ `bar` // string concatenation

[*a, *b], {*a, *b, -removedProperty} // spread, remove property operators

foo.bar // dot-access, dereferencing
foo.bar[5] // array access
foo[`bar`] // computed access
foo.bar() // invocation
foo?.bar?.buzz?.() // optional chaining

c(b(a(123)), 456)
// is equivalent to:
123 |> a |> b |> x -> c(x, 456) // the pipe operator.
// It's still undecided whether the implementation should be similar to F# or Hack.

let count = (list, predicate) -> filter(list, predicate).len
[1, 2, 3, 4].>count(x -> x > 3) // 1
// this is the pipe-access operator. Similar to extension methods.
// It pipes the left operand into the righ operand's (which has to be a function) first argument.
// notice that reduce(list, ...) could also be written as list.>reduce(...)

// explicit conversions
!!0 // to bool. Note: there is no `!!` operator, rather two `!` operators chained.
`{123}`, ``123 // to string. Second syntax is under consideration.
+`123` // to number
```

### Blocks
Queso adds special meaning to the standard `()` grouping operator.
While other languages use it for grouping expressions to alter the precedence of operations, such as in `a - (b + c)`, queso extends this notion to grouping expressions themselves into lists, like normal blocks.
The last expression in the block will be "returned" as the block's value.
The blocks are also full-fledged scopes with the possibility to define and shadow variables.

```rust
let myFavoriteSalsas = (
  let allSalsas = [{name: `fresca`, isFavorite: true}, {name: `blanca`, isFavorite: false}, {name: `crema`, isFavorite: true}];
  allSalsas.filter(salsa -> salsa.isFavorite)
).map(salsa -> salsa.name)
// [`fresca`, `crema`]
```

### Functions
All functions in queso are lambdas and are first-class citizens. There are no function declarations. Instead, simply assign a lambda to a variable.
The return value can be anything, including a block (similar to function declarations in other languages), or any other value.
```rust
arg -> retValue; // one argument
-> retValue; // no arguments
(arg1, arg2) -> retValue // multiple arguments

let getUsers = async -> (
  let users = ...fetch(`GET`, `/users`);
  users.map(user -> {*user, -password})
) // async, block body
```

### Lists, objects
Lists can contain elements of different types.
```rust
[1, 2, 3]
[`foo`, 123, true]
```
Objects can use the following shorthand:
```rust
let foo = 123;
log({foo}) // {foo: 123}
```

Both lists and object can leverage the spread operator `*`. Objects can use the `-` operator to remove properties.

### Loops
Loops are expressions as well and return a value, which is an array of all the values returned by the body of every iteration of the loop.

#### for..in
```rust
let salsas = for salsaName in [`fresca`, `crema`] => `salsa {salsaName}`
log(salsas) // [`salsa fresca`, `salsa crema`]
```
This is similar to list comprehensions in languages such as python.

#### for range-based
```rust
for i in range(0, 5) => i ** 2; // [0, 1, 4, 9, 16]
// a dedicated syntax for ranges, such as 0..5 will be considered
```

#### while
```rust
mut counter = 0;
while conter < 5 => (
  log(counter);
  counter += 1
)
```

### Conditionals
As of now, queso does not have a dedicated `if` statement. Instead, you can use the binary/ternary conditional operator.
The operator acts as a traditional ternary operator (`a ? b : c`) or as a binary operator when the third operand is omitted (`a ? b`)

```rust
condition ? ifTrue;
condition ? ifTrue : ifFalse;
A ? ifATrue : B ? ifBTrue : bothFalse;

user.preferredCuisine == `mexican` ? (
  log(`🎉`)
) : (
  log(`😔`)
)
```

