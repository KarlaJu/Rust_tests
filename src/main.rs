fn get_two() -> i16 {
  2
}
#[cfg(test)]
mod firs_test {
  #[test]
  #[should_panic]    
    fn test(){
      assert!(1==1);
      panic!("Oh nooo!");
    }


  #[test]
  #[ignore]  
    fn equal_test(){
      assert_eq!(2, 1 + 1);
      assert_ne!(2, 1 + 2);
    }
  #[test]
    fn call_method(){
      let number = super::get_two();
      assert_eq!(number, 1 + 1);
      assert_ne!(number, 1 + 2);
    }
  
}
