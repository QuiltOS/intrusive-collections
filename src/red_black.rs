use core::prelude::*;

use core::mem::uninitialized;

use intrusive::{Intrusive, IntrusiveExt};
use aligned_ptr_pun::AlignedPtrPun;
use easy_unsafe_ref::EasyUnsafeRef;


/// The fields requied to be in a node to store it in a intrusive red-black
/// tree.
///
/// Add this to your type, T, and implement `Intrusive<Node<T>>` to, in effect,
/// tell this library what the offset is.q
///
/// Instances of your type should be created with `Node<T>` uninitailized.
pub struct Node<T> {
  left:      *mut T,
  right_red: AlignedPtrPun<T>,
}

impl<T> Node<T> where T: Intrusive<Node<T>> + Ord
{
  #[inline]
  pub fn new(tree: &mut Tree<T>) -> Node<T> {
    Node {
      left:      tree.nil.rf(),
      right_red: AlignedPtrPun::new(tree.nil.rf(), true),
    }
  }

  #[inline]
  fn right(&mut self) -> *mut T {
    self.right_red.eliminate().0
  }

  #[inline]
  fn set_right(&mut self, ptr: *mut T) {
    self.right_red.set_ptr(ptr)
  }

  #[inline]
  fn color(&mut self) -> bool {
    self.right_red.eliminate().1
  }

  #[inline]
  fn set_color(&mut self, color: bool) {
    self.right_red.set_flag(color)
  }
}

trait NodeExt  {
  fn rotate_left(&mut self) -> Self;
  fn rotate_right(&mut self) -> Self;
}

impl<T>  NodeExt for *mut T where T: Intrusive<Node<T>> + Ord
{
  #[inline]
  fn rotate_left(&mut self) -> Self {
    let old_right = self.field().right();
    self.field().set_right(old_right.field().left);
    old_right.field().left = *self;
    old_right
  }

  #[inline]
  fn rotate_right(&mut self) -> Self {
    let old_left = self.field().left;

    self.field().left = old_left.field().right();
    old_left.field().set_right(*self);
    old_left
  }
}

/// Left-leaning 2-3 red-black trees.  Parent pointers are not used, and color
/// bits are stored in the least significant bit of right-child pointers thus
/// making node linkage as compact as is possible for red-black trees.
///
/// Ported from https://github.com/thestinger/allocator/blob/master/rb.h. In
/// turn from jemalloc.
pub struct Tree<T> {
  root: *mut T,
  nil:  T
}

impl<T> Tree<T> where T: Intrusive<Node<T>> + Ord
{
  #[inline]
  pub fn place() -> Tree<T> {
    unsafe { uninitialized() }
  }

  #[inline]
  pub fn init(&mut self) {
    self.root = self.nil.rf();
    *self.nil.field() = Node {
      left:      self.nil.rf(),
      right_red: AlignedPtrPun::new(self.nil.rf(), false),
    };
  }

  // Utils, actual functions subst null ptr for sentinal ptr

  #[inline]
  fn first_(&mut self, subtree: *mut T) -> *mut T {
    let mut node = subtree;

    if node != self.nil.rf() {
      while node.field().left != self.nil.rf() {
        node = node.field().left;
      }
    }
    node
  }

  #[inline]
  fn last_(&mut self, subtree: *mut T) -> *mut T {
    let mut node = subtree;

    if node != self.nil.rf() {
      while node.field().right() != self.nil.rf() {
        node = node.field().right();
      }
    }
    node
  }

  fn sanitize(&mut self, ptr: *mut T) -> *mut T {
    if ptr == self.nil.rf() {
      0 as *mut T
    } else {
      ptr
    }
  }

  #[inline]
  pub fn first(&mut self) -> *mut T {
    let ptr = self.first_(self.root);
    self.sanitize(ptr)
  }

  #[inline]
  pub fn last(&mut self) -> *mut T {
    let ptr = self.last_(self.root);
    self.sanitize(ptr)
  }

  #[inline]
  pub fn next(&mut self, node: *mut T) -> *mut T {
    debug_assert!(node != 0 as *mut T);
    let mut ret;
    if node.field().right() != self.nil.rf() {
      ret = self.first_(node.field().right());
    } else {
      let mut tnode = self.root;
      ret = self.nil.rf();
      assert!(tnode != self.nil.rf());
      loop {
        tnode = match unsafe { (*node).cmp(&*tnode) } {
          Less    => {
            ret = tnode;
            tnode.field().left
          },
          Greater => tnode.field().right(),
          Equal   => break,
        };
        assert!(tnode != self.nil.rf());
      }
    }
    self.sanitize(ret)
  }

