use aoc25::*;

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

#[test]
fn integration_more_ds() {
    // DoublyLinkedList
    let mut dl = DoublyLinkedList::new();
    dl.push_back(1);
    assert_eq!(dl.pop_back_cloned(), Some(1));

    // ArrayList
    let arr = ArrayList::from_iter([1, 2, 3]);
    assert_eq!(arr.len(), 3);

    // HashSetExt
    let mut hs = HashSetExt::new();
    hs.insert(1);
    assert!(hs.contains(&1));

    // Coords / Points
    let p = Point::new(0, 0);
    let _ = p.neighbors4();

    // SparseGrid
    let mut sg = SparseGrid::new();
    sg.insert(Point::new(1, 1), 9);
    assert_eq!(sg.get(&Point::new(1, 1)), Some(&9));

    // Neighbor deltas
    assert_eq!(DELTAS4.len(), 4);

    // Stack / Queue / Deque
    let mut st = Stack::new();
    st.push(7);
    assert_eq!(st.pop(), Some(7));
    let mut q = Queue::new();
    q.push(5);
    assert_eq!(q.pop(), Some(5));
    let mut dq = Deque::new();
    dq.push_front(1);
    dq.push_back(2);
    assert_eq!(dq.pop_front(), Some(1));

    // Heaps
    let mut minh = MinHeap::new();
    minh.push(3);
    minh.push(1);
    assert_eq!(minh.pop(), Some(1));
    let mut maxh = MaxHeap::new();
    maxh.push(1);
    maxh.push(2);
    assert_eq!(maxh.pop(), Some(2));

    // Graph adjacency + Topo
    let mut adj = Adjacency::new();
    adj.add_edge(1, 2);
    let order = topo_sort(&[(1, 2), (2, 3)]).unwrap();
    assert_eq!(order.first(), Some(&1));

    // Disjoint Set Union
    let mut ds = DisjointSet::new();
    ds.union(1, 2);
    assert!(ds.connected(1, 2));

    // Intervals
    let a = Interval::new(1, 3);
    let b = Interval::new(2, 4);
    assert!(a.overlaps(&b));

    // BitMask
    let mut bm = BitMask::new();
    bm.set(3);
    assert!(bm.test(3));

    // MonotonicQueue
    let mut mq = MonotonicQueueMin::new();
    mq.push(3);
    mq.push(1);
    assert_eq!(mq.min(), Some(1));

    // DenseGrid2D
    let g = DenseGrid2D::new(2, 2, 0);
    assert!(g.in_bounds(1, 1));

    // IndexedMinHeap
    let mut ih = IndexedMinHeap::with_items(3);
    ih.set(0, 5);
    ih.set(0, 1);
    let _ = ih.pop_min();

    // FreqMap
    let mut fm = FreqMap::new();
    fm.inc('a');
    assert_eq!(fm.get(&'a'), 1);

    // String algorithms
    let occ = kmp_search("ababa", "aba");
    assert_eq!(occ[0], 0);
    let z = z_function("aaaa");
    assert_eq!(z[1], 3);
    let rh = RollingHash::new("abc", 911382323, 972663749);
    assert_eq!(rh.hash(0, 2), rh.hash(0, 2));

    // SCC (Tarjan)
    let mut g2 = vec![vec![]; 3];
    g2[0].push(1);
    g2[1].push(0);
    g2[1].push(2);
    g2[2].push(1);
    let comps = tarjan_scc(&g2);
    assert!(!comps.is_empty());

    // Fenwick
    let mut fw = Fenwick::new(4);
    fw.add(2, 7);
    assert_eq!(fw.sum_prefix(2), 7);

    // Search helpers and Parsing
    let n = 4;
    let mut uadj = vec![vec![]; n];
    uadj[0] = vec![1, 2];
    let dist = bfs_distances(n, &uadj, 0);
    assert_eq!(dist[1], 1);
    let pre = dfs_preorder(n, &uadj, 0);
    assert!(pre.contains(&1));
    let mut wadj = vec![vec![]; n];
    wadj[0].push((1, 2));
    let (_d, _p) = dijkstra_indexed(n, &wadj, 0);
    let h = |_u: usize| 0;
    let _ = astar_indexed(n, &wadj, 0, 1, &h);
    let grid = parse_grid_chars("ab\n");
    assert_eq!(grid[0].len(), 2);
    let ints = parse_ints_whitespace("1 -2");
    assert_eq!(ints[1], -2);
}
