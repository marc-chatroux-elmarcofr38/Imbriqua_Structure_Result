//! correlation_property_binding
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of CorrelationPropertyBinding (Class : CorrelationPropertyBinding)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationPropertyBinding",
///     name: "CorrelationPropertyBinding",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationPropertyBinding-dataPath",
///                 name: "dataPath",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FormalExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataPath_correlationPropertyBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationPropertyBinding-correlationPropertyRef",
///                 name: "correlationPropertyRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationProperty",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_correlationPropertyRef_correlationPropertyBinding",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct CorrelationPropertyBinding<'a> {
    #[builder(setter(into))]
    pub data_path: &'a FormalExpression<'a>,
    #[builder(setter(into))]
    pub correlation_property_ref: &'a CorrelationProperty<'a>,
}

