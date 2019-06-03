### Chapter 6


Enumerations, or enums, are another way of defining a type with all its possible values


#### 6.1 - `defining an enum`

An enum is a simple as the following

```
enum IpAddr {
  V4(String),
  V6(String),
}
```

Once an enum is defined, we can use it like the following

```
let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

The `Option` enum is a special type of enum, used to help define a value as either something or set to nothing.

- https://doc.rust-lang.org/std/option/index.html

This is extremely useful because in Rust, there is no `Null` value - the value itself is either present or absent

```
enum Option<T> {
    Some(T),
    None,
}
```

#### 6.2 - `match`!

 - https://doc.rust-lang.org/rust-by-example/flow_control/match.html


 The `match` keyword in Rust is a way to compare a value against a pattern and perform code against the intended results

 The book uses the following example

 ```
 enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}```

Its important to remember that matches are exhaustive, meaning we need to code in every last possiblity - this is where the `None` value comes into play, as well as the `_` (underscore) pattern

```

let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

#### 6.3 - `if let`

A concise way to handle a pattern match and ignore the others

```
if let Some(3) = some_u8_value {
    println!("three");
}
```
