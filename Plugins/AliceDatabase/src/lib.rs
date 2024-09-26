#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    #[test]
    fn it_works() {
        let result = 2_i32.add(2);
        assert_eq!(result, 4);
    }
}