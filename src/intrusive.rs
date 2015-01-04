pub trait Intrusive<Struct>
{
  #[inline]
  fn offset(&Self) -> &Struct;

  #[inline]
  fn offset_mut(&mut Self) -> &mut Struct;
}
