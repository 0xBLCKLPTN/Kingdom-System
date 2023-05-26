use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DefaultResponse<'aa> {
    pub status: &'aa str,
    pub message: &'aa str,
}


fn main() {
    DefaultResponse { status: "Hello", message: "world!"};
    println!("Hello World!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn my_test() {
        DefaultResponse { status: "Hello", message: "World!"};
    }
}
