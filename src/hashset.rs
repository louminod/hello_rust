use std::collections::HashSet;

pub(crate) fn run() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("gamma");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("delta");
    if added_vega {
        println!("added vega");
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("removed delta");
    }
    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!("is {:?} a subset of {:?} ? {}", _2_8, _1_10, _2_8.is_subset(&_1_10));

    // disjoint
    println!("is {:?} a subset of {:?} ? {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));
}