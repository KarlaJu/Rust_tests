#[cfg(test)]
mod firs_test {
#[test]
#[should_panic]    
    fn test(){
    assert!(1==1);
    panic!("Oh nooo!");
    }
}
