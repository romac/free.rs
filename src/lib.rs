#![allow(incomplete_features, unused_variables)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![feature(box_patterns)]

pub mod free;
pub mod maybe;

pub trait Family: Copy {
    type This<T>;
}

pub trait Functor: Family {
    fn fmap<A, B, F>(self, this: Self::This<A>, f: F) -> Self::This<B>
    where
        F: Fn(A) -> B;
}

pub fn fmap<M, T, U, F>(m: M, x: M::This<T>, f: F) -> M::This<U>
where
    M: Monad,
    F: Fn(T) -> U,
{
    m.fmap(x, f)
}

pub trait Applicative: Functor {
    fn pure<A>(self, a: A) -> Self::This<A>;

    fn zip2<A, B>(self, this: Self::This<A>, other: Self::This<B>) -> Self::This<(A, B)>;
}

pub trait Monad: Applicative {
    fn bind<A, B, F>(self, this: Self::This<A>, f: F) -> Self::This<B>
    where
        F: Fn(A) -> Self::This<B>;
}

pub fn bind<M, T, U, F>(m: M, x: M::This<T>, f: F) -> M::This<U>
where
    M: Monad,
    F: Fn(T) -> M::This<U>,
{
    m.bind(x, f)
}

pub trait Monoid {
    fn empty() -> Self;
    fn append(self, other: Self) -> Self;
}

// pub trait Foldable: Family {
//     fn fold_map<A, M, F>(_: Self::This<A>, f: F) -> M
//     where
//         M: Monoid,
//         F: Fn(A) -> M;
// }

// pub trait Traversable: Foldable {
//     fn traverse<A, B, F, G>(_: Self::This<A>, f: G) -> F::This<Self::This<B>>
//     where
//         F: Applicative,
//         G: Fn(A) -> F::This<B>;
// }
