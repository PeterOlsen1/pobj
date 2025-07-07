pub mod bucket;
pub mod node;
pub mod table;

// use crate::table::Table;

fn main() {
    let mut iter = iter_test();
    for _ in 1..100 {
        if let Some(res) = iter.next() {
            println!("{}", res);
        }
    }
}

///testing iterator
fn iter_test() -> impl Iterator<Item = i32> {
    let mut num = 1;
    std::iter::from_fn(move || {
        let res = num;
        num += 1;
        Some(res)
    })
}
