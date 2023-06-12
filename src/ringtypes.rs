pub trait RingClass {
    //
}

use std::ops::{
    Add,
    Mul,
    Sub,
};

pub trait Inv {
    type Output;
    fn inv(&self) -> Self::Output;
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

pub trait Ring:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Inv<Output = Self>
{
    //
}

// abstract type RingClass {}
pub trait FractionRingClass: RingClass {
    //
}
// trait QuotientRingClass: RingClass {
//
// }
// trait PolyRingClass <: RingClass {}
// trait Ring{T<:RingClass} {}
// trait FractionRing{S<:RingInt,T<:FractionRingClass} <: Ring{T} {}
// trait QuotientRing{S<:RingInt,T<:QuotientRingClass} <: Ring{T} {}
// trait Polynomial{S<:Ring,T<:PolyRingClass} <: Ring{T} {}

// pub trait Polynomial<R: Ring>: Ring {
//     //
// }
// trait RingTrait {}
// trait CommutativeRingTrait <: RingTrait {}
// trait IntegralDomainTrait <: CommutativeRingTrait {}
// trait IntegrallyClosedDomainTrait <: IntegralDomainTrait {}
// trait GCDDomainTrait <: IntegrallyClosedDomainTrait {}
// trait UniqueFactorizationDomainTrait <: GCDDomainTrait {}
// trait PrincipalIdealDomainTrait <: UniqueFactorizationDomainTrait {}
// trait EuclidianDomainTrait <: PrincipalIdealDomainTrait {}
// trait FieldTrait <: EuclidianDomainTrait {}
// trait AlgebraicallyClosedFieldTrait <: FieldTrait {}

// Integers
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ZZ(i32);

impl ZZ {
    pub fn mod_(i: i32) -> ZZMod {
        unimplemented!()
    }
}

// impl std::ops::Add for ZZ {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Self(self.0 + rhs.0)
//     }
// }

// impl std::ops::Div<usize> for ZZ {
//     type Output = ZZMod;
//     fn div(rhs: usize) -> Self::Output {
//         unimplemented!()
//     }
// }
#[derive(Clone, Copy, PartialEq, Eq)]

pub struct ZZMod {
    //
    val: i32,
    mod_: i32,
}

// pub struct Frac<

// R = ZZ[x,y]; // polynoimal ring
// S = ZZ[a,b,c]; // polynomial ring
// f = map(R,S,{x^2,x*y,y^2})
//  f(a+b+c^2) =>
pub struct Hom<R: Ring, S: Ring, F: Fn(R) -> S> {
    //
    f: F,
    ph: std::marker::PhantomData<(S, R, F)>,
}

impl<R: Ring, S: Ring, F: Fn(R) -> S> Hom<R, S, F> {
    // fn new()

    pub fn domain(&self) -> R {
        unimplemented!()
    }

    pub fn codomain(&self) -> S {
        unimplemented!()
    }

    pub fn eval(&self, v: R) -> S {
        (self.f)(v)
    }
}

mod tests {
    //
}
