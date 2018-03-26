#[macro_use]
extern crate log;

pub fn log_a() {
    debug!("a");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
