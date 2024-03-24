A guessing game. 
Here’s how it works: the program will generate a random integer between 1 and 100. 
It will then prompt the player to enter a guess. 
After a guess is entered, the program will indicate whether the guess is too low or too high. 
If the guess is correct, the game will print a congratulatory message and exit. 


The prelude is the list of things that Rust automatically imports into every Rust program. 
It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single 
Rust program.

If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.


```
[dependencies]
rand = "0.8.5"
```
Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers.   
The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.  
  
Cargo considers these versions to have public APIs compatible with version 0.8.5,   
and this specification ensures you’ll get the latest patch release that will still   
compile with the code in this chapter.   
Any version 0.9.0 or greater is not guaranteed to have the same API as what the following examples use.

When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs   
from the registry, which is a copy of data from Crates.io.  
Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.
After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

Cargo lock file is called Cargo.lock. This file keeps track of the exact versions of dependencies in your project.
When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use   
the versions specified there rather than doing all the work of figuring out versions again.  
This lets you have a reproducible build automatically.   
In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file.  
Because the Cargo.lock file is important for reproducible builds,  
it’s often checked into source control with the rest of the code in your project.

When you do want to update a crate, Cargo provides the command update, which will ignore the Cargo.lock file 
and figure out all the latest versions that fit your specifications in Cargo.toml.  
Cargo will then write those versions to the Cargo.lock file.  
Otherwise, by default, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0.  
If the rand crate has released the two new versions 0.8.6 and 0.9.0, you would see the following if you ran cargo update:

```
$ cargo update
Updating crates.io index
Updating rand v0.8.5 -> v0.8.6
```

To use rand version 0.9.0 or any version in the 0.9.x series, you’d have to update the Cargo.toml 
file to look like this instead:
```
[dependencies]
rand = "0.9.0"
```
