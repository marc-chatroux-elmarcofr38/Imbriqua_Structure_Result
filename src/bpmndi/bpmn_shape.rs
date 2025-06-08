//! bpmn_shape
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmndi::*;
use crate::Builder;

/// Conversion of BPMNShape (Class : BPMNShape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNShape",
///     name: "BPMNShape",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#LabeledShape",
///             },
///         ),
///     ),
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNShape-bpmnElement",
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
///                 association: "A_bpmnElement_shape",
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
///                 xmi_id: "BPMNShape-isHorizontal",
///                 name: "isHorizontal",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNShape-isExpanded",
///                 name: "isExpanded",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNShape-isMarkerVisible",
///                 name: "isMarkerVisible",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNShape-label",
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
///                 association: "A_label_shape",
///                 redefined_property_link: None,
///                 subsetted_property_link: Some(
///                     Property(
///                         SubsettedProperty {
///                             href: "DI.cmof#LabeledShape-ownedLabel",
///                         },
///                     ),
///                 ),
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNShape-isMessageVisible",
///                 name: "isMessageVisible",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNShape-participantBandKind",
///                 name: "participantBandKind",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ParticipantBandKind",
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
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNShape-choreographyActivityShape",
///                 name: "choreographyActivityShape",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BPMNShape",
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
///                 association: "A_choreographyActivityShape_participantBandShape",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct BPMNShape<'a> {
    #[builder(setter(into, strip_option), default)]
    pub bpmn_element: Option<i8>,
    #[builder(setter(into, strip_option), default)]
    pub is_horizontal: Option<dc::Boolean>,
    #[builder(setter(into, strip_option), default)]
    pub is_expanded: Option<dc::Boolean>,
    #[builder(setter(into, strip_option), default)]
    pub is_marker_visible: Option<dc::Boolean>,
    #[builder(setter(into, strip_option), default)]
    pub label: Option<&'a BPMNLabel<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub is_message_visible: Option<dc::Boolean>,
    #[builder(setter(into, strip_option), default)]
    pub participant_band_kind: Option<&'a ParticipantBandKind<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub choreography_activity_shape: Option<&'a BPMNShape<'a>>,
}

