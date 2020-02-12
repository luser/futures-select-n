#![feature(type_alias_impl_trait)]

use futures::future::{self, Either, Future};

pub trait SelectN {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn select_n(self) -> Self::Future;
}

#[derive(Debug, Clone, PartialEq)]
pub enum E3<A, B, C> {
    N1(A),
    N2(B),
    N3(C),
}

#[derive(Debug, Clone, PartialEq)]
pub enum E4<A, B, C, D> {
    N1(A),
    N2(B),
    N3(C),
    N4(D),
}

/*

I'd like to be able to generate versions of `enum E` above with an arbitrary number of variants, each with a type parameter but my macro-fu is not strong enough.

macro_rules! with_type_params {
    ($t:ty: $($i:expr),+) => {
        paste::item! {
            enum [< $t<$( with_type_params!(@p $i) )+> >] { X }
        }
    };

}

with_type_params!(Foo: 1, 2, 3);


macro_rules! decl_one {
    ($i:expr; $($n:expr),+) => {
        paste::item! {
            #[derive(Debug, Clone, PartialEq)]
            pub enum [<E $i>]< decl_one!(@p 1) > {
                $(
                    [<N $n >],
                )+
            }
        }
    };
    (@p 1) => { A };
(@p 2) => { B };
(@p 3) => { C };
(@p 4) => { D };
(@p 5) => { E };
(@p 6) => { F };
(@p 7) => { G };
(@p 8) => { H };
(@p 9) => { I };
(@p 10) => { J };
(@p 11) => { K };
(@p 12) => { L };
(@p 13) => { M };
(@p 14) => { N };
(@p 15) => { O };
(@p 16) => { P };
(@p 17) => { Q };
(@p 18) => { R };
(@p 19) => { S };
(@p 20) => { T };
(@p 21) => { U };
(@p 22) => { V };
(@p 23) => { W };
(@p 24) => { X };
(@p 25) => { Y };
(@p 26) => { Z };
}

decl_one!(3; 1, 2, 3);


macro_rules! decl_enums {
    () => {};
    ([$($n:expr),*] $i:expr, $($t:tt),*) => {
        decl_one!($i; $($n,)* $i);
    }
}

decl_enums!([1, 2] 3, );
*/

impl<A, B, C> SelectN for (A, B, C)
where
    A: Future + Unpin + Send + Sync,
    B: Future + Unpin + Send + Sync,
    C: Future + Unpin + Send + Sync,
{
    type Output = E3<A::Output, B::Output, C::Output>;
    type Future = impl Future<Output = Self::Output>;

    fn select_n(self) -> Self::Future {
        async {
            match future::select(self.0, future::select(self.1, self.2)).await {
                Either::Left((a, _)) => E3::N1(a),
                Either::Right((Either::Left((b, _)), _)) => E3::N2(b),
                Either::Right((Either::Right((c, _)), _)) => E3::N3(c),
            }
        }
    }
}

impl<A, B, C, D> SelectN for (A, B, C, D)
where
    A: Future + Unpin + Send + Sync,
    B: Future + Unpin + Send + Sync,
    C: Future + Unpin + Send + Sync,
    D: Future + Unpin + Send + Sync,
{
    type Output = E4<A::Output, B::Output, C::Output, D::Output>;
    type Future = impl Future<Output = Self::Output>;

    fn select_n(self) -> Self::Future {
        let (a, b, c, d) = self;
        async {
            match future::select(a, future::select(b, future::select(c, d))).await {
                Either::Left((a, _)) => E4::N1(a),
                Either::Right((Either::Left((b, _)), _)) => E4::N2(b),
                Either::Right((Either::Right((Either::Left((c, _)), _)), _)) => E4::N3(c),
                Either::Right((Either::Right((Either::Right((d, _)), _)), _)) => E4::N4(d),
            }
        }
    }
}

pub async fn select_n<T: SelectN>(t: T) -> T::Output {
    t.select_n().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::{pending, ready, Pending};

    fn nope() -> Pending<()> {
        pending::<()>()
    }

    #[async_std::test]
    async fn test_simple_3() {
        assert_eq!(E3::N1(1usize), select_n((ready(1), nope(), nope())).await);
        assert_eq!(E3::N2(2usize), select_n((nope(), ready(2), nope())).await);
        assert_eq!(E3::N3(3usize), select_n((nope(), nope(), ready(3))).await);
        // If all the futures are ready we should simply get the first one.
        assert_eq!(
            E3::N1(1usize),
            select_n((ready(1), ready(2), ready(3))).await
        );
    }

    #[async_std::test]
    async fn test_simple_4() {
        assert_eq!(
            E4::N1(1usize),
            select_n((ready(1), nope(), nope(), nope())).await
        );
        assert_eq!(
            E4::N2(2usize),
            select_n((nope(), ready(2), nope(), nope())).await
        );
        assert_eq!(
            E4::N3(3usize),
            select_n((nope(), nope(), ready(3), nope())).await
        );
        assert_eq!(
            E4::N4(4usize),
            select_n((nope(), nope(), nope(), ready(4))).await
        );
    }
}
