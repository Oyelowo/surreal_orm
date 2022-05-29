pub mod authentication;
pub mod error_handling;
mod macros;
pub mod my_time;
mod util_module_alternative;
pub mod utils;

pub use macros::{calculator, helpers};
pub use util_module_alternative::greeter_alt::alt_good_morning;
pub use utils::{good_morning, local_function, maths};

#[macro_use]
// extern crate ;
#[cfg(test)]
mod tests {
    #[test]
    fn test_adder() {
        assert_eq!(super::sum!(55, 5), 60);
    }
}
