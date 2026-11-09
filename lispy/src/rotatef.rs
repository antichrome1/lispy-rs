/// Each place receives the value of the next place, and the last
/// place receives the value of the first.
///
/// ```
/// use lispy::rotatef;
///
/// let (mut a, mut b, mut c) = (1, 2, 3);
/// rotatef(&mut [&mut a, &mut b, &mut c]);
/// assert_eq!((a, b, c), (2, 3, 1));
/// ```
pub fn rotatef<T>(places: &mut [&mut T]) {
    let n = places.len();
    if n < 2 {
        return;
    }

    for i in 0..n - 1 {
        let (left, right) = places.split_at_mut(i + 1);
        std::mem::swap(&mut *left[i], &mut *right[0]);
    }
}

/// Each place receives the value of the next place, the last place
/// receives `new`, and the old value of the first place is returned.
///
/// ```
/// use lispy::shiftf;
///
/// let (mut a, mut b, mut c) = (1, 2, 3);
/// let old = shiftf(&mut [&mut a, &mut b, &mut c], 9);
/// assert_eq!(old, 1);
/// assert_eq!((a, b, c), (2, 3, 9));
/// ```
pub fn shiftf<T>(places: &mut [&mut T], new: T) -> T {
    let mut carry = new;

    for p in places.iter_mut().rev() {
        carry = std::mem::replace(&mut **p, carry);
    }
    carry
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Pair {
        x: i32,
        y: i32,
    }

    #[test]
    fn rotatef_three_places() {
        let (mut a, mut b, mut c) = (1, 2, 3);
        rotatef(&mut [&mut a, &mut b, &mut c]);
        assert_eq!((a, b, c), (2, 3, 1));
    }

    #[test]
    fn rotatef_two_places_swaps() {
        let (mut a, mut b) = (1, 2);
        rotatef(&mut [&mut a, &mut b]);
        assert_eq!((a, b), (2, 1));
    }

    #[test]
    fn rotatef_single_or_empty_is_noop() {
        let mut a = 5;
        rotatef(&mut [&mut a]);
        assert_eq!(a, 5);
        rotatef::<i32>(&mut []);
    }

    #[test]
    fn rotatef_four_places_repeated_gives_identity() {
        let (mut a, mut b, mut c, mut d) = (1, 2, 3, 4);
        for _ in 0..4 {
            rotatef(&mut [&mut a, &mut b, &mut c, &mut d]);
        }
        assert_eq!((a, b, c, d), (1, 2, 3, 4));
    }

    #[test]
    fn rotatef_struct_fields() {
        let mut p = Pair { x: 10, y: 20 };
        rotatef(&mut [&mut p.x, &mut p.y]);
        assert_eq!((p.x, p.y), (20, 10));
    }

    #[test]
    fn rotatef_with_strings_moves_values() {
        let (mut a, mut b, mut c) = (
            String::from("a"),
            String::from("b"),
            String::from("c"),
        );
        rotatef(&mut [&mut a, &mut b, &mut c]);
        assert_eq!(a, "b");
        assert_eq!(b, "c");
        assert_eq!(c, "a");
    }

    #[test]
    fn shiftf_three_places() {
        let (mut a, mut b, mut c) = (1, 2, 3);
        let old = shiftf(&mut [&mut a, &mut b, &mut c], 9);
        assert_eq!(old, 1);
        assert_eq!((a, b, c), (2, 3, 9));
    }

    #[test]
    fn shiftf_single_place() {
        let mut a = 1;
        let old = shiftf(&mut [&mut a], 5);
        assert_eq!(old, 1);
        assert_eq!(a, 5);
    }

    #[test]
    fn shiftf_two_places() {
        let (mut a, mut b) = (1, 2);
        let old = shiftf(&mut [&mut a, &mut b], 7);
        assert_eq!(old, 1);
        assert_eq!((a, b), (2, 7));
    }

    #[test]
    fn shiftf_struct_fields() {
        let mut p = Pair { x: 1, y: 2 };
        let old = shiftf(&mut [&mut p.x, &mut p.y], 3);
        assert_eq!(old, 1);
        assert_eq!((p.x, p.y), (2, 3));
    }

    #[test]
    fn shiftf_with_strings() {
        let mut a = String::from("a");
        let mut b = String::from("b");
        let old = shiftf(&mut [&mut a, &mut b], String::from("z"));
        assert_eq!(old, "a");
        assert_eq!(a, "b");
        assert_eq!(b, "z");
    }
}
