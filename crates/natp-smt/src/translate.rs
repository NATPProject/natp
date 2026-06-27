//! SMT expression translation.
use crate::vocabulary::Operator;
use natp_core::{ElemExpr, Error, Prop, Theorem, Value};
pub(super) struct Translator;
impl Translator {
    pub(super) fn new(_theorem: &Theorem) -> Self {
        Self
    }
    fn element(&self, expression: &ElemExpr, bindings: &[String]) -> Result<String, Error> {
        Ok(match expression {
            ElemExpr::Var(id) => bindings[*id].clone(),
            ElemExpr::Constant(Value::Bool(value)) => value.to_string(),
            _ => {
                return Err(Error::new(
                    "element expression is not supported by this SMT translator",
                ));
            }
        })
    }
    pub(super) fn translate_proposition(
        &self,
        proposition: &Prop,
        bindings: &[String],
    ) -> Result<String, Error> {
        Ok(match proposition {
            Prop::Bool(value) => value.to_string(),
            Prop::ElemEq(left, right) => Operator::Equal.apply(&[
                self.element(left, bindings)?,
                self.element(right, bindings)?,
            ]),
            _ => {
                return Err(Error::new(
                    "proposition is not supported by this SMT translator",
                ));
            }
        })
    }
}
