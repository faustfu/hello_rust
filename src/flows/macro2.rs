// 1. Declarative macros could use other macros recursively. Macro pattern rules could use other pattern rules, too.

macro_rules!  hashmap {
  // replace a item to be a unit.
  (@unit $($x:tt)*) => (());
  // calculate the length of unit array.
  (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@unit $rest)),*]));
  // main block
  ($($key:expr=>$value:expr),* $(,)*) => (
    {
      let _cap = hashmap!(@count $($key),*);
      let mut _map = ::std::collections::HashMap::with_capacity(_cap);
      $(
        _map.insert($key, $value);
      )*
      _map // return result for outside usage.
    }
  );
}

pub fn run() {
  let map = hashmap! {
    "a" => 1,
    "b" => 2,
  };
  println!("map is {:?}", map);
}
