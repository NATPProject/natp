//! Translate supported theorem expressions into SMT-LIB.
use crate::vocabulary::{ListSymbol, Operator, RuntimePolicy};
use natp_core::{ElemExpr, Element, Error, Expr, Prop, Theorem, Value};

pub(super) struct Translator<'a> {
    theorem: &'a Theorem,
}
impl<'a> Translator<'a> {
    pub(super) fn new(theorem: &'a Theorem) -> Self {
        Self { theorem }
    }
    fn sequence_element_sort(&self, expression: &Expr) -> Element {
        match expression {
            Expr::Empty(element) => *element,
            Expr::Var(id) => self.theorem.variables[*id].element(),
            Expr::Reverse(sequence) => self.sequence_element_sort(sequence),
            Expr::Append(left, _) => self.sequence_element_sort(left),
        }
    }
    fn sequence(&self, expression: &Expr, bindings: &[String]) -> Result<String, Error> {
        let element = self.sequence_element_sort(expression);
        Ok(match expression {
            Expr::Empty(_) => ListSymbol::Empty.render(element),
            Expr::Var(id) => bindings[*id].clone(),
            Expr::Append(left, right) => ListSymbol::Append.apply(
                element,
                &[
                    self.sequence(left, bindings)?,
                    self.sequence(right, bindings)?,
                ],
            ),
            _ => {
                return Err(Error::new(
                    "sequence expression is not supported by this SMT translator",
                ));
            }
        })
    }
    fn element(&self, expression: &ElemExpr, bindings: &[String]) -> Result<String, Error> {
        Ok(match expression {
            ElemExpr::Var(id) => bindings[*id].clone(),
            ElemExpr::Constant(Value::Bool(value)) => value.to_string(),
            ElemExpr::Constant(Value::I32(value)) => format!(
                "(_ bv{} {})",
                *value as u32,
                RuntimePolicy::ElementBits.number()
            ),

            _ => {
                return Err(Error::new(
                    "bound elements are not supported by this SMT translator",
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
            Prop::SeqEq(left, right) => Operator::Equal.apply(&[
                self.sequence(left, bindings)?,
                self.sequence(right, bindings)?,
            ]),
            _ => {
                return Err(Error::new(
                    "proposition is not supported by this SMT translator",
                ));
            }
        })
    }
}
