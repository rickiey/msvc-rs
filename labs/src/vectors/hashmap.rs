use std::collections::HashMap;

pub fn hash_map() {
    let mut hm = HashMap::new();
    hm.insert(String::from("key1"), String::from("value1"));
    hm.insert(String::from("key2"), String::from("value2"));

    for (key, value) in &hm {
        println!("{}: {}", key, value);
    }

    let mut t = vec![String::from("key3"), "key4".to_string(), "343".to_string()];
    let mut  m = vec![21,23];

    let mut h2 :HashMap<_,_> = t.iter().zip(m.iter()).collect();

    println!("{:?}", h2);

    // println!("{:?}", hm.get(&String::from("key5")));

    let i2 = 1;

    for (k,v) in  h2.iter_mut() {
        println!("k: {} v: {}", k, v);
        *v = &i2;
        println!("k: {} v: {}", k, v);
    }

    hm.entry(String::from("key5")).or_insert(String::from("value5"));
    println!("k: {} v: {:?}", hm.get(&String::from("key5")).unwrap(), hm.get(&String::from("key3")));

    let mut m2 = HashMap::new();

    let ss = "h e l l o".to_string();

    for i in ss.split_ascii_whitespace() {
        let count = m2.entry(i.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", m2);

    // HashMap::hasher(&String::from("key1"));
}
