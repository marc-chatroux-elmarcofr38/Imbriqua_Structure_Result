//! ResourceAssignmentExpression
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ResourceAssignmentExpression (Class : ResourceAssignmentExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ResourceAssignmentExpression",
///     name: "ResourceAssignmentExpression",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ResourceAssignmentExpression-expression",
///                 name: "expression",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
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
///                 association: "A_expression_resourceAssignmentExpression",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct ResourceAssignmentExpression {
    #[builder(setter(into))]
    pub expression: Expression,
}

