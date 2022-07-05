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

### Variables
Immutable variables are the basis of functional programming in queso.
```rust
let immutableVariable = 1;
immutableVariable = 2; // err! (compile-time)
mut mutableVariable = 3;
mutableVariable = 4; // ok!
```

<details>
  <summary>
    <h3>Operators</h3>
   </summary>
  
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
  foo ||= bar
  foo &&= bar
  foo ??= bar

  !true
  3 > 2
  3 >= 2
  2 < 3
  2 <= 3
  2 != 3
  3 == 3

  true && true
  false || true

  // short-circuiting
  false || 123
  `foo` && `bar`

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

  let countIf = (list, predicate) -> filter(list, predicate).len
  [1, 2, 3, 4].>countIf(x -> x > 3) // 1
  // this is the pipe-access operator. Similar to extension methods.
  // It pipes the left operand into the right operand's (which has to be a function) first argument.
  // notice that filter(list, ...) could also be written as list.>filter(...)

  // explicit conversions
  !!0 // to bool. Note: there is no `!!` operator, rather two `!` operators chained.
  `{123}`, ``123 // to string. Second syntax is under consideration.
  +`123` // to number

  // async/await, throw, catch
  ...fetch() // await
  throw {}
  ...fetch() catch err -> log(err)
  ```
  
</details>

### Blocks
Queso adds special meaning to the standard `()` grouping operator.
While other languages use it for grouping expressions to alter the precedence of operations, such as in `a - (b + c)`, queso extends this notion to grouping expressions themselves into lists, like normal blocks.
The last expression in the block will be "returned" as the block's value.
The blocks are also full-fledged scopes with the possibility to define and shadow variables.

```rust
let myFavoriteSalsas = (
  let allSalsas = [{name: `fresca`, isFavorite: true}, {name: `blanca`, isFavorite: false}, {name: `crema`, isFavorite: true}];
  allSalsas.filter(salsa -> salsa.isFavorite)
).>map(salsa -> salsa.name)
// [`fresca`, `crema`]
```

This is similar to languages such as Rust. However, what makes it different is that it is considered an error to include a semicolon after the last statement. So, to not return anything from a block, you must state it explicitly. While it could be justified in a statically-typed language such as Rust, forgetting a semi in a language such as Queso could drastically alter the program behaviour. Thus:

```ts
// ✔️
log(
  (
    doSomething();
    null
  )
) // null

// ❌
log(
  (
    doSomething();
  )
) // does not compile. Either remove the trailing semi (and return `doSomething()`), or add the null
```

### Functions
All functions in queso are lambdas and are first-class citizens. There are no function declarations. Instead, simply assign a lambda to a variable.
The return value can be anything, including a block (similar to function declarations in other languages), or any other expression.
```rust
arg -> retValue; // one param
-> retValue; // no params
(arg1, arg2) -> retValue // multiple params

let getUsers = ...-> (
  let users = ...fetch(`GET`, `/users`);
  users.map(user -> {*user, -password})
) // async, no params, block body
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

### async/await programming
Queso uses an async/await syntax similar to JavaScript.

An asynchronous task is represented by a Promise object.

This object is in the form of `type Promise<T> = {then: (callback: (resolvedValue: T) -> ()) -> self, catch: (err) -> ()}`.

To get the promised value, one could call `promise.then(val -> log(val))`, or preferrably, use the await syntax `log(...promise)`.

To use this syntax, the wrapping function must also be marked as async. The full example:

```ts
let getUserNames = ...-> (
  for user in ...fetch(`GET`, `/users`) => user.name
);

let someWhereElse = (arg1, arg2) ...-> (
  log(...getUserNames());
)
```

### throw and try..catch
A full example of a typical throw and try..catch usage:
```ts
// an example of a function which can throw
let assertAllUserNamesNonEmpty = () ...-> (
  let users = ...fetch(`GET`, `/users`);
  !!(
    for user in users => user.name.len ? user.name : throw {type: `emptyUserName`} // the thrown value is just an object
  ).len
);

let someWhereElse = () ...-> (
  // catch errors from a single async function
  // here, catch acts as a binary operator with the right operand being a callback function
  ...assertAllUserNamesNonEmpty() catch err -> log(err);
  
  // this is more alike to traditional try..catch blocks with multiple statements
  (
    foo();
    bar()
  ) catch err -> (
    if err.type == `notFound` => rethrow; // shorthand for "throw err"
    log(err)
  )
)
```

Catches can also be used in series, as well as return a value:
```ts
let data =
  ...fetch(`mainUrl`)
  catch -> ...fetch(`fallbackUrl1`) // notice the lack of parameters in the catch callback, since we don't use the err object
  catch -> ...fetch(`fallbackUrl2`)
  catch -> []; // if everything fails, maybe return some fallback value.
  // notice that omitting the last catch is synonymous to writing `catch -> rethrow`
```

### OOP?
Queso puts an emphasis on paradigms other than OOP. Thus, in the current language design iteration, there are no traditional classes or inheritance. However, it does implement some OOP ideas, such as modules, which are similar to static classes.

### Modules
Each file is a module. Modules are singletons. A module can have exports (named values -- variables), as well as imports (which are the exports from other modules).
```ts
// A.queso
export let utils = {
  countIf = (list, predicate) -> filter(list, predicate).len;
}

// B.queso
import ./A => utils;

log([1, 2, 3, 4].>utils.countIf(x -> x > 3)) // 1
```

### Immutability
All data structures in queso are immutable. By mutating a data structure, you're creating an entirely new one instead. On the other hand, variables can be immutable or mutable. This allows for the usage of common and familiar patterns (such as counting through iteration), while still maintaining an immutable data structure. 

```ts
let foo = [1, 2, 3];
foo = [4, 5, 6]; // ❌ can't mutate the variable!
foo[0] = 7; // ❌ can't mutate the data structure either!

mut common = [1, 2]
common = [3, 4] // ✔️ can mutate the variable
common[0] = 5 // ❌ still can't mutate the data structure itself!

// an example of OOP mutable programming in queso
// this works thanks to late binding
let createCounter = -> (
  mut counter = 0;
  
  let increment = -> counter += 1;
  
  let get = -> counter;
);

let counter = createCounter();
counter.increment();
log( counter.get() ) // 1

```
