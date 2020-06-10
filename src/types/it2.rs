// 1. trait:IntoIterator is not an iterator, but it could be converted to be an iterator by calling the method:into_iter.

// take a list which item type is "list" and generate a flatten iterator.
fn flatten<I>(iter: I) -> Flatten<I::IntoIter> // associated type:IntoIter
where
  I: IntoIterator,
  I::Item: IntoIterator,
{
  Flatten::new(iter.into_iter()) // save converted iterator
}

struct Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  outer: O,                                           // save converted iterator
  inner: Option<<O::Item as IntoIterator>::IntoIter>, // keep current iterator of the second layer
}

impl<O> Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  fn new(iter: O) -> Self {
    Flatten {
      outer: iter,
      inner: None, // use None as initial status
    }
  }
}

impl<O> Iterator for Flatten<O>
where
  O: Iterator,
  O::Item: IntoIterator,
{
  type Item = <O::Item as IntoIterator>::Item; // every items has to be able to be an iterator and use the inner type as outer type.

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if let Some(ref mut inner_iter) = self.inner {
        if let Some(item) = inner_iter.into_iter().next() {
          return Some(item); // normal return
        }
      }
      // second layer iterator is end, try to get next iterator
      let next_inner_iter = self.outer.next()?.into_iter(); // return None if first layer iterator is end
      self.inner = Some(next_inner_iter);
    }
  }
}

pub fn run() {
  println!("it2");
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn empty() {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
  }

  #[test]
  fn one() {
    assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
  }

  #[test]
  fn two() {
    assert_eq!(flatten(std::iter::once(vec!["a", "k"])).count(), 2);
  }

  #[test]
  fn two_wide() {
    assert_eq!(flatten(vec![vec!["a"], vec!["k"]]).count(), 2);
  }
}
