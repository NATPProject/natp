use crate::{
    Assignment, Domain, ElemExpr, Error, ErrorKind, Expr, NatExpr, Prop, Scope, Theorem, Value,
};
use crate::{Code, DiagnosticField};

struct Evaluator<'left> {
    values: &'left [Assignment],
    fuel: &'left mut usize,
}

impl Evaluator<'_> {
    fn spend_fuel(&mut self, cost: usize) -> Result<(), Error> {
        *self.fuel = self.fuel.checked_sub(cost).ok_or_else(|| {
            Error::with_kind(
                ErrorKind::ResourceLimit,
                format!(
                    "evaluation needs {cost} work units but only {} remain",
                    self.fuel,
                ),
            )
            .with_code(Code::EvaluationBudget)
            .with_help(
                "increase the evaluation fuel or simplify the claim; no logical conclusion was established",
            )
        })?;

        Ok(())
    }

    fn evaluate_sequence(&mut self, expression: &Expr) -> Result<Vec<Value>, Error> {
        self.spend_fuel(1)?;

        match expression {
            Expr::Empty(_) => Ok(vec![]),
            Expr::Var(id) => match self.values.get(*id) {
                Some(Assignment::Sequence(sequence_values)) => {
                    self.spend_fuel(sequence_values.len())?;

                    Ok(sequence_values.clone())
                }
                _ => Err(Error::new("expected sequence assignment")),
            },
            Expr::Reverse(sequence) => {
                let mut sequence = self.evaluate_sequence(sequence)?;
                sequence.reverse();

                Ok(sequence)
            }
            Expr::Append(left, right) => {
                let mut left = self.evaluate_sequence(left)?;
                let right = self.evaluate_sequence(right)?;
                self.spend_fuel(right.len())?;
                left.extend(right);

                Ok(left)
            }
        }
    }

    fn evaluate_element(
        &self,
        expression: &ElemExpr,
        bound_element: Option<&Value>,
    ) -> Result<Value, Error> {
        match expression {
            ElemExpr::Constant(value) => Ok(value.clone()),
            ElemExpr::Bound => bound_element
                .cloned()
                .ok_or_else(|| Error::new("unbound element")),
            ElemExpr::Var(id) => match self.values.get(*id) {
                Some(Assignment::Scalar(value)) => Ok(value.clone()),
                _ => Err(Error::new("expected scalar assignment")),
            },
        }
    }

    fn evaluate_natural(&mut self, expression: &NatExpr) -> Result<u128, Error> {
        self.spend_fuel(1)?;

        match expression {
            NatExpr::Constant(n) => Ok(u128::from(*n)),
            NatExpr::Length(sequence) => Ok(self.evaluate_sequence(sequence)?.len() as u128),
            NatExpr::Add(left, right) => self
                .evaluate_natural(left)?
                .checked_add(self.evaluate_natural(right)?)
                .ok_or_else(|| Error::new("natural evaluation overflow")),
        }
    }

    fn evaluate_proposition(
        &mut self,
        proposition: &Prop,
        bound_element: Option<&Value>,
    ) -> Result<bool, Error> {
        self.spend_fuel(1)?;

        Ok(match proposition {
            Prop::Bool(value) => *value,
            Prop::SeqEq(left, right) => {
                self.evaluate_sequence(left)? == self.evaluate_sequence(right)?
            }
            Prop::ElemEq(left, right) => {
                self.evaluate_element(left, bound_element)?
                    == self.evaluate_element(right, bound_element)?
            }
            Prop::NatEq(left, right) => {
                self.evaluate_natural(left)? == self.evaluate_natural(right)?
            }
            Prop::Not(predicate) => !self.evaluate_proposition(predicate, bound_element)?,
            Prop::And(left, right) => {
                self.evaluate_proposition(left, bound_element)?
                    && self.evaluate_proposition(right, bound_element)?
            }
            Prop::Or(left, right) => {
                self.evaluate_proposition(left, bound_element)?
                    || self.evaluate_proposition(right, bound_element)?
            }
            Prop::Implies(left, right) => {
                !self.evaluate_proposition(left, bound_element)?
                    || self.evaluate_proposition(right, bound_element)?
            }
            _ => {
                return Err(
                    Error::new("unsupported proposition").with_code(Code::UnsupportedExpression)
                );
            }
        })
    }
}

impl Theorem {
    /// Concrete semantics. Fuel exhaustion is an error, never a logical result.
    ///
    /// # Errors
    /// Returns errors for malformed IR, incorrect assignments, exhausted fuel,
    /// or an arithmetic failure. Input lengths must respect the theorem's scope.
    pub fn evaluate(&self, values: &[Assignment], fuel: &mut usize) -> Result<bool, Error> {
        self.validate()?;

        if values.len() != self.variables.len() {
            return Err(Error::new(format!(
                "expected {} assignments in quantifier order, found {}",
                self.variables.len(),
                values.len()
            ))
            .with_code(Code::AssignmentCount)
            .at(DiagnosticField::Assignments.as_str())
            .with_help("provide exactly one assignment for each entry in theorem.variables"));
        }

        for (index, (domain, value)) in self.variables.iter().zip(values).enumerate() {
            let invalid = |message: String, location: String| {
                Error::new(message)
                    .with_code(Code::AssignmentSort)
                    .at(location)
                    .with_help(format!(
                        "assignment {index} must match variables[{index}], declared as {domain}"
                    ))
            };

            match (domain, value) {
                (Domain::Scalar(element), Assignment::Scalar(value)) => {
                    if *element != value.element() {
                        return Err(invalid(
                            format!("expected scalar {element}, found {}", value.element()),
                            DiagnosticField::Assignments.index(index),
                        ));
                    }
                }
                (Domain::Sequence(element), Assignment::Sequence(sequence_values)) => {
                    if let Some((cell, value)) = sequence_values
                        .iter()
                        .enumerate()
                        .find(|(_, value)| *element != value.element())
                    {
                        return Err(invalid(
                            format!(
                                "expected sequence element {element}, found {}",
                                value.element()
                            ),
                            format!(
                                "{}.{}",
                                DiagnosticField::Assignments.index(index),
                                DiagnosticField::Value.index(cell)
                            ),
                        ));
                    }

                    if let Scope::Bounded { max_len } = self.scope {
                        if sequence_values.len() > max_len {
                            return Err(Error::new(format!(
                                "sequence length {} exceeds theorem bound {max_len}",
                                sequence_values.len()
                            ))
                            .with_code(Code::AssignmentBound)
                            .at(DiagnosticField::Assignments.index(index))
                            .with_help(
                                "replay an assignment within the theorem's declared scope",
                            ));
                        }
                    }
                }
                (Domain::Scalar(_), Assignment::Sequence(_)) => {
                    return Err(invalid(
                        "expected a scalar assignment, found a sequence".into(),
                        DiagnosticField::Assignments.index(index),
                    ));
                }
                (Domain::Sequence(_), Assignment::Scalar(_)) => {
                    return Err(invalid(
                        "expected a sequence assignment, found a scalar".into(),
                        DiagnosticField::Assignments.index(index),
                    ));
                }
            }
        }

        Evaluator { values, fuel }.evaluate_proposition(&self.claim, None)
    }
}
