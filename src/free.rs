use std::fmt::Debug;
use std::marker::PhantomData;

use crate::*;

pub enum Free<F: Functor, A> {
    Free(Box<F::This<Free<F, A>>>),
    Pure(A),
}

#[derive(Copy, Clone, Default)]
pub struct FreeFamily<F>(F);

impl<F> FreeFamily<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F: Functor> Family for FreeFamily<F> {
    type This<T> = Free<F, T>;
}

impl<F: Functor> Functor for FreeFamily<F> {
    fn fmap<A, B, Fun>(self, this: Self::This<A>, f: Fun) -> Self::This<B>
    where
        Fun: Fn(A) -> B,
    {
        match this {
            Free::Pure(a) => Free::Pure(f(a)),
            Free::Free(box fa) => Free::Free(Box::new(self.0.fmap(fa, |x| self.fmap(x, |a| f(a))))),
        }
    }
}

impl<F: Functor> Applicative for FreeFamily<F> {
    fn pure<A>(self, a: A) -> Self::This<A> {
        Free::Pure(a)
    }

    fn zip2<A, B>(self, this: Self::This<A>, other: Self::This<B>) -> Self::This<(A, B)> {
        todo!()
    }
}

impl<F: Functor> Monad for FreeFamily<F> {
    fn bind<A, B, Fun>(self, this: Self::This<A>, f: Fun) -> Self::This<B>
    where
        Fun: Fn(A) -> Self::This<B>,
    {
        match this {
            Free::Pure(a) => f(a),
            Free::Free(box fa) => Free::Free(Box::new(self.0.fmap(fa, |x| self.bind(x, |a| f(a))))),
        }
    }
}

pub enum Toy<A, Next> {
    Done,
    Bell(Next),
    Output(A, Next),
}

pub struct ToyFamily<A>(PhantomData<A>);

impl<A> Copy for ToyFamily<A> {}
impl<A> Clone for ToyFamily<A> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<A> Family for ToyFamily<A> {
    type This<T> = Toy<A, T>;
}

impl<A0> Functor for ToyFamily<A0> {
    fn fmap<A, B, Fun>(self, this: Self::This<A>, f: Fun) -> Self::This<B>
    where
        Fun: Fn(A) -> B,
    {
        match this {
            Toy::Done => Toy::Done,
            Toy::Bell(next) => Toy::Bell(f(next)),
            Toy::Output(a, next) => Toy::Output(a, f(next)),
        }
    }
}

pub fn lift_f<R, F: Functor>(f: F, x: F::This<R>) -> Free<F, R> {
    Free::Free(Box::new(f.fmap(x, Free::Pure)))
}

pub fn output<A>(a: A) -> Free<ToyFamily<A>, ()> {
    lift_f(ToyFamily(PhantomData), Toy::Output(a, ()))
}

pub fn bell<A>() -> Free<ToyFamily<A>, ()> {
    lift_f(ToyFamily(PhantomData), Toy::Bell(()))
}

pub fn done<A, R>() -> Free<ToyFamily<A>, R> {
    lift_f(ToyFamily(PhantomData), Toy::Done)
}

pub fn subroutine() -> Free<ToyFamily<char>, ()> {
    output('A')
}

pub fn program<R>() -> Free<ToyFamily<char>, R> {
    let fam: FreeFamily<ToyFamily<char>> = FreeFamily(ToyFamily(PhantomData));
    fam.bind(subroutine(), |_| fam.bind(bell(), |_| done()))
}

pub fn pretty<A, R>(fa: Free<ToyFamily<A>, R>) -> String
where
    A: Debug,
    R: Debug,
{
    match fa {
        Free::Pure(r) => format!("return {:?}\n", r),
        Free::Free(box Toy::Done) => "done\n".to_string(),
        Free::Free(box Toy::Bell(x)) => format!("bell\n{:?}", pretty(x)),
        Free::Free(box Toy::Output(a, x)) => format!("output {:?}\n{}", a, pretty(x)),
    }
}

pub fn test() {
    let prog = program::<()>();
    println!("{}", pretty(prog));
}
