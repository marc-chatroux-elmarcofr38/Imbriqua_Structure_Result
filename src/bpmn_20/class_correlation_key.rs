//! CorrelationKey
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of CorrelationKey (Class : CorrelationKey)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CorrelationKey",
///     name: "CorrelationKey",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationKey-correlationPropertyRef",
///                 name: "correlationPropertyRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CorrelationProperty",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_correlationPropertyRef_correlationKey",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CorrelationKey-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     PrimitiveTypeLink(
///                         PrimitiveTypeLink {
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                 ),
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
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct CorrelationKey {
    #[builder(setter(into, strip_option), default)]
    pub correlation_property_ref: Option<Vec<CorrelationProperty>>,
    #[builder(setter(into))]
    pub name: dc::String,
}

