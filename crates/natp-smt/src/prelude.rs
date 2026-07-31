//! Recursive list definitions instantiated from the typed solver vocabulary.

use natp_core::Element;

use crate::vocabulary::{Binding, ListSymbol, Logic, Operator, SmtCommand, Sort};

pub(super) fn generate() -> String {
    let mut prelude = String::new();
    prelude.push_str(&SmtCommand::SetLogic.render(&[Logic::All.token().into()]));
    for element in [Element::I32, Element::Bool] {
        prelude.push_str(&format!(
            include_str!("list.smt2"),
            list_sort = Sort::List(element).render(),
            element_sort = Sort::element(element).render(),
            integer_sort = Sort::Integer.render(),
            boolean_sort = Sort::Boolean.render(),
            nil = ListSymbol::Empty.render(element),
            cons = ListSymbol::Cons.render(element),
            head = ListSymbol::Head.render(element),
            tail = ListSymbol::Tail.render(element),
            app = ListSymbol::Append.render(element),
            rev = ListSymbol::Reverse.render(element),
            len = ListSymbol::Length.render(element),
            mem = ListSymbol::Membership.render(element),
            ite = Operator::IfThenElse.token(),
            add = Operator::Add.token(),
            or = Operator::Or.token(),
            equal = Operator::Equal.token(),
            xs = Binding::ListLeft.render(),
            ys = Binding::ListRight.render(),
            x = Binding::Member.render(),
            false_value = false,
            declare_datatype = SmtCommand::DeclareDatatype.token(),
            define_recursive = SmtCommand::DefineRecursiveFunction.token(),
        ));
    }

    prelude
}
