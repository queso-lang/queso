# 🧀 **queso**

![license](https://img.shields.io/github/license/queso-lang/queso)
![size](https://img.shields.io/github/languages/code-size/queso-lang/queso)
![helpwanted](https://img.shields.io/github/labels/queso-lang/queso/help%20wanted)

Checkout [develop](../../tree/develop) and feature branches for latest commits!

See the [SPECIFICATION](./SPECIFICATION.md) for a more concrete definition of the language.

### What is **queso**?

**queso** is a general-purpose, dynamically-typed, safe, and immutable scripting language with a focus on functional programming. **queso** builds on the foundation of existing languages with convenience and quality of life features and tweaks.

**queso** promotes the *everything is an expression notion*, where constructs such as if, while, as well as blocks, have a value.

**queso** will be compiled to WebAssembly, and is supposed to be run in any WASM runtime, be it the browser, or a native environment.

Enough with that. Let's see it in practice!

```ts
let filterSpicySalsas = salsas -> salsas.>filter(salsa -> salsa.isSpicy);

let salsas = [
    {name: `fresca`, isSpicy: false},
    {name: `roja`, isSpicy: true},
    {name: `habanero`, isSpicy: true},
];

let spicySalsas = salsas |> filterSpicySalsas;

log(
  spicySalsas
    .>map(_.name)
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

Lastly, from the spicy salsas, we want to print out a sorted, comma-separated string of the salsas' names. And so, we map the list of salsas to their names. This could be done like so: `spicySalsas.>map(salsa -> salsa.name)`.

In this case however, we can use special semantics, which come from the fact that operators in queso are functions themselves. Moreover, we can use the placeholder `_` keyword to easily create curried functions. For instance, `let sum = (a, b) -> a + b` can be curried like so: `let sumWithFive = sum(5, _)`. This is equivalent to writing `let sumWithFive = b -> sum(5, b)`.

Thus, `.>map(_.name)` is equivalent to `.>map(salsa -> salsa.name)`. Notice that this creates a unary function, but we can just pass the operator itself without placeholders if we are epxected to provide a binary function. Take this example of a function which reduces a list:

```ts
let reduce = (list, reducer, initial) -> (
  mut accumulator = initial;
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

Let's jump in to a real-world example of a web server in queso:

```ts
// userService.queso
import orm => repos;
export let getUserById = id ...-> (
  let users = ...repos.users.getOne({where: {id}});
  {++user, -password}
)

// middleware.queso
export let adminGuard = (ctx, next) ...-> ctx.state.user.role == 'admin' ? ...next() : throw {type: 401}; 

// userRouter.queso
import ./userService => getUserById;
import ./middleware => adminGuard;
import web => createRouter;

export let router = createRouter();

router.GET(`/user/:id`, adminGuard, ctx ...-> (
  [ctx.request.body.id, ctx.state.user] |> [id, user]
    -> id == user.id ? user : ...getUserById(id)
));
```

So, right off the bat, we get a look at the module system. We define three modules (a file is a module) with their respective exports and imports. All exports are named.

In the first file, we import some theoretical ORM library. Then we define a function to be used later on in our web server to fetch a user by their id. The function is asynchronous, which is indicated by the async `...->` operator. This means you can use the await `...` operator inside of the function. Here, we're calling an import from the ORM library, and then awaiting the returned Promise.

Once we have that user, we want to return it, but remove the `password` property, for security reasons. We create a new object, then spread that original user object (spreading means copying all key:value pairs) with the concatenation `++` operator, and lastly remove the `password` key using the `-` operator. Recall that the last expression in a block will be returned, so we don't need to use the `return` keyword explicitly.

In the second file, we define a small utility function for checking whether the user is authorized to access our endpoint.

Lastly, in our main file, we import the functions from the two other files, as well as a function for creating a router object from some theoretical web server library. We create the router (almosts like instantiation), then define one route with the middleware and the route handler. If the requested user is the current user, we just return the user object which already sits in our `ctx`. Otherwise, we use our `getUserById()` function by awaiting it.

#### Standard Library
Queso provides a flexible system for basic behavior and the ability to swap the standard library with your own implementation that tailors best to your needs. This is because queso does not implement any methods on the built-in primitives, rather it provides a `core` module for basic functions. For instance, let's say the native function for finding an element in a list was not flexible enough for you:

```ts
// this is imported implicitly, but can be disabled entirely
import core => find;
log( [1, 2, 3].>find(_ > 1) ) // prints the element 2, but what if you wanted the index too?

let find = list, predicate ->
  for i in range(list) =>
    list[i] |> el -> predicate(el) ? return [el, i] : continue
  else => [null, -1]

log ( [1, 2, 3].>find(_ > 1) ) // prints [2, 1]
```

#### License

Apache 2.0
