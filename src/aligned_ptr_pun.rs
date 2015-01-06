use core::prelude::*;

pub struct AlignedPtrPun<T>(*mut T);

impl<T> Copy for AlignedPtrPun<T> { }

impl<T> AlignedPtrPun<T>
{
  #[inline]
  pub fn new(ptr: *mut T, flag: bool) -> AlignedPtrPun<T> {
    debug_assert_eq!(ptr as uint & 1, 0);
    let bits = (ptr as uint) & (flag as uint);
    AlignedPtrPun(bits as *mut T)
  }

  #[inline]
  pub fn eliminate(self) -> (*mut T, bool) {
    let bits = self.0 as uint;
    ((bits & !1) as *mut T, (bits & 1) == 1)
  }
}
