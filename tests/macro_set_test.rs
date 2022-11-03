use handy_dandy_macros::set;

use std::collections::HashSet;

#[test]
fn creates_empty_hashset() {
    let set: HashSet<i32> = set!();

    assert_eq!(set.len(), 0);
}

#[test]
fn creates_hashset_with_items() {
    let set = set!(1, 2, 3, 69);

    assert_eq!(set.len(), 4);

    assert_eq!(set, HashSet::from([1, 2, 3, 69]));
}
