pub trait Intrusive<Struct>
{
  #[inline]
  fn field(&mut self) -> &mut Struct;
}

pub trait IntrusiveExt<U> {
  #[inline]
  fn field(&self) -> &mut U;
}

impl<T, U> IntrusiveExt<U> for *mut T where T: Intrusive<U> {
  #[inline]
  fn field(&self) -> &mut U {
    unsafe { &mut **self }.field()
  }
}
