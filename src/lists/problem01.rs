use collections::{ List, Cons, Nil };

/// Find the last element of a list.
pub fn last<T>(list: List<T>) -> Option<T>  {
	match list {
		Nil => return None,
		Cons(head, tail) => match *tail {
			Nil => return Some(head),
			Cons(_, _) => return last(*tail)
		}
	}
}

#[test]
fn last_returns_last_element()
{
	assert_eq!(Some(8), last(list![1, 1, 2, 3, 5, 8]));
}

#[test]
fn last_returns_none_for_empty_list()
{
	assert_eq!(None, last::<i32>(Nil));
}