//! SMT expression translation.
use crate::vocabulary::{ListSymbol, Operator, RuntimePolicy};
use natp_core::{ElemExpr, Element, Error, Expr, NatExpr, Prop, Theorem, Value};
pub(super) struct Translator<'a> {
    theorem: &'a Theorem,
}
impl<'a> Translator<'a> {
    pub(super) fn new(theorem: &'a Theorem) -> Self {
        Self { theorem }
    }
    fn element_sort(&self, expression: &Expr) -> Element {
        match expression {
            Expr::Empty(element) => *element,
            Expr::Var(id) => self.theorem.variables[*id].element(),
            Expr::Reverse(value) => self.element_sort(value),
            Expr::Append(left, _) => self.element_sort(left),
        }
    }
    fn sequence(&self, expression: &Expr, bindings: &[String]) -> Result<String, Error> {
        let element = self.element_sort(expression);
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
            Expr::Reverse(value) => {
                ListSymbol::Reverse.apply(element, &[self.sequence(value, bindings)?])
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
                    "element expression is not supported by this SMT translator",
                ));
            }
        })
    }
    fn natural(&self, expression: &NatExpr, bindings: &[String]) -> Result<String, Error> {
        Ok(match expression {
            NatExpr::Constant(value) => value.to_string(),
            NatExpr::Length(sequence) => ListSymbol::Length.apply(
                self.element_sort(sequence),
                &[self.sequence(sequence, bindings)?],
            ),
            NatExpr::Add(left, right) => Operator::Add.apply(&[
                self.natural(left, bindings)?,
                self.natural(right, bindings)?,
            ]),
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
            Prop::NatEq(left, right) => Operator::Equal.apply(&[
                self.natural(left, bindings)?,
                self.natural(right, bindings)?,
            ]),
            Prop::IsEmpty(sequence) => ListSymbol::is_empty(
                self.element_sort(sequence),
                &self.sequence(sequence, bindings)?,
            ),
            _ => {
                return Err(Error::new(
                    "proposition is not supported by this SMT translator",
                ));
            }
        })
    }
}
