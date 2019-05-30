### Chapter 4

Some of the core/critical components and parts of understading "why" Rust is unique (and awesome!) is found here

#### Ownership

The key takeaway here is anything found within the `{}` is "in scope" and valid and goes "out of scope" at the closing of the function

```
{
  my_awesome_thingy = "in_scope";
}
```
#### `String` type

 - https://doc.rust-lang.org/std/string/struct.String.html


 Strings **can** mutated, and this happens specifically via the following

 - The memory must be requested from the operating system at runtime.

   `String::from`

 - We need a way of returning this memory to the operating system when we’re done with our String (scope).


 ```
 {
    let s = String::from("hello");

}                                  
 ```

 #### References

 - https://doc.rust-lang.org/book/appendix-02-operators.html


 `&something` _refers_ to the value of `something` but doesnt _own_ it

 For someone new to this language, its important to understand that the compiler is basically "protecting" us from doing something that could cause a serious bug, security vulnerablity, data race, or overall just something that could come back and "bite" us in production


  - At any given time, you can have either one mutable reference or any number of immutable references.

  - References must always be valid.


  #### Slice Type

  Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection

   - https://doc.rust-lang.org/std/str/

   #### Summary

   Rust is again helping us out by not having to "worry", at least too much, about GC and memory safety. This is done "automatically" for us, as long as we structure our code correctly, and, as always, the compiler will tell us "up front" before the code is even running out in the wild!
