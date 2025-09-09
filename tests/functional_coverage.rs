use data_structures::{LinkedList, SimpleHashMap, BstMap, fcov};
use std::hash::{Hash, Hasher};

#[test]
fn functional_coverage_end_to_end() {
    // LinkedList behaviors
    let mut ll = LinkedList::new();
    fcov::hit("ll_empty_new");
    assert!(ll.is_empty());
    ll.push_front(1);
    ll.push_front(2);
    ll.push_front(3);
    assert_eq!(ll.pop_front(), Some(3));
    assert_eq!(ll.pop_front(), Some(2));
    assert_eq!(ll.pop_front(), Some(1));
    assert_eq!(ll.pop_front(), None);
    fcov::hit("ll_push_pop_order");
    ll.push_front(10);
    ll.clear();
    assert!(ll.is_empty());
    fcov::hit("ll_clear_idempotent");

    // SimpleHashMap behaviors
    let mut hm = SimpleHashMap::with_capacity(2);
    assert_eq!(hm.insert("a", 1), None);
    fcov::hit("hm_insert_new");
    assert_eq!(hm.insert("a", 2), Some(1));
    fcov::hit("hm_update_existing");
    assert_eq!(hm.remove(&"missing"), None);
    fcov::hit("hm_remove_missing");

    // Collisions
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct Collide(u64);
    impl Hash for Collide {
        fn hash<H: Hasher>(&self, state: &mut H) { 0u64.hash(state); }
    }
    let mut hm2 = SimpleHashMap::with_capacity(4);
    for i in 0..8 { hm2.insert(Collide(i), i as i32); }
    for i in 0..8 { assert_eq!(hm2.get(&Collide(i)), Some(&(i as i32))); }
    fcov::hit("hm_collision_chain");

    // Resize by exceeding load factor
    let mut hm3 = SimpleHashMap::with_capacity(2);
    for i in 0..64 { hm3.insert(i, i); }
    for i in 0..64 { assert_eq!(hm3.get(&i), Some(&i)); }
    fcov::hit("hm_resize");

    // BstMap behaviors
    let mut bst = BstMap::new();
    // insert
    for k in [5, 3, 7] { bst.insert(k, k*10); }
    fcov::hit("bst_insert");
    // get_mut miss left
    assert!(bst.get_mut(&2).is_none());
    fcov::hit("bst_get_mut_miss_left");
    // get_mut miss right
    assert!(bst.get_mut(&9).is_none());
    fcov::hit("bst_get_mut_miss_right");

    // remove leaf
    bst.insert(2, 20);
    assert_eq!(bst.remove(&2), Some(20));
    fcov::hit("bst_remove_leaf");

    // remove one-left-child
    let mut bst2 = BstMap::new();
    for k in [5, 3, 2, 7] { bst2.insert(k, k); }
    assert_eq!(bst2.remove(&3), Some(3));
    fcov::hit("bst_remove_one_left");

    // remove one-right-child
    let mut bst3 = BstMap::new();
    for k in [5, 3, 4, 7] { bst3.insert(k, k); }
    assert_eq!(bst3.remove(&3), Some(3));
    fcov::hit("bst_remove_one_right");

    // remove two-children, immediate successor (right child)
    let mut bst4 = BstMap::new();
    for k in [5, 3, 6, 2, 4, 7] { bst4.insert(k, k); }
    assert_eq!(bst4.remove(&5), Some(5));
    fcov::hit("bst_remove_two_children_immediate_succ");

    // remove two-children, successor has right child (strings)
    let mut bst5: BstMap<String, i32> = BstMap::new();
    for k in ["5", "3", "7", "2", "4", "6", "6a"] { bst5.insert(k.to_string(), 1); }
    assert_eq!(bst5.remove(&"5".to_string()), Some(1));
    fcov::hit("bst_remove_two_children_succ_with_right");

    // Final assertion: all expected behaviors hit
    if !fcov::all_hit() {
        let missing = fcov::missing();
        panic!("Functional coverage missing: {:?}", missing);
    }
}

