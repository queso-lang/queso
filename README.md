# 🧀 **queso**

![license](https://img.shields.io/github/license/queso-lang/queso)
![size](https://img.shields.io/github/repo-size/queso-lang/queso)
![maintained](https://img.shields.io/maintenance/yes/2020)
![issues](https://img.shields.io/github/issues/queso-lang/queso)
![helpwanted](https://img.shields.io/github/labels/queso-lang/queso/help%20wanted)

### What is **queso**?

**queso** is a functional dynamically-typed scripting language that builds on the foundation of existing languages with many **unique** convenience and quality of life features and tweaks. In fact, the driving force behind it was bad design choices in other langs.

**queso** promotes the *everything is an expression notion*, which fits nicely in a functional scripting language. However, declarations (`let`, `mut`, `fn`, `class`) are not considered expressions,
since `if a -> let b` doesn't make sense.
A snippet showcasing some of queso's functional features:

**queso** runs everywhere **C** does. It is an interpreted language and uses a byte code virtual machine to make it silky smooth.

Enough with that. Let's see it in practice!

This is a snippet of some functional aspects of the language:
```rust
// functional programming
fn filterSpicySalsas(salsas): salsas.filter(salsa: salsa.isSpicy);

let salsas = [
    #{name = "fresca", isSpicy = false}, //object literals
    #{name = "habanero", isSpicy = true},
    #{name = "verde", isSpicy = false},
];

trace salsas |> filterSpicySalsas();
// prints [#{name = "habanero", isSpicy = true}] along with the line and filename
```
So, what's going on here? this looks a little different than other langs

**First**, function declaration has a colon `:`. That's because the function body doesn't have to be a block. It just has to be an expression. The salsas.filter(...) is immediately returned. Lambdas are similar, in fact salsa: salsa.isSpicy is an example of one.

When invoked with an array, filterSpicySalsas returns a subarray containining only the salsas that have a field `isSpicy = true`

**Second**, `#{}` is the syntax for object literals. `{}` is way too similar to code blocks (and in fact could be ambiguous in queso).
Why `#`? because object literals are quite similar to **hash** maps.

**Lastly**, there's the ~~cheese~~ pipe `|>` operator, which lets you chain functions and inject the left operand as the right operand's first argument. This is a common tool in functional programming. It lets you convert a bunch of nested expressions into a linear one!
```rust
// instead of this:
e(d(c(b(a))))
// do this:
a |> b() |> c() |> d() |> e()
```
---
### That's just the top of the iceberg!
Look at these beautiful event-driven capabilities of queso:
```rust
// OOP with event-driven capabilities
class DataLoader {
  mut evtLoaded = new Event();
  let endpoint = ~; // null or rather simply nothingness
  static fn :@new(init endpoint): this;
  fn load(): emit evtLoaded -> ..fetch(endpoint);
}
let dl = new DataLoader("http://localhost/data");
dl.load(); // `evtLoaded` has been emitted.

// somewhere
on dl.evtLoaded -> data: trace data; // invoke this lambda when dl.evtLoaded happens

// somewhere else
on evtLoaded -> refreshGUI;
```

*Whoa..* new things. You're greeted with the `mut` keyword which just means a mutable variable. `let` variables cannot be changed after initializing and are the basis of functional programming in queso.

Then there's the `:@new` constructor. We'll get to it later. What you might find interesting is the `init` keyword. That's just syntax sugar for `(endpoint): {this.endpoint = endpoint}`. Are you convinced yet? So many languages lack this obvious feature.

The `load` method is where the real event-driven capabilities shine. When called, it emits the `evtLoaded` Event with some data.
When that happens, it's like a signal being propagated across your code. That signal is captured by the `on` keyword and the data is passed to the function after `->`. Technically speaking, `on` registers a listener on the Event.

If you have a keen eye, you probably noticed the `..` syntax. That's just `await`, but simpler.

## Symbols

Above, we used a `:@new` to construct an instance of `DataLoader`. What `:@` syntax really means is a ***Symbol***.

A symbol is a meta-field that's attached to the variable itself, and not it's value.

Consider this example:

```rust
mut taco = "sloppy joe";
taco:@toString = (): `TACO: $taco`;
trace taco; // prints "TACO: sloppy joe"
taco = "al pastor";
trace taco; // prints "TACO: al pastor" even though the value of taco changed
```

This lets you define fields that are used by the language (and also your own!). Examples of other built-in symbols are:

#### :@iter - iterators
```rust
let cheese = 123;
cheese:@iter: *(): { yield 1; yield 2; } //this is a generator function
for n in foo -> trace n; //1 2
```

#### :@get and :@set - getter and setter functions (everything's a property!)
```rust
mut cheese = 123;
cheese:@get = (it): it * 2;
trace cheese; // 246
cheese:@set = (it, val): it = {trace "cheese has changed! beware.."; val};
cheese = 5;
trace cheese; // cheese has changed! beware.. 10
```

Symbols are a work in progress and the syntax might change. `cheese:#iter` and `cheese#iter` are also some good candidates.

And guess what! There's a heck of a lot more. It's getting a little long for the readme though. Unfortunately, The full-fledged docs are a work in progress, so you'll have to wait. You could also help!

### Contributing

All help is welcome! Let's make this delicious language a reality.

#### License

Apache 2.0
