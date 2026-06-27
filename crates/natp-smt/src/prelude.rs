//! Recursive list definitions instantiated from the typed solver vocabulary.

use crate::vocabulary::{Binding, ListSymbol, Logic, Operator, SmtCommand, Sort};
use natp_core::Element;

pub(super) fn generate() -> String {
    let mut prelude = String::new();
    prelude.push_str(&SmtCommand::SetLogic.render(&[Logic::All.token().into()]));
    for element in [Element::I32, Element::Bool] {
        prelude.push_str(&format!(
            include_str!("list.smt2"),
            list_sort = Sort::List(element).render(),
            element_sort = Sort::element(element).render(),
            integer_sort = Sort::Integer.render(),
            nil = ListSymbol::Empty.render(element),
            cons = ListSymbol::Cons.render(element),
            head = ListSymbol::Head.render(element),
            tail = ListSymbol::Tail.render(element),
            app = ListSymbol::Append.render(element),
            rev = ListSymbol::Reverse.render(element),
            len = ListSymbol::Length.render(element),
            ite = Operator::IfThenElse.token(),
            add = Operator::Add.token(),
            xs = Binding::ListLeft.render(),
            ys = Binding::ListRight.render(),
            declare_datatype = SmtCommand::DeclareDatatype.token(),
            define_recursive = SmtCommand::DefineRecursiveFunction.token(),
        ));
    }

    prelude
}
