extern "C" {
    fn foo(val1: usize, val2: usize) -> usize;
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn it_works() {
        let result = unsafe { foo(2, 14) };
        assert_eq!(result, 28);
    }
}
