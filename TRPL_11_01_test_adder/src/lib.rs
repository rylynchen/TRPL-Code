fn internal_adder(a: i32,b:i32) -> i32 {
    a + b
}

pub fn add_two(x : i32) -> i32 {
    x + 2
}

pub fn add_three(x: i32) -> i32 {
    x + 3
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    #[test]
//    fn it_works() {
//        assert_eq!(2+2, 4);
//    }
//
//    #[test]
//    #[should_panic(expected="Make this test")]
//    fn it_not_works() {
////        assert_eq!(2+2, 5);
//        panic!("Make this test fail xxx");
//    }
//
//    #[test]
//    #[ignore]
//    fn it_work_result() ->Result<(), String> {
//        if 2+2 == 5 {
//            Ok(())
//        } else {
//            Err(String::from("two plus two does not equal five"))
//        }
//    }
//
//    #[test]
//    fn internal() {
//        assert_eq!(5, internal_adder(2, 2));
//    }
//}