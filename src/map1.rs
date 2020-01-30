// 1. The type HashMap<K, V> stores a mapping of keys of type K to values of type V.
// 2. If types of key or value are stored in heap, the map will take the ownership.
// 3. If types of key or value are references, the lifetime of key or value should be loger then the map.

use std::collections::HashMap;

pub fn map1() {
  // case 1(init)
  let mut scores = HashMap::new(); // initial an empty map without type definition.

  scores.insert(String::from("Blue"), 10); // first key-value pair will decide type of the map

  scores.insert(String::from("Yellow"), 50);

  println!("map1 is {:?}", scores);

  // case 2(init)
  let mut mapper: HashMap<i32, i32> = HashMap::new(); // initial an empty map with type definition.

  mapper.insert(1, 2);

  println!("map2 is {:?}", mapper);

  // case 3(init)
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let histories: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // initial a map with default pairs by zip method.

  println!("map3 is {:?}", histories);

  // case 4(get by key)
  let team_name1 = String::from("Blue");
  let score1 = scores.get(&team_name1);
  println!("blue score is {:?}", score1); // score's type is option.

  let team_name2 = String::from("Red");
  let score2 = scores.get(&team_name2);
  println!("red score is {:?}", score2); // score's type is option.

  // case 5(for loop)
  for (key, value) in &scores {
    println!("{}: {}", key, value); // types of key or value are same with hash's definition.
  }

  // case 6(insert&update by key)
  scores.insert(String::from("Green"), 10);
  scores.insert(String::from("Green"), 25); // insert more than once will replace it

  println!("map4 is {:?}", scores);

  // case 7(insert if not exists)
  scores.entry(String::from("Green")).or_insert(33); // method:entry will return a Entry.
  scores.entry(String::from("Brown")).or_insert(33); // method:or_insert will insert if not exists or return a mutable reference for the value.

  println!("map5 is {:?}", scores);
}
