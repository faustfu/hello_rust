// 1. Macro has two usage categories:
//    Callable(passive): println!, assert_eq!, ...
//    Attribute(active): #derive(Debug), #cfg, ...
// 2. Token types:
//    expr: expression, able to be evaluated
//    tt: token tree
//    item: language item(keyword)
//    block: {} block
//    stmt: statement ending with ";"
//    pat: pattern
//    ty: type
//    ident: identity token, ex: i, j, k...
//    path: naming path, ex: std::iter, ...
//    meta: meta info, ex: #[test], ...
//    vis: visibility, ex: pub
//    lifetime: lifetime arguments, ex: 'a, ...

macro_rules! unless {
  // rule 1
  ($arg:expr, $branch:expr) => {
    if !$arg {
      $branch
    }
  }; // rule 2 ...
}

fn cmp(a: i32, b: i32) {
  unless!(a > b, println!("{} < {}", a, b));
}
pub fn run() {
  let (a, b) = (1, 2);
  cmp(a, b);
}
