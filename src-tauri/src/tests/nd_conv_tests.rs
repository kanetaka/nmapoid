use super::super::nd_conv;
//use app::nd_conv;

#[test]
fn test1() {
  assert_eq!(2+2, 4);
}

#[test]
fn test2() {
  match nd_conv::asl_conv::read_csv() {
    Ok(_) => assert!(true),
    Err(_) => assert!(false),
  }
}