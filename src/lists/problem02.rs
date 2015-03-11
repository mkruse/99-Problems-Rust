use collections::{ List, Cons, Nil };

// Find the last but one element of a list
pub fn penultimate<T>(list: List<T>) -> Option<T> 
{
    match list {
        Nil => return None,
        Cons(head, tail) => match *tail {
            Nil => return None,
            Cons(_, tail_of_tail) => match *tail_of_tail {
                Nil => return Some(head),
                Cons(_, _) => return penultimate(*tail_of_tail)
            }
        }
    }
}

#[test]
fn penultimate_returns_last_but_one_element()
{
    assert_eq!(Some(5), penultimate(list![1, 1, 2, 3, 5, 8]));
}

#[test]
fn penultimate_returns_none_for_empty_list()
{
    assert_eq!(None, penultimate::<i32>(Nil));
}

#[test]
fn penultimate_returns_none_for_list_with_only_one_element()
{
    assert_eq!(None, penultimate(list![1]));
}