//! Validate theorem structure and report the field containing an error.

use crate::{Code, DiagnosticField, DomainKind, catalogue::SyntaxEdge};
use crate::{Domain, ElemExpr, Element, Error, ErrorKind, Expr, NatExpr, Prop, Scope, Theorem, limits};

impl Theorem {
    /// Check schema, quantifiers, scope, sorts, binders, and syntax budgets.
    ///
    /// # Errors
    /// Returns a coded diagnostic with the failing IR path and a corrective hint.
    /// This checks representation validity, not whether the proposition is true.
    pub fn validate(&self) -> Result<(), Error> {
        let header_error = |code, path, message: String, help| {
            Error::new(message).with_code(code).at(path).with_help(help)
        };

        if self.schema_version != limits::SCHEMA_VERSION {
            return Err(header_error(
                Code::SchemaVersion,
                DiagnosticField::SchemaVersion.as_str().to_owned(),
                format!(
                    "unsupported schema version {}; expected {}",
                    self.schema_version,
                    limits::SCHEMA_VERSION,
                ),
                "regenerate the theorem with Theorem::save; do not relabel an older schema without \
                 migrating its contents",
            ));
        }

        if self.name.is_empty() || self.name.len() > limits::MAX_NAME_BYTES {
            return Err(header_error(
                Code::TheoremName,
                DiagnosticField::Name.as_str().to_owned(),
                format!(
                    "theorem name has {} UTF-8 bytes; expected 1..={}",
                    self.name.len(),
                    limits::MAX_NAME_BYTES
                ),
                "provide a nonempty name within the byte limit",
            ));
        }

        if self.variables.is_empty() || self.variables.len() > limits::MAX_VARIABLES {
            return Err(header_error(
                Code::QuantifierCount,
                DiagnosticField::Variables.as_str().to_owned(),
                format!(
                    "received {} quantified variables; expected 1..={}",
                    self.variables.len(),
                    limits::MAX_VARIABLES,
                ),
                "declare one to eight Seq or Elem quantifiers and keep variable IDs in declaration order",
            ));
        }

        if let Scope::Bounded { max_len } = self.scope {
            if max_len > limits::MAX_BOUNDED_LENGTH {
                return Err(header_error(
                    Code::LengthBound,
                    DiagnosticField::Scope.child(DiagnosticField::MaximumLength),
                    format!(
                        "sequence bound {max_len} exceeds {}",
                        limits::MAX_BOUNDED_LENGTH
                    ),
                    "reduce max_length; use unbounded kernel reasoning only if that is the intended scope",
                ));
            }
        }

        Validator::new(&self.variables, DiagnosticField::Claim)
            .validate_proposition(&self.claim, None, 0)
            .map_err(|error| error.context(format!("theorem {:?}", self.name)))
    }

    /// Infer the element sort of a sequence term in this theorem's environment.
    ///
    /// # Errors
    /// Returns a path-specific diagnostic for invalid IDs, sorts, or syntax limits.
    pub fn sequence_element(&self, expression: &Expr) -> Result<Element, Error> {
        Validator::new(&self.variables, DiagnosticField::Expression)
            .validate_sequence(expression, 0)
    }
}

struct Validator<'left> {
    variables: &'left [Domain],
    nodes: usize,
    root: DiagnosticField,
    path: Vec<SyntaxEdge>,
}

impl<'left> Validator<'left> {
    fn new(variables: &'left [Domain], root: DiagnosticField) -> Self {
        Self {
            variables,
            nodes: 0,
            root,
            path: vec![],
        }
    }

    fn location(&self) -> String {
        let mut path = self.root.as_str().to_owned();

        for segment in &self.path {
            path.push_str(segment.as_str());
        }

        path
    }

    fn error(&self, code: Code, message: impl Into<String>, help: impl Into<String>) -> Error {
        Error::new(message)
            .with_code(code)
            .at(self.location())
            .with_help(help)
    }

    fn at_path<T>(
        &mut self,
        segment: SyntaxEdge,
        check: impl FnOnce(&mut Self) -> Result<T, Error>,
    ) -> Result<T, Error> {
        self.path.push(segment);
        let result = check(self);
        self.path.pop();

        result
    }

    fn check_syntax_budget(&mut self, depth: usize) -> Result<(), Error> {
        self.nodes += 1;
        let excess = if depth > limits::MAX_SYNTAX_DEPTH {
            Some(format!(
                "syntax depth {depth} exceeds {}",
                limits::MAX_SYNTAX_DEPTH
            ))
        } else if self.nodes > limits::MAX_SYNTAX_NODES {
            Some(format!(
                "syntax node count {} exceeds {}",
                self.nodes,
                limits::MAX_SYNTAX_NODES
            ))
        } else {
            None
        };

        if let Some(message) = excess {
            return Err(Error::with_kind(ErrorKind::ResourceLimit, message)
                .with_code(Code::SyntaxBudget)
                .at(self.location())
                .with_help(
                    "simplify the expression or split it into smaller independently checked claims",
                ));
        }

        Ok(())
    }

    fn require_same_element_sort(&self, expected: Element, actual: Element) -> Result<(), Error> {
        if expected == actual {
            Ok(())
        } else {
            Err(self.error(
                Code::ElementSort,
                format!("element sort mismatch: expected {expected}, found {actual}"),
                "use the same element sort on both sides; Seq<i32> and Seq<bool> cannot be mixed",
            ))
        }
    }

