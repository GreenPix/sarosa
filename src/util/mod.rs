use std::ptr;


pub fn insert_at<T>(into: &mut Vec<T>,
                    pos: usize,
                    other: &[T])
    where T: Clone
{
    let old_len = into.len();
    let offset = old_len - pos;

    // First insert all element at the end
    into.push_all(other);

    if pos >= old_len {
        return;
    }

    let p = into.as_mut_ptr();
    let len = into.len();

    // Swap starting
    unsafe {
        // Overwrite memory with end
        for i in (pos as isize)..(pos + other.len()) as isize {
            ptr::swap(p.offset(i), p.offset(i + offset as isize));
        }

        for i in (other.len() + pos)..len - 1 {
            ptr::swap(p.offset(i as isize), p.offset(i as isize + 1));
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn insert_at_0() {
        let mut vec = vec![1, 2, 3];
        let other = vec![4, 5];

        super::insert_at(&mut vec, 0, &other);

        assert_eq!(vec, vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn insert_at_empty() {
        let mut vec = Vec::new();
        let other = vec![4, 5];

        super::insert_at(&mut vec, 0, &other);

        assert_eq!(vec, other);
    }

    #[test]
    fn insert_at_end() {
        let mut vec = vec![1, 2, 3];
        let other = vec![4, 5];

        super::insert_at(&mut vec, 3, &other);

        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }


    #[test]
    fn insert_at_n() {
        let mut vec = vec![1, 4];
        let other = vec![2, 3];

        super::insert_at(&mut vec, 1, &other);
        assert_eq!(vec, vec![1, 2, 3, 4]);
    }
}