  #[inline]
  pub fn prev(&mut self, node: *mut T) -> *mut T {
    let mut ret;
    if node.field().left != self.nil.rf() {
      ret = self.last_(node.field().left);
    } else {
      let mut tnode = self.root;
      ret = self.nil.rf();
      assert!(tnode != self.nil.rf());
      loop {
        tnode = match unsafe { (*node).cmp(&*tnode) } {
          Less    => tnode.field().left,
          Greater => {
            ret   = tnode;
            tnode.field().right()
          },
          Equal   => break,
        };
        assert!(tnode != self.nil.rf());
      }
    }
    self.sanitize(ret)
  }

  #[inline]
  pub fn search(&mut self, key: *mut T) -> *mut T {
    let mut ret = self.root;
    while ret != self.nil.rf() {
      ret = match unsafe { (*key).cmp(&*ret) } {
        Less    => ret.field().left,
        Greater => ret.field().right(),
        Equal   => break,
      }
    }
    self.sanitize(ret)
  }

  #[inline]
  pub fn nsearch(&mut self, key: *mut T) -> *mut T {
    let mut ret = self.nil.rf();
    let mut tnode = self.root;
    while tnode != self.nil.rf() {
      tnode = match unsafe { (*key).cmp(&*ret) } {
        Less    => {
          ret = tnode;
          ret.field().left
        },
        Greater => ret.field().right(),
        Equal   => {
          ret = tnode;
          break
        },
      }
    }
    self.sanitize(ret)
  }

  #[inline]
  pub fn psearch(&mut self, key: *mut T) -> *mut T {
    let mut ret = self.nil.rf();
    let mut tnode = self.root;
    while tnode != self.nil.rf() {
      tnode = match unsafe { (*key).cmp(&*ret) } {
        Less    => ret.field().left,
        Greater => {
          ret = tnode;
          ret.field().right()
        },
        Equal   => {
          ret = tnode;
          break
        },
      }
    }
    self.sanitize(ret)
  }

  #[inline]
  pub fn insert(&mut self, node: *mut T) {
    let mut path: [PathElem<T>, ..::core::uint::BITS << 1] = unsafe { uninitialized() };
    *node.field() = Node::new(self);

    // Wind
    {
      path[0].node = self.root;
      let mut iter = path.iter_mut();
      let mut cur  = iter.next().unwrap();
      let mut next = iter.next().unwrap();
      loop {
        if cur.node == self.nil.rf() { break };

        cur.cmp = unsafe { (*node).cmp(&*cur.node) };
        next.node = match cur.cmp {
          Equal   => unreachable!(),
          Less    => cur.node.field().left,
          Greater => cur.node.field().right(),
        };

        cur = next;
        next = iter.next().unwrap();
      }
      cur.node = node;
    }

    // Unwind
    {
      let mut iter = path.iter_mut().rev();
      let mut prev = iter.next().unwrap();
      while let Some(cur) = iter.next() {
        let mut cnode = cur.node;
        cnode = match cur.cmp {
          Less => {
            let left = prev.node;
            cnode.field().left = cnode;
            if left.field().color() {
              let left_left = left.field().left;
              if left_left.field().color() {
                // Fix up 4-node
                left_left.field().set_color(false);
                cnode.rotate_right()
              } else {
                cnode // keep current
              }
            } else {
              return
            }
          },
          #[cfg(not(ndebug))]
          Equal => unreachable!(),
          _ => {
            let right = prev.node;
            node.field().set_right(right);
            if right.field().color() {
              let left = right.field().left;
              if left.field().color() {
                // Split 3-node
                left.field().set_color(false);
                right.field().set_color(false);
                cnode.field().set_color(true);
                cnode // keep current
              } else {
                let tred = cnode.field().color();
                let tnode = cnode.rotate_left();
                tnode.field().set_color(tred);
                cnode.field().set_color(true);
                tnode
              }
            } else {
              return
            }
          },
        };
        cur.node = cnode;
        prev = cur;
      }
    }

    // Set root, and paint it black
    self.root = path[0].node;
    self.root.field().set_color(false);
  }
}

struct PathElem<T> {
  node: *mut T,
  cmp:  Ordering,
}
