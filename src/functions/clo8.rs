// 1. Use struct to simulate a readonly-borrowed closure.

struct Pick<F> {
  data: (u32, u32),
  func: F,
}
impl<F> Pick<F>
where
  F: Fn(&(u32, u32)) -> &u32,
{
  fn call(&self) -> &u32 {
    (self.func)(&self.data)
  }
}

fn max(data: &(u32, u32)) -> &u32 {
  if data.0 > data.1 {
    &data.0
  } else {
    &data.1
  }
}

pub fn run() {
  let pair = (3, 1);
  let elm = Pick {
    data: pair,
    func: max,
  };

  println!("1){}, 2){} for pair {:?}", elm.call(), elm.call(), pair);
}
