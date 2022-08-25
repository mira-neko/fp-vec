/// Anything to what you can push.
pub trait Fpushable {
    type Item;
    type Output;

    fn fpush(self, value: Self::Item) -> Self::Output;
}

/// Pushing to a slice.
/// 
/// # Examples
/// 
/// ```
/// use fp_vec::Fpushable;
/// 
/// assert_eq!(
///     [1, 2].fpush(3),
///     vec![1, 2, 3]
/// );
/// ```
impl<T: Clone> Fpushable for &[T] {
    type Item = T;
    type Output = Vec<T>;

    fn fpush(self, value: Self::Item) -> Self::Output {
        let mut vec = Vec::with_capacity(self.len() + 1);
        vec.extend_from_slice(self);
        vec.push(value);
        vec
    }
}

/// Pushing to a Vec.
/// 
/// # Examples
/// 
/// ```
/// use fp_vec::Fpushable;
/// 
/// assert_eq!(
///     vec![1, 2].fpush(3),
///     vec![1, 2, 3]
/// );
/// ```
impl<T: Clone> Fpushable for Vec<T> {
    type Item = T;
    type Output = Self;

    fn fpush(self, value: Self::Item) -> Self::Output {
        let mut vec = self.clone();
        vec.push(value);
        vec
    }
}
