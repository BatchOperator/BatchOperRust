//! batch_oper provides some batch operation macro for some operations
//! ## Usage
//! - **Basic**  
//!   - batch `||`  
//!     ```rust
//!     # use batch_oper::*;
//!     # let v =
//!     bop!(|| 4; == 2, > 3);
//!     # assert!(v);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     # let v =
//!     4 == 2 || 4 > 3;
//!     # assert!(v);
//!     ```
//!   - batch `&&`  
//!     ```rust
//!     # use batch_oper::*;
//!     # let v =
//!     bop!(&& 4; == 2, > 3);
//!     # assert!(!v);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     # let v =
//!     4 == 2 && 4 > 3;
//!     # assert!(!v);
//!     ```
//!   - `!`
//!     ```rust
//!     # use batch_oper::*;
//!     # let a = 1;
//!     # let v =
//!     bop!(|| a; == 1;!, == 2);
//!     # assert!(v);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     # let a = 1;
//!     # let v =
//!     1 == a || a == 2
//!     # ;
//!     # assert!(v);
//!     ```
//!   - batch op
//!     ```rust
//!     # use batch_oper::*;
//!     # let v =
//!     bop!(&& 5; > ; 2, 3, 6;!);
//!     # assert!(v);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     # let v =
//!     5 > 2 && 5 > 3 && 6 > 5;
//!     # assert!(v);
//!     ```
//! - **Set**
//!   ```rust
//!   # use batch_oper::*;
//!   let mut a = 1;
//!   bop!(= a; + 1, - 2;!, * 3);
//!   # assert_eq!(a, 0);
//!   ```
//!   *equivalent to*
//!   ```rust
//!   let mut a = 1;
//!   a = a + 1;
//!   a = 2 - a;
//!   a = a * 3;
//!   # assert_eq!(a, 0);
//!   ```
//! - **Let**
//!   ```rust
//!   # use batch_oper::*;
//!   bop! { let a|u8 = 1, mut b = 2 }
//!   # assert_eq!(a, 1);
//!   # assert_eq!(b, 2);
//!   ```
//!   *equivalent to*
//!   ```no_run
//!   let a: u8 = 1;
//!   let mut b = 2;
//!   ```
//! - **Let chain**
//!   - basic
//!     ```rust
//!     # use batch_oper::*;
//!     let a = Some(1);
//!     let b = Some(2);
//!     
//!     let v: i32 = bop!(match && Some(va) = a, Some(vb) = b => {
//!         1
//!     } else {
//!         2
//!     });
//!     # assert_eq!(v, 1);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     let a = Some(1);
//!     let b = Some(2);
//!     
//!     let v: i32 = loop {
//!         if let Some(va) = a {
//!             if let Some(vb) = b {
//!                 break { 1 };
//!             }
//!         }
//!         break { 2 };
//!     };
//!     # assert_eq!(v, 1);
//!     ```
//!   - `bool`
//!     ```rust
//!     # use batch_oper::*;
//!     # let a = Some(1);
//!     # let b = Some(2);
//!     let v: bool = bop!(bool match && Some(va) = a, Some(vb) = b => {
//!         1
//!     } else {
//!         2
//!     });
//!     # assert!(v);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     # let a = Some(1);
//!     # let b = Some(2);
//!     let v: bool = loop {
//!         if let Some(va) = a {
//!             if let Some(vb) = b {
//!                 { 1 };
//!                 break true;
//!             }
//!         }
//!         { 2 };
//!         break false;
//!     };
//!     # assert!(v);
//!     ```
//!   - `!loop`
//!     ```rust
//!     # use batch_oper::*;
//!     # let a = Some(1);
//!     # let b = Some(2);
//!     let v: i32 = bop!(!loop match && Some(va) = a, Some(vb) = b => {
//!         1
//!     } else {
//!         2
//!     });
//!     # assert_eq!(v, 1)
//!     ```
//!     *equivalent to*
//!     ```rust
//!     # let a = Some(1);
//!     # let b = Some(2);
//!     let v: i32 = if let Some(va) = a {
//!         if let Some(vb) = b {
//!             { 1 }
//!         } else { { 2 } }
//!     } else  { { 2 } };
//!     # assert_eq!(v, 1);
//!     ```
//!   - `!loop bool`
//!     ```rust
//!     # use batch_oper::*;
//!     # let a = Some(1);
//!     # let b = Some(2);
//!     let v: bool = bop!(!loop bool match && Some(va) = a, Some(vb) = b => {
//!         1
//!     } else {
//!         2
//!     });
//!     # assert!(v);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     # let a = Some(1);
//!     # let b = Some(2);
//!     let v: bool = if let Some(va) = a {
//!         if let Some(vb) = b {
//!             { 1 }; true
//!         } else { { 2 }; false }
//!     } else  { { 2 }; false };
//!     # assert!(v);
//!     ```
//! - **In**
//!   ```rust
//!   # use batch_oper::*;
//!   let r = 0..5;
//!   let c = bop!(&1, &2 => in && r);
//!   # assert!(c);
//!   ```
//!   *equivalent to*
//!   ```rust
//!   let r = 0..5;
//!   let c = r.contains(&1) && r.contains(&2);
//!   # assert!(c);
//!   ```
//!   - `||`
//!     ```rust
//!     # use batch_oper::*;
//!     # let r = 0..5;
//!     let c = bop!(&1, &2 => in || r);
//!     # assert!(c);
//!     ```
//!     *equivalent to*
//!     ```rust
//!     # let r = 0..5;
//!     let c = r.contains(&1) || r.contains(&2);
//!     # assert!(c);
//!     ```
//!   - custom funcion name
//!     ```ignore
//!     let c = bop!(has; &1, &2 => in && r);
//!     ```
//!     *equivalent to*
//!     ```ignore
//!     let c = r.has(&1) && r.has(&2);
//!     ```
//! - `Using`
//!   ```rust
//!   # use batch_oper::using;
//!   let v = (1, 2);
//!   let v2 = (3, 4);
//!   using!((a, b) = v, (c, d) = v2; {
//!     println!("{} {} {} {}", a, b, c, d)
//!     # ;
//!     # assert_eq!(a, 1);
//!     # assert_eq!(b, 2);
//!     # assert_eq!(c, 3);
//!     # assert_eq!(d, 4);
//!   })
//!   ```
//!   *equivalent to*
//!   ```no_run
//!   let v = (1, 2);
//!   let v2 = (3, 4);
//!   {
//!     let (a, b) = v;
//!     let (c, d) = v2;
//!     {
//!       println!("{} {} {} {}", a, b, c, d)
//!     }
//!   }
//!   ```
//!

