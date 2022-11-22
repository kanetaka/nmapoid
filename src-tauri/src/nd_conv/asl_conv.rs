extern crate csv;
use encoding_rs;
use std::error::Error;
use std::process;
use std::fs;

pub struct AslConverter;

impl AslConverter {
}


pub fn read_csv() -> Result<(), Box<dyn Error>> {
  let file_path = "A:/PERSONAL/kanetaka/braino/trade/asl/アメリカ-中国_27_sitc/ASL_アメリカ_export_27_sitc.csv";
  let file = fs::read(file_path).unwrap();
  let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file);
  let mut rdr = csv::Reader::from_reader(res.as_bytes());

  for result in rdr.records() {
    let record = result?;
    println!("{:?}", record);
  }
  Ok(())
}