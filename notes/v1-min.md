# Minimum Requirements for 1.0.0 Release

While there are many things I hope to implement in Claw, these are the requirements.

## Variable declaration

The 3 core ideas of programming are getting data, settings data, and doing operations with said data. The ability to declare variables is **very important**. I believe the syntax for creating variables should look like this.

```claw
declare Variable {
    Score,
    HighScore,
}
```

This should be able to be scoped into both the stage and sprite. (Focus on global, stage scoped variables first.) To access variables, I was thinking of an enum-like syntax.

```claw
looks::say($Variable::Score);
```

The `$` will differentiate between accessing an enum and a function. There will also a module for working with variables.

```claw
variables::set($Variable::Score, 0);
variables::change($Variable::Score, 2);
```

The `variables::get(variable)` function will also be available, but the `$Variable::Name` is shorthand for that. This works by detecting the argument type. If it needs a variable name, it returns the variable name. If it needs a number, it returns the value of the variable.

## Most of the standard library

For this to be a programming language, most of the standard library features should be available. Not everything, as some are more complicated, but a majority of blocks.

## Creating functions

The difference between Scratch blocks and Claw is that everything is a function. You can create a function that runs when the green flag is clicked, but you can also call that function from other pieces of code.

```claw
@on_start
fn main() {
    looks::say("hi");
    control::wait(1);

    self::alternate();
}

fn alternate() {
    looks::say("there");
    control::wait(1);

    self::main();
}
```

`main` will be called first, and then it and `alternate` will alternate saying "hi" and "there." This solution of "all events are functions" allows multiple functions to have the same event.

Another feature that I eventually want to add are parameters to functions, but for now they are just allowed in standard library blocks.

---

These are all of the features actually necessary for the start of the programming language. I have ideas about incorporating loops, if-statements, syntactic sugar for variables/lists, variable/list defaults, sprite declaration, asset management, and more. The thing is I'm not going to work on these extra bells and whisles until the core, main features are done.
