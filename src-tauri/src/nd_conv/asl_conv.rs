extern crate csv;
use encoding_rs;
use std::error::Error;
//use std::process;
use std::fs;
use geojson::{GeoJson, Geometry, Value};

#[derive(Default)]
pub struct AslGeoJson {
  lon: f64,
  lat: f64,
}

impl AslGeoJson {
  pub fn new(lon: f64, lat: f64) -> Self {
    AslGeoJson { lon: lon, lat: lat }
  }
}

pub fn to_geojson(file_path: &str) -> Option<AslGeoJson> {
  let asl = AslGeoJson::new(0.0, 0.0);
  Some(asl)
}

pub fn read_csv() -> Result<(), Box<dyn Error>> {
  let file_path = "./assets/samples/ASL_sample.csv";
  let file = fs::read(file_path).unwrap();
  let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file);
  let mut rdr = csv::Reader::from_reader(res.as_bytes());

  for result in rdr.records() {
    let record = result?;
    println!("{:?}", record);
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let asl = AslGeoJson::new(1.0, 2.0);
    assert_eq!(asl.lon, 1.0);
    assert_eq!(asl.lat, 2.0);
  }

  #[test]
  fn test2() {
    match read_csv() {
      Ok(_) => assert!(true),
      Err(_) => assert!(false),
    }
}
}
