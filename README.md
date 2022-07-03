# 🧀 **queso**

![license](https://img.shields.io/github/license/queso-lang/queso)
![size](https://img.shields.io/github/languages/code-size/queso-lang/queso)
![helpwanted](https://img.shields.io/github/labels/queso-lang/queso/help%20wanted)

Checkout [develop](../../tree/develop) and feature branches for latest commits!
### What is **queso**?

**queso** is a functional dynamically-typed scripting language that builds on the foundation of existing languages with convenience and quality of life features and tweaks.

**queso** promotes the *everything is an expression notion*, where constructs such as if, while, as well as blocks, have a value.

**queso** will be compiled to WebAssembly, and is supposed to be run in any WASM VM, be it the browser, or a native environment.

Enough with that. Let's see it in practice!

```ts
let filterSpicySalsas = (salsas) -> salsas.>filter(salsa -> salsa.isSpicy);

let salsas = [
    {name: `fresca`, isSpicy: false},
    {name: `roja`, isSpicy: true},
    {name: `habanero`, isSpicy: true},
];

let spicySalsas = salsas |> filterSpicySalsas;

log(
  spicySalsas
    .>map(.name)
    .>sort()
    .>join(`, `)
)
// prints habanero, roja
```

First, we define the function `filterSpicySalsas`. All functions in queso are lambdas.

Inside that function, we see the dot-pipe operator `.>`. This is because `filter` is not actually a method, but rather just a function.

Traditionally, we could represent that same operation with: `filter(salsas, salsa -> salsa.isSpicy)` or with the pipe operator `salsas |> x -> filter(x, salsa -> salsa.isSpicy)`. Thus, the dot-pipe operator `.>` pipes the left operand into the right operand's first argument.

Then, familiarly, we define a list of objects. The value of the `name` key is a string (all strings in queso are multiline and interpolated), while `isSpicy` contains a bool.

We could then do `filterSpicySalsas(salsas)` to retrieve just the salsas with `isSpicy == true`, or simply use the ~~cheese~~ pipe operator, like we did above.

Lastly, from the spicy salsas, we want to print out a sorted, comma-separated string of the salsas' names. And so, we map the list of salsas to their names. This could be done like so: `spicySalsas.>map(salsa -> salsa.name)`. Alternatively, if the expected argument of a function is some predicate, we can use any binary operator, and leave the left operand empty. Queso will pipe the predicate's argument to that operand. The same applies to binary functions, and not only predicates. Take this example of a function which reduces the list:

```ts
let reduce = (list, reducer, initial) -> (
  let accumulator = initial;
  for el in list => (
    accumulator = reducer(accumulator, el)
  );
  accumulator // last value in a block is returned
);

let foo = [1, 2, 3];
// same as foo.>reduce((a, b) -> a + b, 0)
log( foo.>reduce(+, 0) ); // 6

// a more complicated example to show that even the dot-access operator can be used this way:
let traverseKeys = [`buzz`, `yeet`];
let bar = {buzz: {yeet: 123}};
log( traverseKeys.>reduce(., bar) ) // 123
// here, bar is being accessed with the keys specified in traverseKeys
// this is just like writing bar.buzz.yeet
```

Notice how our reduce function uses parentheses `()` to denote a block. This is because while other languages use `()` for grouping expressions to alter the precedence of operations, such as in `a - (b + c)`, queso extends this notion to grouping expressions themselves into lists, just like normal blocks. The last expression in the block will be "returned" as the block's value. The blocks are also full-fledged scopes with the possibility to define and shadow variables.

Coming back to the original example, we use dot-piping to 1. map the salsa objects to just their names, 2. sort the values lexicographically, 3. join them with a comma. We end up with `habanero, roja`.

#### License

Apache 2.0
