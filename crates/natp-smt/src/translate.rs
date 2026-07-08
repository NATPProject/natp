//! Translate validated theorem expressions using quantified input bindings.

use natp_core::{ElemExpr, Element, Expr, NatExpr, Prop, Theorem, Value};

use crate::vocabulary::{Binding, ListSymbol, Operator, RuntimePolicy, Sort};

pub(super) struct Translator<'left> {
    theorem: &'left Theorem,
}

impl<'theorem> Translator<'theorem> {
    /// Requires a theorem already checked by Theorem::validate.
    pub(super) fn new(theorem: &'theorem Theorem) -> Self {
        Self { theorem }
    }

    fn sequence_element_sort(&self, expression: &Expr) -> Element {
        match expression {
            Expr::Empty(element) => *element,
            Expr::Var(variable_index) => self.theorem.variables[*variable_index].element(),
            Expr::Reverse(expression) => self.sequence_element_sort(expression),
            Expr::Append(expression, _) => self.sequence_element_sort(expression),
        }
    }

    fn translate_sequence(&self, expression: &Expr, bindings: &[String]) -> String {
        let element = self.sequence_element_sort(expression);

        match expression {
            Expr::Empty(_) => ListSymbol::Empty.render(element),
            Expr::Var(id) => bindings[*id].clone(),
            Expr::Reverse(sequence) => {
                ListSymbol::Reverse.apply(element, &[self.translate_sequence(sequence, bindings)])
            }
            Expr::Append(left, right) => ListSymbol::Append.apply(
                element,
                &[
                    self.translate_sequence(left, bindings),
                    self.translate_sequence(right, bindings),
                ],
            ),
        }
    }

    fn translate_element(&self, expression: &ElemExpr, bindings: &[String]) -> String {
        match expression {
            ElemExpr::Var(id) => bindings[*id].clone(),
            ElemExpr::Bound => Binding::BoundElement.render(),
            ElemExpr::Constant(Value::I32(value)) => format!(
                "(_ bv{} {})",
                *value as u32,
                RuntimePolicy::ElementBits.number()
            ),
            ElemExpr::Constant(Value::Bool(value)) => value.to_string(),
        }
    }

    fn translate_natural(&self, expression: &NatExpr, bindings: &[String]) -> String {
        match expression {
            NatExpr::Constant(value) => value.to_string(),
            NatExpr::Length(sequence) => ListSymbol::Length.apply(
                self.sequence_element_sort(sequence),
                &[self.translate_sequence(sequence, bindings)],
            ),
            NatExpr::Add(left, right) => Operator::Add.apply(&[
                self.translate_natural(left, bindings),
                self.translate_natural(right, bindings),
            ]),
        }
    }

    pub(super) fn translate_proposition(&self, proposition: &Prop, bindings: &[String]) -> String {
        match proposition {
            Prop::Bool(value) => value.to_string(),
            Prop::SeqEq(left, right) => Operator::Equal.apply(&[
                self.translate_sequence(left, bindings),
                self.translate_sequence(right, bindings),
            ]),
            Prop::ElemEq(left, right) => Operator::Equal.apply(&[
                self.translate_element(left, bindings),
                self.translate_element(right, bindings),
            ]),
            Prop::NatEq(left, right) => Operator::Equal.apply(&[
                self.translate_natural(left, bindings),
                self.translate_natural(right, bindings),
            ]),
            Prop::IsEmpty(sequence) => ListSymbol::is_empty(
                self.sequence_element_sort(sequence),
                &self.translate_sequence(sequence, bindings),
            ),
            Prop::Contains(sequence, element) => ListSymbol::Membership.apply(
                self.sequence_element_sort(sequence),
                &[
                    self.translate_sequence(sequence, bindings),
                    self.translate_element(element, bindings),
                ],
            ),
            Prop::All(sequence, predicate) => {
                let element = self.sequence_element_sort(sequence);
                let bound = Binding::BoundElement.render();
                let membership = ListSymbol::Membership.apply(
                    element,
                    &[self.translate_sequence(sequence, bindings), bound.clone()],
                );
                let implication = Operator::Implies
                    .apply(&[membership, self.translate_proposition(predicate, bindings)]);

                format!(
                    "({} (({bound} {})) {implication})",
                    Operator::ForAll.token(),
                    Sort::element(element).render()
                )
            }
            Prop::Not(predicate) => {
                Operator::Not.apply(&[self.translate_proposition(predicate, bindings)])
            }
            Prop::And(left, right) | Prop::Or(left, right) | Prop::Implies(left, right) => {
                let operator = match proposition {
                    Prop::And(..) => Operator::And,
                    Prop::Or(..) => Operator::Or,
                    _ => Operator::Implies,
                };

                operator.apply(&[
                    self.translate_proposition(left, bindings),
                    self.translate_proposition(right, bindings),
                ])
            }
        }
    }
}
