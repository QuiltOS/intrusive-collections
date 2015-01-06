pub trait EasyUnsafeRef
{
  fn rf(&mut self) -> *mut Self;
}

impl<T> EasyUnsafeRef for T
{
  fn rf(&mut self) -> *mut T {
    self as *mut T
  }
}