#[doc(hidden)]
#[macro_export]
macro_rules! _matchand {
    { ; $b:block $($el:block)? } => { $b };
    { $p:pat = $e:expr; { } $($pp:pat = $ee:expr; { })+ ; $b:block $($el:block)?} => {
        if let $p = $e { _matchand!($($pp = $ee ; {})* ; $b $($el)?) } $(else $el)?
    };
    { $p:pat = $e:expr; { } ; $b:block $($el:block)? } => { if let $p = $e $b $(else $el)? };
}
#[doc(hidden)]
#[macro_export]
macro_rules! _select_op {
    { $x:expr ; $op:tt $a:expr } => { $x $op $a };
    { $x:expr ; $op:tt $a:expr ; !  } => { $a $op $x };
}
/// batch opers
/// ## Usage
/// - **Basic**
///   - batch `||`
///     ```rust
///     # use batch_oper::*;
///     # let v =
///     bop!(|| 4; == 2, > 3);
///     # assert!(v);
///     ```
///     *equivalent to*
///     ```rust
///     # let v =
///     4 == 2 || 4 > 3;
///     # assert!(v);
///     ```
///   - batch `&&`
///     ```rust
///     # use batch_oper::*;
///     # let v =
///     bop!(&& 4; == 2, > 3);
///     # assert!(!v);
///     ```
///     *equivalent to*
///     ```rust
///     # let v =
///     4 == 2 && 4 > 3;
///     # assert!(!v);
///     ```
///   - `!`
///     ```rust
///     # use batch_oper::*;
///     # let a = 1;
///     # let v =
///     bop!(|| a; == 1;!, == 2);
///     # assert!(v);
///     ```
///     *equivalent to*
///     ```rust
///     # let a = 1;
///     # let v =
///     1 == a || a == 2
///     # ;
///     # assert!(v);
///     ```
///   - batch op
///     ```rust
///     # use batch_oper::*;
///     # let v =
///     bop!(&& 5; > ; 2, 3, 6;!);
///     # assert!(v);
///     ```
///     *equivalent to*
///     ```rust
///     # let v =
///     5 > 2 && 5 > 3 && 6 > 5;
///     # assert!(v);
///     ```
/// - **Set**
///   ```rust
///   # use batch_oper::*;
///   let mut a = 1;
///   bop!(= a; + 1, - 2;!, * 3);
///   # assert_eq!(a, 0);
///   ```
///   *equivalent to*
///   ```rust
///   let mut a = 1;
///   a = a + 1;
///   a = 2 - a;
///   a = a * 3;
///   # assert_eq!(a, 0);
///   ```
/// - **Let**
///   ```rust
///   # use batch_oper::*;
///   bop! { let a|u8 = 1, mut b = 2 }
///   # assert_eq!(a, 1);
///   # assert_eq!(b, 2);
///   ```
///   *equivalent to*
///   ```no_run
///   let a: u8 = 1;
///   let mut b = 2;
///   ```
/// - **Let chain**
///   - basic
///     ```rust
///     # use batch_oper::*;
///     let a = Some(1);
///     let b = Some(2);
///
///     let v: i32 = bop!(match && Some(va) = a, Some(vb) = b => {
///         1
///     } else {
///         2
///     });
///     # assert_eq!(v, 1);
///     ```
///     *equivalent to*
///     ```rust
///     let a = Some(1);
///     let b = Some(2);
///
///     let v: i32 = loop {
///         if let Some(va) = a {
///             if let Some(vb) = b {
///                 break { 1 };
///             }
///         }
///         break { 2 };
///     };
///     # assert_eq!(v, 1);
///     ```
///   - `bool`
///     ```rust
///     # use batch_oper::*;
///     # let a = Some(1);
///     # let b = Some(2);
///     let v: bool = bop!(bool match && Some(va) = a, Some(vb) = b => {
///         1
///     } else {
///         2
///     });
///     # assert!(v);
///     ```
///     *equivalent to*
///     ```rust
///     # let a = Some(1);
///     # let b = Some(2);
///     let v: bool = loop {
///         if let Some(va) = a {
///             if let Some(vb) = b {
///                 { 1 };
///                 break true;
///             }
///         }
///         { 2 };
///         break false;
///     };
///     # assert!(v);
///     ```
///   - `!loop`
///     ```rust
///     # use batch_oper::*;
///     # let a = Some(1);
///     # let b = Some(2);
///     let v: i32 = bop!(!loop match && Some(va) = a, Some(vb) = b => {
///         1
///     } else {
///         2
///     });
///     # assert_eq!(v, 1)
///     ```
///     *equivalent to*
///     ```rust
///     # let a = Some(1);
///     # let b = Some(2);
///     let v: i32 = if let Some(va) = a {
///         if let Some(vb) = b {
///             { 1 }
///         } else { { 2 } }
///     } else  { { 2 } };
///     # assert_eq!(v, 1);
///     ```
///   - `!loop bool`
///     ```rust
///     # use batch_oper::*;
///     # let a = Some(1);
///     # let b = Some(2);
///     let v: bool = bop!(!loop bool match && Some(va) = a, Some(vb) = b => {
///         1
///     } else {
///         2
///     });
///     # assert!(v);
///     ```
///     *equivalent to*
///     ```rust
///     # let a = Some(1);
///     # let b = Some(2);
///     let v: bool = if let Some(va) = a {
///         if let Some(vb) = b {
///             { 1 }; true
///         } else { { 2 }; false }
///     } else  { { 2 }; false };
///     # assert!(v);
///     ```
/// - **In**
///   ```rust
///   # use batch_oper::*;
///   let r = 0..5;
///   let c = bop!(&1, &2 => in && r);
///   # assert!(c);
///   ```
///   *equivalent to*
///   ```rust
///   let r = 0..5;
///   let c = r.contains(&1) && r.contains(&2);
///   # assert!(c);
///   ```
///   - `||`
///     ```rust
///     # use batch_oper::*;
///     # let r = 0..5;
///     let c = bop!(&1, &2 => in || r);
///     # assert!(c);
///     ```
///     *equivalent to*
///     ```rust
///     # let r = 0..5;
///     let c = r.contains(&1) || r.contains(&2);
///     # assert!(c);
///     ```
///   - custom funcion name
///     ```ignore
///     let c = bop!(has; &1, &2 => in && r);
///     ```
///     *equivalent to*
///     ```ignore
///     let c = r.has(&1) && r.has(&2);
///     ```
#[macro_export(local_inner_macros)]
macro_rules! bop {
    {} => { };

    // let op
    { let $($p:pat $(| $t:ty)? $(= $e:expr)?),*} => { $(let $p $(: $t)? $(= $e)?;)* };

    // if let op
    { $($l:lifetime :)? match && $($p:pat = $e:expr),* => $b:block else $el:block } => {
        $($l :)? loop { _matchand!( $( $p = $e ; { } )* ; { break $($l)? $b ; }) ; break $($l)? $el; }
    };
    { bool $($l:lifetime :)? match && $($p:pat = $e:expr),* => $b:block $(else $el:block)? } => {
        $($l :)? loop { _matchand!( $( $p = $e ; { } )* ; { $b ; break $($l)? true; }) ; $($el ;)? break $($l)? false; }
    };
    { !loop match && $($p:pat = $e:expr),* => $b:block else $el:block } => {
        _matchand!( $( $p = $e ; { } )* ; { $b } { $el })
    };
    { !loop bool match && $($p:pat = $e:expr),* => $b:block $(else $el:block)? } => {
        _matchand!( $( $p = $e ; { } )* ; { $b ; true } { $($el ;)? false })
    };

    // base op
    { $x:expr $(;)? } => { $x };
    { || $x:expr $(;)? } => { $x };
    { && $x:expr $(;)? } => { $x };
    { = $x:ident $(;)? } => { };
    { || $x:expr ; $($op:tt $a:expr $(;$n:tt)?),* } => { $(_select_op!($x; $op $a $(;$n)?))||* };
    { && $x:expr ; $($op:tt $a:expr $(;$n:tt)?),* } => { $(_select_op!($x; $op $a $(;$n)?))&&* };
    { = $x:ident ; $($op:tt $a:expr $(;$n:tt)?),* } => { $($x = _select_op!($x; $op $a $(;$n)?));* ; };
    // batch op
    { || $x:expr ; $op:tt $(;)? } => { $x };
    { && $x:expr ; $op:tt $(;)? } => { $x };
    { = $x:ident ; $op:tt $(;)? } => { };
    { || $x:expr ; $op:tt ; $($a:expr $(;$n:tt)?),* } => { $(_select_op!($x; $op $a $(;$n)?))||* };
    { && $x:expr ; $op:tt ; $($a:expr $(;$n:tt)?),* } => { $(_select_op!($x; $op $a $(;$n)?))||* };
    { = $x:ident ; $op:tt ; $($a:expr $(;$n:tt)?),* } => { $($x = _select_op!($x; $op $a $(;$n)?));* ; };

    // inop
    { $fname:ident ; $($v:expr),* => in && $t:expr } => { $($t.$fname($v))&&* };
    { $fname:ident ; $($v:expr),* => in || $t:expr } => { $($t.$fname($v))||* };
    { $fname:ident ; $v:expr => in && $($t:expr),* } => { $($t.$fname($v))&&* };
    { $fname:ident ; $v:expr => in || $($t:expr),* } => { $($t.$fname($v))||* };

    { $($fname:ident ;)? => in && $t:expr  } => { false };
    { $($fname:ident ;)? => in || $t:expr  } => { false };
    { $($fname:ident ;)? $v:expr => in && } => { false };
    { $($fname:ident ;)? $v:expr => in || } => { false };

    { $($v:expr),* => in && $t:expr } => { $($t.contains($v))&&* };
    { $($v:expr),* => in || $t:expr } => { $($t.contains($v))||* };
    { $v:expr => in && $($t:expr),* } => { $($t.contains($v))&&* };
    { $v:expr => in || $($t:expr),* } => { $($t.contains($v))||* };
}

