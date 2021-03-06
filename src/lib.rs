//! libsugar provides syntactic sugar in the form of a library
//! 
//! ## Features
//!
//! default = `["std", "combin", "named-into", "macro-lit", "side-effect", "re-exports", "chain_panic", "chain_todo", "tuples", "once_get", "chain_drop"]`  
//!
//! - `"std"` Enable std  
//! - `"side-effect"` Enable mod [side_effect](side_effect/index.html)  
//! - `"named-into"` Enable mod [named_into](named_into/index.html)  
//! - `"combin"` Enable mod [combin](combin/index.html)  
//! - `"macro-lit"` Enable macro like [new](macro.new.html), [list](macro.list.html)  
//! - `"chain_panic"` Enable mod [chain_panic](chain_panic/index.html)
//! - `"chain_todo"` Enable mod [chain_todo](chain_todo/index.html)
//! - `"chain_drop"` Enable mod [chain_drop](chain_drop/index.html)
//! - `"tuples"` Enable mod [tuples](tuples/index.html)  
//! - `"once_get"` Enable mod [once_get](once_get/index.html)  
//! - `"re-exports"` Enable re-export of all mods  
//!

#![cfg_attr(not(feature = "std"), no_std)]

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
///     # use libsugar::*;
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
///     # use libsugar::*;
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
///     # use libsugar::*;
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
///     # use libsugar::*;
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
///   # use libsugar::*;
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
///   # use libsugar::*;
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
///     # use libsugar::*;
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
///     # use libsugar::*;
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
///     # use libsugar::*;
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
///     # use libsugar::*;
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
///   # use libsugar::*;
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
///     # use libsugar::*;
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

#[cfg(feature = "side-effect")]
pub mod side_effect;
#[cfg(all(feature = "side-effect", feature = "re-exports"))]
pub use side_effect::*;

#[cfg(feature = "macro-lit")]
mod macro_lit;
#[cfg(feature = "macro-lit")]
pub use macro_lit::*;

#[cfg(feature = "named-into")]
pub mod named_into;
#[cfg(all(feature = "named-into", feature = "re-exports"))]
pub use named_into::*;

#[cfg(feature = "combin")]
pub mod combin;
#[cfg(all(feature = "combin", feature = "re-exports"))]
pub use combin::*;

#[cfg(all(feature = "chain_panic", feature = "std"))]
pub mod chain_panic;
#[cfg(all(feature = "chain_panic", feature = "std", feature = "re-exports"))]
pub use chain_panic::*;

#[cfg(feature = "chain_todo")]
pub mod chain_todo;
#[cfg(all(feature = "chain_todo", feature = "re-exports"))]
pub use chain_todo::*;

#[cfg(feature = "tuples")]
/// Re-exports from [tuples](https://crates.io/crates/tuples)  
pub mod tuples {
    pub use tuples::*;
}
#[cfg(all(feature = "tuples", feature = "re-exports"))]
pub use crate::tuples::*;

#[cfg(feature = "once_get")]
pub mod once_get;
#[cfg(all(feature = "once_get", feature = "re-exports"))]
pub use crate::once_get::*;

#[cfg(feature = "chain_drop")]
pub mod chain_drop;
#[cfg(all(feature = "chain_drop", feature = "re-exports"))]
pub use chain_drop::*;

#[cfg(test)]
mod tests;
