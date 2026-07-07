//! Translate supported theorem expressions into SMT-LIB.
use crate::vocabulary::{Binding, ListSymbol, Operator, RuntimePolicy, Sort};
use natp_core::{ElemExpr, Element, Error, Expr, NatExpr, Prop, Theorem, Value};

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
            Expr::Reverse(sequence) => {
                ListSymbol::Reverse.apply(element, &[self.sequence(sequence, bindings)?])
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
            ElemExpr::Bound => Binding::BoundElement.render(),
        })
    }
    fn natural(&self, expression: &NatExpr, bindings: &[String]) -> Result<String, Error> {
        Ok(match expression {
            NatExpr::Constant(value) => value.to_string(),
            NatExpr::Length(sequence) => ListSymbol::Length.apply(
                self.sequence_element_sort(sequence),
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
                self.sequence_element_sort(sequence),
                &self.sequence(sequence, bindings)?,
            ),
            Prop::Contains(sequence, element) => ListSymbol::Membership.apply(
                self.sequence_element_sort(sequence),
                &[
                    self.sequence(sequence, bindings)?,
                    self.element(element, bindings)?,
                ],
            ),
            Prop::All(sequence, predicate) => {
                let element = self.sequence_element_sort(sequence);
                let bound = Binding::BoundElement.render();
                let member = ListSymbol::Membership.apply(
                    element,
                    &[self.sequence(sequence, bindings)?, bound.clone()],
                );
                let implication = Operator::Implies
                    .apply(&[member, self.translate_proposition(predicate, bindings)?]);
                format!(
                    "({} (({bound} {})) {implication})",
                    Operator::ForAll.token(),
                    Sort::element(element).render()
                )
            }
            Prop::Not(predicate) => {
                Operator::Not.apply(&[self.translate_proposition(predicate, bindings)?])
            }
            Prop::And(left, right) | Prop::Or(left, right) | Prop::Implies(left, right) => {
                let operator = match proposition {
                    Prop::And(..) => Operator::And,
                    Prop::Or(..) => Operator::Or,
                    _ => Operator::Implies,
                };
                operator.apply(&[
                    self.translate_proposition(left, bindings)?,
                    self.translate_proposition(right, bindings)?,
                ])
            }
        })
    }
}