/// using
/// ## Usage
/// ```rust
/// # use batch_oper::*;
/// let v = (1, 2);
/// let v2 = (3, 4);
/// using!((a, b) = v, (c, d) = v2; {
///   println!("{} {} {} {}", a, b, c, d)
///   # ;
///   # assert_eq!(a, 1);
///   # assert_eq!(b, 2);
///   # assert_eq!(c, 3);
///   # assert_eq!(d, 4);
/// })
/// ```
/// *equivalent to*
/// ```no_run
/// let v = (1, 2);
/// let v2 = (3, 4);
/// {
///   let (a, b) = v;
///   let (c, d) = v2;
///   {
///     println!("{} {} {} {}", a, b, c, d)
///   }
/// }
///   ```
#[macro_export(local_inner_macros)]
macro_rules! using {
    { $($p:pat = $v:expr),* ; $b:block } => {
        { $(let $p = $v ;)* $b }
    };
}

/// Create an implicit variable, perform some side effects, and return it
/// ## Example
/// ```rust
/// # use batch_oper::effect;
/// let v = 1;
/// let v = effect(v, |v| { assert_eq!(*v, 1) });
/// assert_eq!(v, 1);
/// ```
#[inline(always)]
pub fn effect<T>(v: T, f: impl FnOnce(&T)) -> T {
    f(&v);
    v
}
/// Create an implicit variable, and make a mapping for it
/// ## Example
/// ```rust
/// # use batch_oper::using;
/// let v = 1;
/// let mut v = using(v, |v| { v + 1 });
/// assert_eq!(v, 2);
/// using(&mut v, |v| { *v = 3 });
/// assert_eq!(v, 3);
/// ```
#[inline(always)]
pub fn using<T, R>(v: T, f: impl FnOnce(T) -> R) -> R {
    f(v)
}

