// 1. Tuple type is a named compisition of other types.
// 2. Tuple type has same generic and lifetime rules in struct type.
// 3. Tuple elements don't have a name, they can only be accessed by pattern-matching or
//     by using N directly as a field to access the Nth element.
// 4. The tuple type with no elements (()) is often called ‘unit’ or ‘the unit type’.

type Pair<'a> = (i32, &'a str);

pub fn run() {
  let p: Pair<'static> = (10, "ten");
  let (a, b) = p;

  assert_eq!(a, 10);
  assert_eq!(b, "ten");
  assert_eq!(p.0, 10);
  assert_eq!(p.1, "ten");
}