    fn variable(&self, id: usize, kind: DomainKind) -> Result<Element, Error> {
        let expected = kind.as_str();

        match self.variables.get(id) {
            Some(Domain::Sequence(element)) if kind == DomainKind::Sequence => Ok(*element),
            Some(Domain::Scalar(element)) if kind == DomainKind::Scalar => Ok(*element),
            Some(domain) => Err(self.error(
                Code::VariableDomain,
                format!("variable {id} has domain {domain}; expected a {expected} variable"),
                format!("use a {expected} quantifier at this position or reference a variable of that domain"),
            )),
            None => Err(self.error(
                Code::UnknownVariable,
                format!(
                    "variable ID {id} is out of range for {} declared quantifiers",
                    self.variables.len(),
                ),
                "variable IDs are zero-based and must refer to an entry in variables",
            )),
        }
    }

    fn validate_sequence(&mut self, expression: &Expr, depth: usize) -> Result<Element, Error> {
        self.check_syntax_budget(depth)?;

        match expression {
            Expr::Empty(element) => Ok(*element),
            Expr::Var(id) => self.variable(*id, DomainKind::Sequence),
            Expr::Reverse(sequence) => self.at_path(SyntaxEdge::Unary, |validator| {
                validator.validate_sequence(sequence, depth + 1)
            }),
            Expr::Append(left, right) => {
                let left = self.at_path(SyntaxEdge::First, |validator| {
                    validator.validate_sequence(left, depth + 1)
                })?;
                self.at_path(SyntaxEdge::Second, |validator| {
                    let right = validator.validate_sequence(right, depth + 1)?;

                    validator.require_same_element_sort(left, right)
                })?;

                Ok(left)
            }
        }
    }

    fn validate_element(
        &mut self,
        expression: &ElemExpr,
        bound_element: Option<Element>,
        depth: usize,
    ) -> Result<Element, Error> {
        self.check_syntax_budget(depth)?;

        match expression {
            ElemExpr::Constant(v) => Ok(v.element()),
            ElemExpr::Bound => bound_element.ok_or_else(|| {
                self.error(
                    Code::UnboundElement,
                    "bound element used outside an all predicate",
                    "use this element only inside its enclosing all closure; use an Elem quantifier for a free scalar",
                )
            }),
            ElemExpr::Var(id) => self.variable(*id, DomainKind::Scalar),
        }
    }

    fn validate_natural(&mut self, expression: &NatExpr, depth: usize) -> Result<(), Error> {
        self.check_syntax_budget(depth)?;

        match expression {
            NatExpr::Constant(_) => Ok(()),
            NatExpr::Length(sequence) => self.at_path(SyntaxEdge::Unary, |validator| {
                validator.validate_sequence(sequence, depth + 1).map(|_| ())
            }),
            NatExpr::Add(left, right) => {
                self.at_path(SyntaxEdge::First, |validator| {
                    validator.validate_natural(left, depth + 1)
                })?;

                self.at_path(SyntaxEdge::Second, |validator| {
                    validator.validate_natural(right, depth + 1)
                })
            }
        }
    }

    fn validate_proposition(
        &mut self,
        proposition: &Prop,
        bound_element: Option<Element>,
        depth: usize,
    ) -> Result<(), Error> {
        self.check_syntax_budget(depth)?;

        match proposition {
            Prop::Bool(_) => Ok(()),
            Prop::SeqEq(left, right) => {
                let left = self.at_path(SyntaxEdge::First, |validator| {
                    validator.validate_sequence(left, depth + 1)
                })?;

                self.at_path(SyntaxEdge::Second, |validator| {
                    let right = validator.validate_sequence(right, depth + 1)?;

                    validator.require_same_element_sort(left, right)
                })
            }
            Prop::ElemEq(left, right) => {
                let left = self.at_path(SyntaxEdge::First, |validator| {
                    validator.validate_element(left, bound_element, depth + 1)
                })?;

                self.at_path(SyntaxEdge::Second, |validator| {
                    let right = validator.validate_element(right, bound_element, depth + 1)?;

                    validator.require_same_element_sort(left, right)
                })
            }
            Prop::NatEq(left, right) => {
                self.at_path(SyntaxEdge::First, |validator| {
                    validator.validate_natural(left, depth + 1)
                })?;

                self.at_path(SyntaxEdge::Second, |validator| {
                    validator.validate_natural(right, depth + 1)
                })
            }
            Prop::IsEmpty(sequence) => self.at_path(SyntaxEdge::Unary, |validator| {
                validator.validate_sequence(sequence, depth + 1).map(|_| ())
            }),
            Prop::Contains(sequence, element) => {
                let left = self.at_path(SyntaxEdge::First, |validator| {
                    validator.validate_sequence(sequence, depth + 1)
                })?;

                self.at_path(SyntaxEdge::Second, |validator| {
                    let right = validator.validate_element(element, bound_element, depth + 1)?;

                    validator.require_same_element_sort(left, right)
                })
            }
            Prop::All(sequence, predicate) => {
                if bound_element.is_some() {
                    return Err(self.error(
                        Code::NestedBinder,
                        "nested all binders are not supported",
                        "remove the inner all binder; this language supports one bound element at a time",
                    ));
                }

                let element = self.at_path(SyntaxEdge::First, |validator| {
                    validator.validate_sequence(sequence, depth + 1)
                })?;

                self.at_path(SyntaxEdge::Second, |validator| {
                    validator.validate_proposition(predicate, Some(element), depth + 1)
                })
            }
            Prop::Not(predicate) => self.at_path(SyntaxEdge::Unary, |validator| {
                validator.validate_proposition(predicate, bound_element, depth + 1)
            }),
            Prop::And(left, right) | Prop::Or(left, right) | Prop::Implies(left, right) => {
                self.at_path(SyntaxEdge::First, |validator| {
                    validator.validate_proposition(left, bound_element, depth + 1)
                })?;

                self.at_path(SyntaxEdge::Second, |validator| {
                    validator.validate_proposition(right, bound_element, depth + 1)
                })
            }
        }
    }
}
