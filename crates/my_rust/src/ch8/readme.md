These three options—modules in their own file, 
modules in their own directory with a mod.rs, 
and modules in their own file with a supplementary directory containing submodules—
give the module system enough flexibility to support almost any project structure you might desire.


To resolve the ambiguity, Rust has a special kind of path called an absolute path, start‐ ing with ::, which always refers to an external crate. To refer to the Pixels type in the image crate, you can write:
use ::image::Pixels; // the `image` crate's `Pixels` 
To refer to your own module’s Sampler type, you can write:
use self::image::Sampler; // the `image` module's `Sampler`

std::prelude::v1 is the only prelude that is ever imported automati‐ cally. Naming a module prelude is just a convention that tells users it’s meant to be imported using *.


### 
Use constants for magic numbers and strings in your code. Use statics for larger amounts of data, or any time you need to borrow a reference to the constant value.


## test

Functions marked with #[test] are compiled conditionally.
A plain cargo build or cargo build --release skips the testing code. But when you run cargo test, 
Cargo builds your program twice: once in the ordinary way and once with your tests and the test harness enabled.
This means your unit tests can live right alongside the code they test, accessing internal implementation details if they need to, and yet there’s no run-time cost.