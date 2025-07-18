pub mod bucket;
pub mod node;
pub mod pobj;
pub mod traits;

use crate::pobj::Pobj;

fn main() {
    let mut iter = iter_test();
    // for _ in 1..100 {
    //     if let Some(res) = iter.next() {
    //         println!("{}", res);
    //     }
    // }

    let table = Pobj::new();
    let _ = table.put("first", 1);
    let _ = table.put("second", "hello!");
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
