#[macro_use]
extern crate log;

pub fn log_b() {
    debug!("b");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
