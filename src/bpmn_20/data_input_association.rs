//! data_input_association

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of DataInputAssociation (Class : DataInputAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataInputAssociation",
///     name: "DataInputAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "DataAssociation",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct DataInputAssociation<'a> {
}

