mod determinant;
mod enumerations;
mod expr;
mod ext;
mod factorization;
mod fraction;
mod galoisfields;
mod generic;
mod hom;
mod ideal;
mod intfactorization;
mod linearalgebra;
mod multivarpolynom;
mod numbertheoretical;
mod powerseries;
mod promoteconvert;
mod qq;
mod quotient;
mod rationalcanonical;
mod resultant;
mod ring;
mod ringtypes;
mod specialseries;
mod typevars;
mod univarpolynom;
mod zz;
mod zzmod;

pub mod prelude {
    pub use crate::{
        determinant::*,
        enumerations::*,
        expr::*,
        ext::*,
        factorization::*,
        fraction::*,
        galoisfields::*,
        generic::*,
        hom::*,
        ideal::*,
        intfactorization::*,
        linearalgebra::*,
        multivarpolynom::*,
        numbertheoretical::*,
        powerseries::*,
        promoteconvert::*,
        qq::*,
        quotient::*,
        rationalcanonical::*,
        resultant::*,
        ring::*,
        ringtypes::*,
        specialseries::*,
        typevars::*,
        univarpolynom::*,
        zz::*,
        zzmod::*,
    };
}

pub trait AbstractMatrix:
    std::ops::Index<(usize, usize)>
{
    //
}
