use alloc::{borrow::Cow, string::String};

use eunomia::RealField;

use crate::BiologicalResponse;

/// Named tissue composed with one statically dispatched response law.
#[derive(Clone, Debug, PartialEq)]
pub struct Tissue<'name, Model> {
    name: Cow<'name, str>,
    model: Model,
}

impl<'name, Model> Tissue<'name, Model> {
    /// Construct a tissue with a borrowed name and no name allocation.
    #[must_use]
    pub const fn borrowed(name: &'name str, model: Model) -> Self {
        Self {
            name: Cow::Borrowed(name),
            model,
        }
    }

    /// Construct a tissue with an owned runtime name.
    #[must_use]
    pub fn owned(name: String, model: Model) -> Self {
        Self {
            name: Cow::Owned(name),
            model,
        }
    }

    /// Borrow the tissue name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Borrow the response model.
    #[must_use]
    pub const fn model(&self) -> &Model {
        &self.model
    }

    /// Evaluate the tissue response.
    ///
    /// # Errors
    ///
    /// Returns the model's typed evaluation failure.
    pub fn evaluate<'a, T>(
        &'a self,
        observation: Model::Observation<'a>,
    ) -> Result<Model::Output, Model::Error>
    where
        T: RealField + 'a,
        Model: BiologicalResponse<T>,
    {
        self.model.evaluate(observation)
    }
}
