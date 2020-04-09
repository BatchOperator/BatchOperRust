//! batch_oper provides some batch operation macro for some operations
//! ## Usage
//! - **Basic**  
//!   - batch `||`  
//!     ```rust  
//!     bop!(|| 4; == 2, > 3);
//!     ```
//!     equivalent to
//!     ```rust
//!     4 == 2 || 4 > 3
//!     ```
//!   - batch `&&`  
//!     ```rust  
//!     bop!(&& 4; == 2, > 3);
//!     ```
//!     equivalent to
//!     ```rust
//!     4 == 2 && 4 > 3
//!     ```
//!   - `!`
//!     ```rust
//!     bop!(|| a; == 1;!, == 2);
//!     ```
//!     equivalent to
//!     ```rust
//!     1 == a || a == 2
//!     ```
//! - **Set**
//!   ```rust
//!   let mut a = 1;
//!   bop!(= a; + 1, - 2;!, * 3);
//!   ```
//!   equivalent to
//!   ```rust
//!   let mut a = 1;
//!   a = a + 1;
//!   a = 2 - a;
//!   a = a * 3;
//!   ```
//! - **Let**
//!   ```rust
//!   bop! { let a|u8 = 1, b = 2 }
//!   ```
//!   equivalent to
//!   ```rust
//!   let a: u8 = 1;
//!   let b = 2;
//!   ```

macro_rules! _matchand {
    { ; $b:block $($el:block)? } => { $b };
    { $p:pat = $e:expr; { } $($pp:pat = $ee:expr; { })+ ; $b:block $($el:block)?} => {
        if let $p = $e { _matchand!($($pp = $ee ; {})* ; $b $($el)?) } $(else $el)?
    };
    { $p:pat = $e:expr; { } ; $b:block $($el:block)? } => { if let $p = $e $b $(else $el)? };
}
macro_rules! _select_op {
    { $x:expr ; $op:tt $a:expr } => { $x $op $a };
    { $x:expr ; $op:tt $a:expr ; !  } => { $a $op $x };
}
#[macro_export]
macro_rules! bop {
    {} => { };

    // let op
    { let $($p:pat $(| $t:ty)? $(= $e:expr)?),*} => { $(let $p $(: $t)? $(= $e)?;)* };

    // if let op
    { match && $($p:pat = $e:expr),* => $b:block else $el:block } => {
        loop { _matchand!( $( $p = $e ; { } )* ; { break $b ; }) ; break $el; }
    };
    { bool match && $($p:pat = $e:expr),* => $b:block $(else $el:block)? } => {
        loop { _matchand!( $( $p = $e ; { } )* ; { $b ; break true; }) ; $($el ;)? break false; }
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
    { = $x:expr $(;)? } => { $x };
    { || $x:expr ; $($op:tt $a:expr $(;$n:tt)?),* } => { $(_select_op!($x; $op $a $(;$n)?))||* };
    { && $x:expr ; $($op:tt $a:expr $(;$n:tt)?),* } => { $(_select_op!($x; $op $a $(;$n)?))&&* };
    { = $x:ident ; $($op:tt $a:expr $(;$n:tt)?),* } => { $($x = _select_op!($x; $op $a $(;$n)?));* ; };

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

#[macro_export]
macro_rules! using {
    { $($p:pat = $v:expr),* ; $b:block } => {
        { $(let $p = $v ;)* $b }
    };
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
        assert_eq!(a, 7);
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

        let _: i32 = bop!(match && Some(va) = a, Some(vb) = b => {
            println!("some {} {}", va, vb);
            1
        } else {
            2
        });

        let _: bool = bop!(bool match && Some(va) = a, Some(vb) = b => {
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
        using!((a, b) = v ; {
            println!("some {} {}", a, b)
        })
    }
}
