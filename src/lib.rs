#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

/// Only works if list is sorted over key(T)
/// returns index of element
pub fn fibonacci_search<T: Ord>(list: &[T], elem: T, key: impl Fn(&T) -> f64) -> usize {
    let k = key(&elem);
    let mut l = 0usize;
    let mut r = list.len() - 1;

    loop {
        let u = key(&list[l]);
        let o = key(&list[r]);
        let m = (l as f64 + ((k - u) / (o - u)) * (r - l) as f64).ceil() as usize;

        use std::cmp::Ordering;
        match list[m].cmp(&elem) {
            Ordering::Less => l = m,
            Ordering::Greater => r = m,
            Ordering::Equal => return m,
        };
    }
}