/// Create an implicit variable, perform some side effects, and return it
/// ## Example
/// ```rust
/// # use batch_oper::Effect;
/// let v = Some(1);
/// let v = v.effect(|v| { assert_eq!(*v, 1) });
/// assert_eq!(v, Some(1));
/// ```
pub trait Effect<T> {
    /// Create an implicit variable, perform some side effects, and return it
    /// ## Example
    /// ```rust
    /// # use batch_oper::Effect;
    /// let v = Some(1);
    /// let v = v.effect(|v| { assert_eq!(*v, 1) });
    /// assert_eq!(v, Some(1));
    /// ```
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self;
}
impl<T> Effect<T> for Option<T> {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Some(ref v) = self {
            f(v);
        }
        self
    }
}
impl<T, E> Effect<T> for Result<T, E> {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Ok(ref v) = self {
            f(v);
        }
        self
    }
}
impl<T, S: core::ops::Deref<Target = T>> Effect<T> for &S {
    #[inline(always)]
    fn effect<F: FnOnce(&T)>(self, f: F) -> Self {
        f(self.deref());
        self
    }
}

/// new a `Box<T>`  
/// ```
/// # use batch_oper::*;
/// # let xxx = 1;
/// new!(xxx)
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # let xxx = 1;
/// Box::new(xxx)
/// # ;
/// ```
#[macro_export]
macro_rules! new {
    () => {
        Box::new(Default::default())
    };
    ($e:expr) => {
        Box::new($e)
    };
}

