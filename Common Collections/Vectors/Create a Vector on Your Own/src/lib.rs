pub fn create_array() -> [i32; 4] {
    let a = [10, 20, 30, 40]; // a plain array
    a
}

pub fn create_vector() -> Vec<i32> {
    let v = vec![create_array()[0], create_array()[1], create_array()[2], create_array()[3]];
    v
}

