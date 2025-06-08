//! ad_hoc_sub_process

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of AdHocSubProcess (Class : AdHocSubProcess)
///
/// ```json
/// CMOFClass {
///     xmi_id: "AdHocSubProcess",
///     name: "AdHocSubProcess",
///     is_abstract: false,
///     super_class: Some(
///         "SubProcess",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "AdHocSubProcess-completionCondition",
///                 name: "completionCondition",
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
///                 association: "A_completionCondition_adHocSubProcess",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "AdHocSubProcess-ordering",
///                 name: "ordering",
///                 visibility: Public,
///                 simple_type: Some(
///                     "AdHocOrdering",
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
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "AdHocSubProcess-cancelRemainingInstances",
///                 name: "cancelRemainingInstances",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     PrimitiveTypeLink(
///                         PrimitiveTypeLink {
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "true",
///                 ),
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
pub struct AdHocSubProcess<'a> {
    #[builder(setter(into))]
    pub completion_condition: &'a Expression<'a>,
    #[builder(setter(into))]
    pub ordering: &'a AdHocOrdering<'a>,
    #[builder(setter(into), default = "true")]
    pub cancel_remaining_instances: dc::Boolean,
}

