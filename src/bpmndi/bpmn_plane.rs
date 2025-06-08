//! bpmn_plane

use crate::bpmndi::*;
use crate::Builder;

/// Conversion of BPMNPlane (Class : BPMNPlane)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNPlane",
///     name: "BPMNPlane",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Plane",
///             },
///         ),
///     ),
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNPlane-bpmnElement",
///                 name: "bpmnElement",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ClassLink(
///                         ClassLink {
///                             href: "BPMN20.cmof#BaseElement",
///                         },
///                     ),
///                 ),
///                 datatype: None,
///                 lower: 0,
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
///                 association: "A_bpmnElement_plane",
///                 redefined_property_link: Some(
///                     Property(
///                         RedefinedProperty {
///                             href: "DI.cmof#DiagramElement-modelElement",
///                         },
///                     ),
///                 ),
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct BPMNPlane<'a> {
    #[builder(setter(into, strip_option), default)]
    pub bpmn_element: Option<i8>,
}

