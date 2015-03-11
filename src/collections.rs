pub use self::List::Cons;
pub use self::List::Nil;

#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>), 
    Nil
}

impl <T> List<T> {
    pub fn new<TClonable :Clone>(elements: &[TClonable]) -> List<TClonable> 
    {
        let mut list = Nil;
        for element in elements.iter().rev() {
            list = Cons(element.clone(), Box::new(list));
        }

        list
    }   
}

#[macro_export]
macro_rules! list {
    () => { Nil };

    ( $( $x:expr ),+ ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            
            let mut list = Nil;
            for element in temp_vec.iter().rev() {
                list = Cons(element.clone(), Box::new(list));
            }

            list
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_list() {
        let a : &[i32] = &[];
        let list = List::<i32>::new(a);
        
        match list {
            Nil => {}, // OK
            Cons(_, _) => panic!("List is not empty")
        }
    }

    #[test]
    fn create_single_element_list() {
        let a = [ 1 ];
        let list = List::<i32>::new(&a);
        
        match list {
            Nil => panic!("List is empty"),
            Cons(1, tail) => match *tail {
                Nil => {}, // OK
                Cons(_, _) => panic!("List ist too long")
            }, 
            Cons(_, tail) => match *tail {
                Nil => panic!("Wrong head"),
                Cons(_, _) => panic!("Wrong head and list is too long")
            }
        }
    }

    #[test]
    fn create_list_with_three_elements() {
        let a = [ 1, 2, 3];
        let list = List::<i32>::new(&a);
        
        match list {
            Nil => panic!("List is empty"),
            Cons(1, tail) => match *tail {
                Nil => panic!("List ist too short (1)"),
                Cons(2, tail2) => match *tail2 {
                    Nil => panic!("List ist too short (1, 2)"),
                    Cons(3, tail3) => match *tail3 {
                        Nil => {} // OK
                        Cons(_, _) => panic!("List is too long")
                    },
                    Cons(_, _) => panic!("Third element is wrong")
                }, 
                Cons(_, _) => panic!("Second element is wrong")
            }, 
            Cons(_, _) => panic!("First element is wrong")
        }
    }

    #[test]
    fn create_empty_list_using_macro() {
        let list: List<i32> = list![];
        
        match list {
            Nil => {}, // OK
            Cons(_, _) => panic!("List is not empty")
        }
    }

    #[test]
    fn create_single_element_list_using_macro() {
        let list = list![1];
        
        match list {
            Nil => panic!("List is empty"),
            Cons(1, tail) => match *tail {
                Nil => {}, // OK
                Cons(_, _) => panic!("List ist too long")
            }, 
            Cons(_, tail) => match *tail {
                Nil => panic!("Wrong head"),
                Cons(_, _) => panic!("Wrong head and list is too long")
            }
        }
    }


    #[test]
    fn create_list_with_three_elements_using_macro() {
        let list = list![ 1, 2, 3];
        
        match list {
            Nil => panic!("List is empty"),
            Cons(1, tail) => match *tail {
                Nil => panic!("List ist too short (1)"),
                Cons(2, tail2) => match *tail2 {
                    Nil => panic!("List ist too short (1, 2)"),
                    Cons(3, tail3) => match *tail3 {
                        Nil => {} // OK
                        Cons(_, _) => panic!("List is too long")
                    },
                    Cons(_, _) => panic!("Third element is wrong")
                }, 
                Cons(_, _) => panic!("Second element is wrong")
            }, 
            Cons(_, _) => panic!("First element is wrong")
        }
    }

}
