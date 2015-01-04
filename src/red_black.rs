use intrusive::Intrusive;

struct AlignedPtrPun<T>(*mut T);

struct RBNode<T> {
  left:  *mut T,
  right: AlignedPtrPun<T>,
}

fn insert<T>(x: RBNode<T>) where T: Intrusive<RBNode<T>> { }
