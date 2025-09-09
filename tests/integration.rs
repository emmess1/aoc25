use aoc25::{LinkedList, SimpleHashMap, BstMap};

#[test]
fn integrated_usage_strings_and_ints() {
    // LinkedList usage
    let mut ll = LinkedList::new();
    ll.push_front("c");
    ll.push_front("b");
    ll.push_front("a");
    let gathered: Vec<_> = ll.iter().cloned().collect();
    assert_eq!(gathered, vec!["a", "b", "c"]);

    // HashMap usage
    let mut hm = SimpleHashMap::new();
    hm.insert("alpha".to_string(), 1);
    hm.insert("beta".to_string(), 2);
    assert_eq!(hm.get(&"alpha".to_string()), Some(&1));
    assert_eq!(hm.remove(&"beta".to_string()), Some(2));
    assert!(!hm.contains_key(&"beta".to_string()));

    // TreeMap usage
    let mut tm = BstMap::new();
    tm.insert(10, "ten");
    tm.insert(5, "five");
    tm.insert(15, "fifteen");
    assert_eq!(tm.get(&5), Some(&"five"));
    assert_eq!(tm.remove(&10), Some("ten"));
    assert!(!tm.contains_key(&10));
}

#[test]
fn stress_mix_operations() {
    let mut hm = SimpleHashMap::with_capacity(1);
    let mut tm = BstMap::new();

    for i in 0..200 {
        hm.insert(format!("k{}", i), i);
        tm.insert(i, i);
    }
    for i in 0..200 {
        assert_eq!(hm.get(&format!("k{}", i)), Some(&i));
        assert_eq!(tm.get(&i), Some(&i));
    }
    for i in (0..200).step_by(3) {
        assert!(hm.remove(&format!("k{}", i)) .is_some());
        assert!(tm.remove(&i).is_some());
    }
}
