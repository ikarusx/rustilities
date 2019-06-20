
#[cfg(test)]
pub mod tests
{
    //use super::*;
    use crate::samples;

    #[test]
    fn actual_test01_structs_and_types()
    {
        samples::s01_types::sample01_structs_and_types();
    }

    #[should_panic]
    #[test]
    fn test_any_panic() {
        panic!("Any panic test!");
    }
    
    #[should_panic(expected = "This is a successful specific panic test!")]
    #[test]
    fn test_specific_panic() {
        panic!("This is a successful specific panic test!");
    }

    #[test]
    fn utest04_loop_return()
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }
}