/// new a `Box<[T]>`  
/// ```
/// # use batch_oper::*;
/// # let a = 1; let b = 2; let c = 3;
/// arr![a, b, c]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # let a = 1; let b = 2; let c = 3;
/// Box::new([a, b, c])
/// # ;
/// ```
#[macro_export]
macro_rules! arr {
    [ $($e:expr),* $(,)? ] => { Box::new([$($e),*]) };
}

/// new a `VecDeque<T>`
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// deque![a, b, c]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// VecDeque::from(vec![a, b, c])
/// # ;
/// ```
#[macro_export]
macro_rules! deque {
    [] => { std::collections::VecDeque::new() };
    [ $elem:expr; $n:expr ] => { std::collections::VecDeque::from(vec![$elem; $n]) };
    [ $($e:expr),+ $(,)? ] => { std::collections::VecDeque::from(vec![$($e),+]) };
}

/// new a `LinkedList<T>`  
///
/// ----------  
/// ### Push Back  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// list![a, b, c]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// {
///     let mut l = LinkedList::new();
///     l.push_back(a);
///     l.push_back(b);
///     l.push_back(c);
///     l
/// }
///
/// # ;
/// ```
///   
/// ----------  
/// ### Push Front  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// list![<- a, b, c]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2; let c = 3;
/// {
///     let mut l = LinkedList::new();
///     l.push_front(a);
///     l.push_front(b);
///     l.push_front(c);
///     l
/// }
///
/// # ;
/// ```
///   
/// ----------  
/// ### From Elem  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1;
/// list![a; 3]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1;
/// {
///     let mut l = LinkedList::new();
///     for _ in 0..3 {
///         l.push_back(a);
///     }
///     l
/// }
///
/// # ;
/// ```
#[macro_export]
macro_rules! list {
    [] => { std::collections::LinkedList::new() };
    [ $elem:expr; $n:expr ] => {{
        let mut l = std::collections::LinkedList::new();
        for _ in 0..$n {
            l.push_back($elem);
        }
        l
    }};
    [ $($e:expr),+ $(,)? ] => {{
        let mut l = std::collections::LinkedList::new();
        $( l.push_back($e); )+
        l
    }};
    [ <- $($e:expr),+ $(,)? ] => {{
        let mut l = std::collections::LinkedList::new();
        $( l.push_front($e); )+
        l
    }};
}

