use core::fmt;

#[repr(transparent)]
pub struct Array64<T>(pub(crate) [T; 64]);

impl<T: Default> Default for Array64<T>
where T: Default
{
  #[rustfmt::skip]
  fn default() -> Self {
        Self([
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
            T::default(), T::default(), T::default(), T::default(), 
        ])
    }
}

impl<T> AsRef<[T]> for Array64<T> {
  fn as_ref(&self) -> &[T] { &self.0 }
}
impl<T> AsMut<[T]> for Array64<T> {
  fn as_mut(&mut self) -> &mut [T] { &mut self.0 }
}
impl<T> Clone for Array64<T>
where T: Copy + Default
{
  fn clone(&self) -> Self {
    let mut new = Self::default();
    new.0.copy_from_slice(&self.0);
    new
  }
}

impl<T> fmt::Debug for Array64<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Array64 {{}}") }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_array64(){
    let a: Array64<u64> = Array64::default();
    assert_eq!(a.0.len(), 64);
    assert_eq!(format!("{:?}", a), "Array64 {}");
  }
}