struct Rectangle{
  width: u8,
  height: u8
}

impl Rectangle{
  fn is_square(&self) -> bool {
    self.width == self.height
  }
}

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

  #[test]
  #[should_panic]
    fn test_struct_rectangle(){
      let size = super::Rectangle {
        width: 50,
        height: 25
      };
      assert!(size.is_square());
    }  
}
