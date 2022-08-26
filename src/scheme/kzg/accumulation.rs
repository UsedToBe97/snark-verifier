use crate::{
    loader::Loader,
    protocol::Protocol,
    scheme::kzg::accumulator::Accumulator,
    util::{Curve, Transcript},
    Error,
};

pub mod plonk;
pub mod shplonk;

pub trait AccumulationScheme<C, L, T, S>
where
    C: Curve,
    L: Loader<C>,
    T: Transcript<C, L>,
    S: AccumulationStrategy<C, L, T, Self::Proof>,
{
    type Proof;

    fn accumulate(
        protocol: &Protocol<C>,
        loader: &L,
        instances: Vec<Vec<L::LoadedScalar>>,
        transcript: &mut T,
        strategy: &mut S,
    ) -> Result<S::Output, Error>;
}

pub trait AccumulationStrategy<C, L, T, P>
where
    C: Curve,
    L: Loader<C>,
    T: Transcript<C, L>,
{
    type Output;

    fn extract_accumulator(
        &self,
        _: &Protocol<C>,
        _: &L,
        _: &mut T,
        _: &[Vec<L::LoadedScalar>],
    ) -> Option<Accumulator<C, L>> {
        None
    }

    fn process(
        &mut self,
        loader: &L,
        transcript: &mut T,
        proof: P,
        accumulator: Accumulator<C, L>,
    ) -> Result<Self::Output, Error>;
}

/// Accumulation strategy that does accumulation on the same curve.
///
/// Since in circuit everything are in scalar field, but while doing elliptic
/// curve operation we need to operate in base field, so we need to emulate
/// base field in scalar field.
/// The const generic `LIMBS` and `BITS` respectively represents how many limbs
/// a base field element are split into and how many bits each limbs could has.
pub struct SameCurveAccumulation<C: Curve, L: Loader<C>, const LIMBS: usize, const BITS: usize> {
    pub accumulator: Option<Accumulator<C, L>>,
}

impl<C: Curve, L: Loader<C>, const LIMBS: usize, const BITS: usize> Default
    for SameCurveAccumulation<C, L, LIMBS, BITS>
{
    fn default() -> Self {
        Self { accumulator: None }
    }
}
