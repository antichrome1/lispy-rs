/// Common Lisp-style `map` for one sequence, returning a Vec.
/// ```
/// use lispy::map;
/// assert_eq!(map(&[1, 2, 3], |x| x * 2), vec![2, 4, 6]);
/// ```
pub fn map<T, U, F>(sequence: &[T], mut f: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    sequence.iter().map(&mut f).collect()
}

/// Two-sequence `map`: stops when either input ends.
/// ```
/// use lispy::map2;
/// assert_eq!(map2(&[1, 2, 3], &[10, 20], |a, b| a + b), vec![11, 22]);
/// ```
pub fn map2<A, B, U, F>(a: &[A], b: &[B], mut f: F) -> Vec<U>
where
    F: FnMut(&A, &B) -> U,
{
    a.iter().zip(b).map(|(x, y)| f(x, y)).collect()
}

/// Common Lisp-style `map-into` for one source sequence.
/// Leaves any destination elements beyond the mapped prefix unchanged.
/// ```
/// use lispy::map_into;
///
/// let source = [1, 2];
/// let mut destination = [0, 0, 9];
/// map_into(&mut destination, &source, |x| x * 10);
/// assert_eq!(destination, [10, 20, 9]);
/// ```
pub fn map_into<T, U, F>(destination: &mut [U], source: &[T], mut f: F)
where
    F: FnMut(&T) -> U,
{
    for (dst, src) in destination.iter_mut().zip(source) {
        *dst = f(src);
    }
}

/// Two-source `map-into`: stops at the shortest of destination and inputs.
/// ```
/// use lispy::map_into2;
///
/// let a = [1, 2, 3];
/// let b = [10, 20];
/// let mut destination = [0, 0, 99];
/// map_into2(&mut destination, &a, &b, |x, y| x + y);
/// assert_eq!(destination, [11, 22, 99]);
/// ```
pub fn map_into2<A, B, U, F>(destination: &mut [U], a: &[A], b: &[B], mut f: F)
where
    F: FnMut(&A, &B) -> U,
{
    for ((dst, x), y) in destination.iter_mut().zip(a).zip(b) {
        *dst = f(x, y);
    }
}

/// Apply a function to each element and collect the results.
/// ```
/// use lispy::mapcar;
/// assert_eq!(mapcar(&[1, 2, 3], |x| x + 1), vec![2, 3, 4]);
/// ```
pub fn mapcar<T, U, F>(sequence: &[T], f: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    map(sequence, f)
}

/// Two-sequence `mapcar`.
/// ```
/// use lispy::mapcar2;
/// assert_eq!(mapcar2(&[1, 2], &[3, 4], |a, b| a * b), vec![3, 8]);
/// ```
pub fn mapcar2<A, B, U, F>(a: &[A], b: &[B], f: F) -> Vec<U>
where
    F: FnMut(&A, &B) -> U,
{
    map2(a, b, f)
}

/// Apply a function to each element; return the original sequence.
///
/// ```
/// use lispy::mapc;
///
/// let input = [1, 2, 3];
/// let mut sum = 0;
/// let returned = mapc(&input, |x| sum += x);
/// assert_eq!(sum, 6);
/// assert_eq!(returned, &input);
/// ```
pub fn mapc<T, F>(sequence: &[T], mut f: F) -> &[T]
where
    F: FnMut(&T),
{
    for item in sequence {
        f(item);
    }
    sequence
}

/// Apply a function to each nonempty tail and collect the results.
/// ```
/// use lispy::maplist;
/// assert_eq!(maplist(&[1, 2, 3], |tail| tail.len()), vec![3, 2, 1]);
/// ```
pub fn maplist<T, U, F>(sequence: &[T], mut f: F) -> Vec<U>
where
    F: FnMut(&[T]) -> U,
{
    (0..sequence.len())
        .map(|i| f(&sequence[i..]))
        .collect()
}

/// Apply a function to each nonempty tail; return the original sequence.
/// ```
/// use lispy::mapl;
///
/// let input = [1, 2, 3];
/// let mut lengths = Vec::new();
/// let returned = mapl(&input, |tail| lengths.push(tail.len()));
/// assert_eq!(lengths, vec![3, 2, 1]);
/// assert_eq!(returned, &input);
/// ```
pub fn mapl<T, F>(sequence: &[T], mut f: F) -> &[T]
where
    F: FnMut(&[T]),
{
    for i in 0..sequence.len() {
        f(&sequence[i..]);
    }
    sequence
}

/// Apply a function to each element and concatenate its Vec results.
/// ```
/// use lispy::mapcan;
/// assert_eq!(mapcan(&[1, 2, 3], |x| vec![*x, *x]), vec![1, 1, 2, 2, 3, 3]);
/// ```
pub fn mapcan<T, U, F>(sequence: &[T], mut f: F) -> Vec<U>
where
    F: FnMut(&T) -> Vec<U>,
{
    sequence.iter().flat_map(&mut f).collect()
}

/// Apply a function to each nonempty tail and concatenate its Vec results.
/// ```
/// use lispy::mapcon;
/// assert_eq!(mapcon(&[1, 2, 3], |tail| vec![tail[0]]), vec![1, 2, 3]);
/// ```
pub fn mapcon<T, U, F>(sequence: &[T], mut f: F) -> Vec<U>
where
    F: FnMut(&[T]) -> Vec<U>,
{
    (0..sequence.len())
        .flat_map(|i| f(&sequence[i..]))
        .collect()
}

