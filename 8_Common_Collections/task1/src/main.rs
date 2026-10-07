use std::collections::HashMap;

fn get_median(items: &[i32]) -> i32 {
    if items.len() % 2 == 0 {
        items[items.len()/2]
    } else {
        (items[items.len()/2] + items[items.len()/2+1])/2
    }
}

fn get_map(items: &[i32]) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in items {
        let entr = map.entry(*i).or_insert(0);
        *entr += 1;
    }
    map
}

fn main() {
    let test_nums = vec![1, 2, 3, 4, 5, 5, 5, 6, 7];
    println!("{}", get_median(&test_nums));
    print!("{:#?}", get_map(&test_nums));
}