/// new a `HashMap<K, V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// map! {
///     ka => va,
///     kb => vb,
/// }
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// {
///     let mut m = HashMap::new();
///     m.insert(ka, va);
///     m.insert(kb, vb);
///     m
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! map {
    { } => { std::collections::HashMap::new() };
    { $($k:expr => $v:expr),+ $(,)? } => {{
        let mut m = std::collections::HashMap::new();
        $(
            m.insert($k, $v);
        )+
        m
    }};
}

/// new a `BTreeMap<K, V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// btmap! {
///     ka => va,
///     kb => vb,
/// }
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// {
///     let mut m = BTreeMap::new();
///     m.insert(ka, va);
///     m.insert(kb, vb);
///     m
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! btmap {
    { } => { std::collections::BTreeMap::new() };
    { $($k:expr => $v:expr),+ $(,)? } => {{
        let mut m = std::collections::BTreeMap::new();
        $(
            m.insert($k, $v);
        )+
        m
    }};
}

/// append items to a map  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// let mut m = HashMap::new();
/// map_append! { m;
///     ka => va,
///     kb => vb,
/// }
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let ka = 1; let va = 2; let kb = 3; let vb = 4;
/// let mut m = HashMap::new();
/// m.insert(ka, va);
/// m.insert(kb, vb);
/// # ;
/// ```
#[macro_export]
macro_rules! map_append {
    { $m:expr; $($k:expr => $v:expr),+ $(,)? } => {
        $(
            $m.insert($k, $v);
        )+
    };
}

/// new a `HashSet<V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// set![a, b]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// {
///     let mut s = HashSet::new();
///     s.insert(a);
///     s.insert(b);
///     s
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! set {
    { } => { std::collections::HashSet::new() };
    { $($e:expr),+ $(,)? } => {{
        let mut s = std::collections::HashSet::new();
        $(
            s.insert($e);
        )+
        s
    }};
}

/// new a `BTreeSet<V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// btset![a, b]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// {
///     let mut s = BTreeSet::new();
///     s.insert(a);
///     s.insert(b);
///     s
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! btset {
    { } => { std::collections::BTreeSet::new() };
    { $($e:expr),+ $(,)? } => {{
        let mut s = std::collections::BTreeSet::new();
        $(
            s.insert($e);
        )+
        s
    }};
}

/// new a `BinaryHeap<V>`  
/// ```
/// # use batch_oper::*;
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// heap![a, b]
/// # ;
/// ```
/// *equivalent to*
/// ```
/// # use std::collections::*;
/// # let a = 1; let b = 2;
/// BinaryHeap::from(vec![a, b])
/// # ;
/// ```
#[macro_export]
macro_rules! heap {
    [ ] => { std::collections::BinaryHeap::new() };
    [ $elem:expr; $n:expr ] => { std::collections::BinaryHeap::from(vec![$elem; $n]) };
    [ $($e:expr),+ $(,)? ] => { std::collections::BinaryHeap::from(vec![$($e),+]) };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x = bop!(|| 4 ; == 2, > 3);
        assert!(x);

        let mut a = 1;
        bop!(= a ; + 1, - 2;!, + 3);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_batch() {
        let x = bop!(|| 1 ; < ; 5, 6, 7, 0;!);
        assert!(x)
    }

    #[test]
    fn test_let() {
        bop! { let a |u8 = 1, b = 2 }
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }

    #[test]
    fn test_match() {
        let a = Some(1);
        let b = Some(2);

        let _: i32 = bop!('a: match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });

        let _: bool = bop!(bool 'b: match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });

        let _: i32 = bop!(!loop match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });

        let _: bool = bop!(!loop bool match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });
    }

    #[test]
    fn test_in() {
        let r = 0..5;
        let c = bop!(&1, &2 => in && r);
        assert!(c);
    }

    #[test]
    fn test_using() {
        let v = (1, 2);
        let v2 = (3, 4);
        using!((a, b) = v, (c, d) = v2 ; {
            println!("{} {} {} {}", a, b, c, d)
        })
    }

    #[test]
    fn test_fn() {
        let v = 1;
        let v = effect(v, |v| assert_eq!(*v, 1));
        assert_eq!(v, 1);
        let mut v = using(v, |v| v + 1);
        assert_eq!(v, 2);
        using(&mut v, |v| *v = 3);
        assert_eq!(v, 3);
    }
}
