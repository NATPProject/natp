//! One length vector flattened into independent scalar input cells.

use std::{iter::repeat_n, ops::Range};

use natp_core::{Assignment, Domain, Element, Value};

use crate::input_variable::InputVariable;

/// Scalar variables occupy one cell; sequences occupy their selected lengths.
pub(super) struct InputLayout {
    variables: Vec<InputVariable>,
    elements: Vec<Element>,
}

impl InputLayout {
    pub(super) fn new(domains: &[Domain], lengths: &[usize]) -> Self {
        let mut elements = Vec::new();
        let variables = domains
            .iter()
            .zip(lengths)
            .map(|(&domain, &length)| {
                let start = elements.len();
                elements.extend(repeat_n(domain.element(), length));

                InputVariable::new(domain, start..elements.len())
            })
            .collect();

        Self {
            variables,
            elements,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.elements.len()
    }

    pub(super) fn element(&self, cell: usize) -> Element {
        self.elements[cell]
    }

    pub(super) fn positions(&self, variable: usize) -> Range<usize> {
        self.variables[variable].positions()
    }

    pub(super) fn zeroes(&self) -> Vec<Value> {
        self.elements
            .iter()
            .map(|element| match element {
                Element::I32 => Value::I32(0),
                Element::Bool => Value::Bool(false),
            })
            .collect()
    }

    pub(super) fn bind(&self, values: &[Value]) -> Vec<Assignment> {
        self.variables
            .iter()
            .map(|variable| variable.bind(values))
            .collect()
    }
}