/// Return the first truthy callback result, or None.
/// ```
/// use lispy::some;
/// assert_eq!(some(&[1, 2, 3], |x| (*x == 2).then_some(x * 10)), Some(20));
/// ```
pub fn some<T, U, F>(sequence: &[T], mut predicate: F) -> Option<U>
where
    F: FnMut(&T) -> Option<U>,
{
    sequence.iter().find_map(&mut predicate)
}

/// Return true if the predicate holds for every element.
/// ```
/// use lispy::every;
/// assert!(every(&[2, 4, 6], |x| x % 2 == 0));
/// ```
pub fn every<T, F>(sequence: &[T], mut predicate: F) -> bool
where
    F: FnMut(&T) -> bool,
{
    sequence.iter().all(&mut predicate)
}

/// Return true if the predicate holds for no elements.
/// ```
/// use lispy::notany;
/// assert!(notany(&[1, 3, 5], |x| x % 2 == 0));
/// ```
pub fn notany<T, F>(sequence: &[T], predicate: F) -> bool
where
    F: FnMut(&T) -> bool,
{
    !sequence.iter().any(predicate)
}

/// Return true if the predicate fails for at least one element.
/// ```
/// use lispy::notevery;
/// assert!(notevery(&[2, 3, 4], |x| x % 2 == 0));
/// ```
pub fn notevery<T, F>(sequence: &[T], predicate: F) -> bool
where
    F: FnMut(&T) -> bool,
{
    !sequence.iter().all(predicate)
}

/// Left-to-right reduction of a nonempty sequence.
///
/// Returns None for an empty sequence. The callback receives the accumulator
/// first and the next element second.
///
/// ```
/// use lispy::reduce;
/// assert_eq!(reduce(&[1, 2, 3, 4], |acc, x| acc + x), Some(10));
/// ```
pub fn reduce<T, F>(sequence: &[T], f: F) -> Option<T>
where
    T: Clone,
    F: FnMut(T, &T) -> T,
{
    let mut iter = sequence.iter();
    let first = iter.next()?.clone();
    Some(iter.fold(first, f))
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn map_and_mapcar_collect_results() {
        assert_eq!(map(&[1, 2, 3], |x| x * 2), vec![2, 4, 6]);
        assert_eq!(mapcar(&[1, 2, 3], |x| x + 1), vec![2, 3, 4]);
    }

    #[test]
    fn two_sequence_mapping_stops_at_shorter_input() {
        assert_eq!(map2(&[1, 2, 3], &[10, 20], |a, b| a + b), vec![11, 22]);
        assert_eq!(mapcar2(&[1, 2], &[3, 4], |a, b| a * b), vec![3, 8]);
    }

    #[test]
    fn map_into_preserves_unmapped_destination_suffix() {
        let source = [1, 2];
        let mut destination = [0, 0, 9];
        map_into(&mut destination, &source, |x| x * 10);
        assert_eq!(destination, [10, 20, 9]);
    }

    #[test]
    fn map_into2_stops_at_shortest_input() {
        let a = [1, 2, 3];
        let b = [10, 20];
        let mut destination = [0, 0, 99];
        map_into2(&mut destination, &a, &b, |x, y| x + y);
        assert_eq!(destination, [11, 22, 99]);
    }

    #[test]
    fn mapc_runs_callback_and_returns_input() {
        let input = [1, 2, 3];
        let mut sum = 0;
        let returned = mapc(&input, |x| sum += x);
        assert_eq!(sum, 6);
        assert_eq!(returned, &input);
    }

    #[test]
    fn maplist_visits_nonempty_tails() {
        assert_eq!(maplist(&[1, 2, 3], |tail| tail.len()), vec![3, 2, 1]);
        assert_eq!(maplist::<i32, usize, _>(&[], |tail| tail.len()), vec![]);
    }

    #[test]
    fn mapl_visits_tails_and_returns_input() {
        let input = [1, 2, 3];
        let mut lengths = Vec::new();
        let returned = mapl(&input, |tail| lengths.push(tail.len()));
        assert_eq!(lengths, vec![3, 2, 1]);
        assert_eq!(returned, &input);
    }

    #[test]
    fn mapcan_and_mapcon_concatenate_results() {
        assert_eq!(mapcan(&[1, 2, 3], |x| vec![*x, *x]), vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(mapcon(&[1, 2, 3], |tail| vec![tail[0]]), vec![1, 2, 3]);
    }

    #[test]
    fn sequence_predicates() {
        assert_eq!(some(&[1, 2, 3], |x| (*x == 2).then_some(x * 10)), Some(20));
        assert_eq!(some::<_, i32, _>(&[1, 3], |x| (*x == 2).then_some(*x)), None);

        assert!(every(&[2, 4, 6], |x| x % 2 == 0));
        assert!(!every(&[2, 3, 6], |x| x % 2 == 0));

        assert!(notany(&[1, 3, 5], |x| x % 2 == 0));
        assert!(!notany(&[1, 2, 3], |x| x % 2 == 0));

        assert!(notevery(&[2, 3, 4], |x| x % 2 == 0));
        assert!(!notevery(&[2, 4, 6], |x| x % 2 == 0));
    }

    #[test]
    fn reduce_handles_nonempty_and_empty_sequences() {
        assert_eq!(reduce(&[1, 2, 3, 4], |acc, x| acc + x), Some(10));
        assert_eq!(reduce(&[5], |acc, x| acc + x), Some(5));
        assert_eq!(reduce::<i32, _>(&[], |acc, x| acc + x), None);
    }
}
