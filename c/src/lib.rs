extern crate a;
extern crate b;

pub use a::log_a;
pub use b::log_b;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
