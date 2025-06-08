//! formal_expression
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of FormalExpression (Class : FormalExpression)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FormalExpression",
///     name: "FormalExpression",
///     is_abstract: false,
///     super_class: Some(
///         "Expression",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "FormalExpression-language",
///                 name: "language",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "FormalExpression-body",
///                 name: "body",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ClassLink(
///                         ClassLink {
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "FormalExpression-evaluatesToTypeRef",
///                 name: "evaluatesToTypeRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ItemDefinition",
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
///                 association: "A_evaluatesToTypeRef_formalExpression",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct FormalExpression<'a> {
    #[builder(setter(into))]
    pub language: dc::String,
    #[builder(setter(into))]
    pub body: i8,
    #[builder(setter(into))]
    pub evaluates_to_type_ref: &'a ItemDefinition<'a>,
}

