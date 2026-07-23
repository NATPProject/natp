use super::Claim;
use crate::{Domain, Prop, Scope, SymbolicDomain, Theorem};
use std::marker::PhantomData;

/// Quantifier types in declaration order, together with their common scope.
#[derive(Clone, Debug)]
#[must_use = "attach a claim before invoking a backend"]
pub struct Quantified<Q> {
    name: String,
    variables: Vec<Domain>,
    scope: Scope,
    marker: PhantomData<Q>,
}

#[doc(hidden)]
pub trait Push<D> {
    type Output;
}

impl<Q> Quantified<Q> {
    pub(super) fn first(name: String, domain: Domain) -> Self {
        Self {
            name,
            variables: vec![domain],
            scope: Scope::Unbounded,
            marker: PhantomData,
        }
    }

    /// Append a universal quantifier, preserving the scope selected so far.
    pub fn forall<D: SymbolicDomain>(mut self) -> Quantified<<Q as Push<D>>::Output>
    where
        Q: Push<D>,
    {
        self.variables.push(D::DOMAIN);

        Quantified {
            name: self.name,
            variables: self.variables,
            scope: self.scope,
            marker: PhantomData,
        }
    }

    /// Explicitly quantify over all finite sequence lengths.
    pub fn any_length(mut self) -> Self {
        self.scope = Scope::Unbounded;

        self
    }

    /// Set the inclusive maximum input length for every sequence quantifier, including later ones.
    pub fn max_length(mut self, max_len: usize) -> Self {
        self.scope = Scope::Bounded { max_len };

        self
    }

    fn finish(self, claim: Prop) -> Claim {
        Claim::new(Theorem {
            schema_version: natp_core::limits::SCHEMA_VERSION,
            name: self.name,
            variables: self.variables,
            scope: self.scope,
            claim,
        })
    }
}

macro_rules! quantifiers {
    ($($ty:ident: $id:literal),+) => {
        impl<$($ty: SymbolicDomain),+> Quantified<($($ty,)+)> {
            /// Build the proposition once from distinct typed symbolic variables.
            pub fn claim(self, build: impl FnOnce($($ty),+) -> Prop) -> Claim {
                self.finish(build($($ty::variable($id)),+))
            }
        }

        impl<$($ty,)+ Next> Push<Next> for ($($ty,)+) {
            type Output = ($($ty,)+ Next,);
        }
    };
}

quantifiers!(A: 0);
quantifiers!(A: 0, B: 1);
quantifiers!(A: 0, B: 1, C: 2);
quantifiers!(A: 0, B: 1, C: 2, D: 3);
quantifiers!(A: 0, B: 1, C: 2, D: 3, E: 4);
quantifiers!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
quantifiers!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
quantifiers!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
