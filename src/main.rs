#[cfg(test)]
mod firs_test {
  #[test]
  #[should_panic]    
    fn test(){
      assert!(1==1);
      panic!("Oh nooo!");
    }


  #[test]
    fn equal_test(){
      assert_eq!(2, 1 + 1);
      assert_ne!(2, 1 + 2);
    }
}
