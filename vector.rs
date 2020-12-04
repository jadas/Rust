pub struct Vec<T> {
    ptr: Box<T>,
    cap: usize,
    len: usize,
}


