//! boundary_event

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of BoundaryEvent (Class : BoundaryEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BoundaryEvent",
///     name: "BoundaryEvent",
///     is_abstract: false,
///     super_class: Some(
///         "CatchEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BoundaryEvent-cancelActivity",
///                 name: "cancelActivity",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "BoundaryEvent-attachedToRef",
///                 name: "attachedToRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Activity",
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
///                 association: "A_boundaryEventRefs_attachedToRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct BoundaryEvent<'a> {
    #[builder(setter(into), default = "true")]
    pub cancel_activity: dc::Boolean,
    #[builder(setter(into))]
    pub attached_to_ref: &'a Activity<'a>,
}

