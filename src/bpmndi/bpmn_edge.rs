//! bpmn_edge

use crate::bpmndi::*;
use crate::Builder;

/// Conversion of BPMNEdge (Class : BPMNEdge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNEdge",
///     name: "BPMNEdge",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#LabeledEdge",
///             },
///         ),
///     ),
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNEdge-label",
///                 name: "label",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BPMNLabel",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
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
///                 association: "A_label_edge",
///                 redefined_property_link: None,
///                 subsetted_property_link: Some(
///                     Property(
///                         SubsettedProperty {
///                             href: "DI.cmof#LabeledEdge-ownedLabel",
///                         },
///                     ),
///                 ),
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNEdge-bpmnElement",
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
///                 association: "A_bpmnElement_edge",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNEdge-sourceElement",
///                 name: "sourceElement",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ClassLink(
///                         ClassLink {
///                             href: "DI.cmof#DiagramElement",
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
///                 association: "A_sourceElement_sourceEdge",
///                 redefined_property_link: Some(
///                     Property(
///                         RedefinedProperty {
///                             href: "DI.cmof#Edge-source",
///                         },
///                     ),
///                 ),
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNEdge-targetElement",
///                 name: "targetElement",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ClassLink(
///                         ClassLink {
///                             href: "DI.cmof#DiagramElement",
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
///                 association: "A_targetElement_targetEdge",
///                 redefined_property_link: Some(
///                     Property(
///                         RedefinedProperty {
///                             href: "DI.cmof#Edge-target",
///                         },
///                     ),
///                 ),
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNEdge-messageVisibleKind",
///                 name: "messageVisibleKind",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageVisibleKind",
///                 ),
///                 complex_type: None,
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
pub struct BPMNEdge<'a> {
    #[builder(setter(into, strip_option), default)]
    pub label: Option<&'a BPMNLabel<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub bpmn_element: Option<i8>,
    #[builder(setter(into, strip_option), default)]
    pub source_element: Option<i8>,
    #[builder(setter(into, strip_option), default)]
    pub target_element: Option<i8>,
    #[builder(setter(into, strip_option), default)]
    pub message_visible_kind: Option<&'a MessageVisibleKind<'a>>,
}

