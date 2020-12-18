use crate::*;

#[derive(Copy, Clone)]
pub struct Maybe;

impl Family for Maybe {
    type This<T> = Option<T>;
}

impl Functor for Maybe {
    fn fmap<A, B, F>(self, this: Self::This<A>, f: F) -> Self::This<B>
    where
        F: Fn(A) -> B,
    {
        this.map(f)
    }
}

impl Applicative for Maybe {
    fn pure<A>(self, a: A) -> Self::This<A> {
        Some(a)
    }

    fn zip2<A, B>(self, this: Self::This<A>, that: Self::This<B>) -> Self::This<(A, B)> {
        this.zip(that)
    }
}

impl Monad for Maybe {
    fn bind<T, U, F>(self, this: Self::This<T>, f: F) -> Self::This<U>
    where
        F: Fn(T) -> Self::This<U>,
    {
        this.and_then(f)
    }
}

// impl Foldable for Maybe {
//     fn fold_map<A, M, F>(opt: Self::This<A>, f: F) -> M
//     where
//         M: Monoid,
//         F: Fn(A) -> M,
//     {
//         opt.map(f).unwrap_or_else(M::empty)
//     }
// }

// impl Traversable for Maybe {
//     fn traverse<A, B, F, G>(opt: Self::This<A>, f: G) -> F::This<Self::This<B>>
//     where
//         F: Applicative,
//         G: Fn(A) -> F::This<B>,
//     {
//         match opt {
//             None => Maybe.pure(None),
//             Some(a) => Maybe.fmap(f(a), Some),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = bind(Maybe, Some(1), |x| Some(x * 2));
        println!("{:?}", x);
    }
}
