use structopt_derive::*;
#[derive(StructOpt, Debug)]
#[structopt(name="hello_rust", about="Usage")]
pub struct Opt {
  #[structopt(help="Input file")]
  pub input:String,
  #[structopt(help="Column name")]
  pub column_name:String,
  #[structopt(help="Replacement column name")]
  pub replacement:String,
  #[structopt(help="Output file, stdout if not present")]
  pub output:Option<String>,
}