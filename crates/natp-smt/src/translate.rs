//! SMT expression translation.
use natp_core::{Error, Prop, Theorem};
pub(super) struct Translator;
impl Translator {
    pub(super) fn new(_theorem: &Theorem) -> Self {
        Self
    }
    pub(super) fn translate_proposition(
        &self,
        proposition: &Prop,
        _bindings: &[String],
    ) -> Result<String, Error> {
        Ok(match proposition {
            Prop::Bool(value) => value.to_string(),
            _ => {
                return Err(Error::new(
                    "proposition is not supported by this SMT translator",
                ));
            }
        })
    }
}
