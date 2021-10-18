use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn progress<T, Iter>(iter: Iter, f: fn(T) -> ())
where Iter: Iterator<Item = T> {

    let mut i = 1;
    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f (n);
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];
    progress(v.iter(), expensive_calculation);

    use std::collections::HashSet;
    let mut h = HashSet::new();
    h.insert(0);
    progress(h.iter(), expensive_calculation);
}
