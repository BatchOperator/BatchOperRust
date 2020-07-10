//! No nesting combine
//! 
//! # Tuple example
//! 
//! ```
//! # use batch_oper::combin::*;
//! let a: (i32, u8) = 1.with(2u8);
//! assert_eq!(a, (1, 2));
//! let b: (i32, u8, f64) = a.with(3f64);
//! assert_eq!(b, (1, 2, 3.0));
//! let c: (usize, i32, u8, f64) = b.after(0usize);
//! assert_eq!(c, (0, 1, 2, 3.0));
//! ```
//! 
//! # Array example
//! 
//! ```
//! # use batch_oper::combin::*;
//! let a: [u8; 2] = 1.with(2);
//! assert_eq!(a, [1, 2]);
//! let b: [u8; 3] = a.with(3);
//! assert_eq!(b, [1, 2, 3]);
//! let c: [u8; 4] = b.after(0);
//! assert_eq!(c, [0, 1, 2, 3]);
//! ```
//! 
//! *[With](trait.With.html) is a alias for [Before](trait.Before.html)*

/// No nesting combine  
/// Add at the end  
pub trait After<T, Output> {
    /// No nesting combine  
    /// Add at the end
    fn after(self, v: T) -> Output;
}
/// No nesting combine
pub trait Before<T, Output> {
    /// No nesting combine
    fn before(self, v: T) -> Output;
}

/// No nesting combine  
/// Same to [Before](trait.Before.html)  
pub trait With<T, Output> {
    /// No nesting combine  
    /// Same to [Before](trait.Before.html)  
    fn with(self, v: T) -> Output;
}
impl<A: Before<B, O>, B, O> With<B, O> for A {
    #[inline(always)]
    fn with(self, v: B) -> O {
        self.before(v)
    }
}

macro_rules! do_impl_tuple {
    { } => { };
    { $($t:ident),* } => {
        do_impl_tuple! { , $($t),* }
        do_impl_tuple! { ; $($t),* }
    };
    { , $h:ident $(, $($t:ident),*)? } => {
        do_impl_tuple! { $($($t),*)? }
    };
    { ; $($t:ident),* } => {
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<$($t),*, Ta> After<Ta, (Ta, $($t),*)> for ($($t),*) {
            #[inline(always)]
            fn after(self, v: Ta) -> (Ta, $($t),*) {
                let ($($t),*) = self;
                (v, $($t),*)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<$($t),*, Ta> Before<Ta, ($($t),*, Ta)> for ($($t),*) {
            #[inline(always)]
            fn before(self, v: Ta) -> ($($t),*, Ta) {
                let ($($t),*) = self;
                ($($t),*, v)
            }
        }
    };
}
do_impl_tuple! {
    z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a,
    Z, Y, X, W, V, U, T, S, R, Q, P, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A
}

macro_rules! do_impl_array {
    { } => { };
    { : $i:ident } => { 1 };
    { $($t:ident),* } => {
        do_impl_array! { , $($t),* }
        do_impl_array! { ; $($t),* }
    };
    { , $h:ident $(, $($t:ident),*)? } => {
        do_impl_array! { $($($t),*)? }
    };
    { ; $($t:ident),* } => {
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<T> After<T, [T; 1 + $(do_impl_array!(:$t) + )* 0 ]> for [T; $(do_impl_array!(:$t) + )* 0 ] {
            #[inline(always)]
            fn after(self, v: T) -> [T; 1 + $(do_impl_array!(:$t) + )* 0 ] {
                let [$($t),*] = self;
                [v, $($t),*]
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<T> Before<T, [T; 1 + $(do_impl_array!(:$t) + )* 0 ]> for [T; $(do_impl_array!(:$t) + )* 0 ] {
            #[inline(always)]
            fn before(self, v: T) -> [T; 1 + $(do_impl_array!(:$t) + )* 0 ] {
                let [$($t),*] = self;
                [$($t),*, v]
            }
        }
    };
}
impl<T> After<T, [T; 1 + 1]> for T {
    fn after(self, v: T) -> [T; 1 + 1] {
        [v, self]
    }
}
impl<T> Before<T, [T; 1 + 1]> for T {
    fn before(self, v: T) -> [T; 1 + 1] {
        [self, v]
    }
}
do_impl_array! {
    z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a,
    Z, Y, X, W, V, U, T, S, R, Q, P, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combin_tuple() {
        let a: (i32, u8) = 1.with(2u8);
        assert_eq!(a, (1, 2));
        let b: (i32, u8, f64) = a.with(3f64);
        assert_eq!(b, (1, 2, 3.0));
        let c: (usize, i32, u8, f64) = b.after(0usize);
        assert_eq!(c, (0, 1, 2, 3.0));
    }

    #[test]
    fn test_combin_array() {
        let a: [u8; 2] = 1.with(2);
        assert_eq!(a, [1, 2]);
        let b: [u8; 3] = a.with(3);
        assert_eq!(b, [1, 2, 3]);
        let c: [u8; 4] = b.after(0);
        assert_eq!(c, [0, 1, 2, 3]);
    }
}