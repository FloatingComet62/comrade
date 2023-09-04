<div align="center">

![Comrade](/Comrade.svg)

<h3>Comrade</h3>
A flexible language transpiled to C
<br>
</div>

# Overview

Comrade is a programming language which transpiles to [`C`](<https://en.wikipedia.org/wiki/C_(programming_language)>)

<bold><font color="red">Comrade is not production ready</font></bold>

# Hello World

Get used to arrows. Sorry C++ devs<br>
`->` is objectively better than `::`<br><br>
Also no semicolons

```rust
include std->io

fun main(_argc -> i32, _argv -> str[]) => u8 {
  io->out("Hello World")
  return 0
}
```

# Declaration

JS devs will have a good time here

`let` creates a mutable variable<br>
`const` creates an immutable constant

```rust
let mutable = 0
const immutable = 1
```

# Naming

ASCII letters, underscores and numbers but the very first character can't be a number.

```rust
const test = 15   // ok
const test2 = 16  // ok
const _2test = 17 // ok
const 2_test = 18 // not ok
const 2test = 19  // not ok
```

# Lists

```rust
include std->io

const list  = [1, 2, 3, 4, 5] // ok
const list2 = 1, 2, 3, 4, 5   // ok
const list3 = 1 2 3 4 5       // ok

io->out(list[0]) // 1
```

<b>Technical Note: </b> This is an accidental feature

# Loops

While statements are simple, they take a true false statement<br>
No braces needed

```rust
include std->io

while true {
    io->out("Hello World")
}
```

Iterators can be iterated over with the `in` keyword<br>
If you define the variable as mutable, then if you change the value, the value of the item in the iterator will also change.<br>

```rust
include std->io

let iterator = 1 2 3 4 5

for let value in iterator {
    value += 1
}

io->out(iterator) // 2 3 4 5 6
```

Instead of semicolon seperating parts, use `/`

```rust
include std->io

for let i -> i32 = 0 / i < 10 / i++ {
    io->out(i)
}
```

# Branches

if and else if are nothing special

```rust
include std->io

age = 16
if age >= 18 {
    io->out("Here is your beer")
} else if age >= 13 {
    io->out("Here is your pepsi")
} else {
    io->out("Here is your orange juice")
}
```

match statements exist

```rust
include std->io

match 5 > 4 {
  true => io->out("TRRUUUUUUU")
  false => io->out("NAHHHHH")
  default => io->out("Impossible")
}
```

# Functions

The syntax for function is

```rust
fun greater_than(x -> i32, y -> i32) => bool {
    if x > y {
        return true
    }
    return false
}
```

Functions can be mutable, meaning their internal code can be modified<br>
But the function signature can't be changed

```rust
fun let mutable_function() => i32 {
    let x = 5
    return x
}
fun const immutable_function() => bool {
    return false
}
fun immutable_by_default_function() => bool {
    return true
}

io->out(mutable_function()) // 5

fun let mutable_function() => i32 {
    let x = 4
    return x
}

io->out(mutable_function()) // 4

// doesn't work
fun let mutable_function() => u32 {
    let x -> u32 = 4
    return x
}

// doesn't work
fun const immutable_function() => bool {
    return true
}
```

# String Interpolation

C style format is used for now, f-strings might be implemented later

```rust
include std->io
include std->string

let res = ""
io->out("Enter word: ")
io->in(res)
io->out(string->format("%s", res))
```

# Types

```rust
let s = "" // implicitly "str"
let x = 5 // implicitly "i32"
let y -> u32 = 18
let z -> str[] = "Hello" "World"
let xx -> u8[] = 0 128 255
let yy -> i8[] = -128 0 127
```

Reserved keywords for types

```rust
    u4 u8 u16 u32 u64 u128
    i4 i8 i16 i32 i64 i128
    f4 f8 f16 f32 f64 f128
    str bool
```

# Importing and Exporting

src/file.cmr

```rust
include std->math

fun calc(x -> i32, y -> i32) => f32 {
    return math->sin(x) + math->cos(y)
}

fun calc2(x -> i32, y -> i32) => f32 {
    return math->sin(2 * (x + y))
}
```

src/file2.cmr

```rust
fun __init__() {
    include weird_namespace->weird_lib
}

fun test() {
    weird_lib->weird_function()
}
```

src/main.cmr

```rust
include src->file
include src->file2

fun main() => i32 {
    // std->math doesn't exist here
    // weird_lib does exist here

    file->calc(14, 78)
    file->calc(14, 78)

    weird_lib->weird_function2() // works
    math->cos(15) // doesn't work

    file2->test() // works

    return 0
}
```

# ExternC

Comrade allows you to directly access `C`

```rust
fun x() => i32 {
    return 0
}

externC {
    #include <stdio.h>

    int main() {
        printf("Hello World");

        return x();
    }
}
```

# Borrow Checker

Borrow checker, same as rust

# Classes

Classes are bullshit

# Traits

Rust inspired traits

```rust
trait Vehicle {
    fun drive(&mut self)
    fun honk(&self) -> !None
}
```

# Structs

Plain old structs

```rust
struct Truck {
    name -> str
    capactity -> u8
    on_road -> bool
}
```

# Implementations

Rust inspired implementation system

```rust
import std->io

implement Vehicle for Truck {
    drive(&mut self) {
        self->on_road = true
    }
    honk(&self) -> !None {
        io->out("HONK")
    }
}
implement Truct {
    fun set_capacity(&mut self, new_capacity) {
        self->capacity = new_capacity
    }
}
```

# Enums

```rust
import std->io

enum HumanEmotions {
    Happy Sad Angry Fart Jealous
}
fun main() => i32 {
    let emotion = HumanEmotions->Fart
    io->out(emotion) // 3
    return 0
}
```

# Macros

Rust inspired macros

```rust
import std->io
import std->string

macro test {
  compile {
    // code ran at compile time
    () => {
        io->out("compiling test macro")
    }
    (arg: i32) => {
        io->out(string->format("compiling test macro with %d", arg))
    }
  }
  main {
    () => {
        2 + 2
    }
    (arg: ) => {
        2 + arg
    }
  }
}
```
