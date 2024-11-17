//! bpmndi
use derive_builder::Builder;

/// Conversion of _packageImport.0 (PackageImport)
use crate::di;

/// Conversion of _packageImport.1 (PackageImport)
use crate::dc;

/// Conversion of _packageImport.2 (PackageImport)
use crate::bpmn_20;

/// Conversion of BPMNDiagram (Class : BPMNDiagram)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNDiagram",
///     name: "BPMNDiagram",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Diagram",
///             },
///         ),
///     ),
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNDiagram-plane",
///                 name: "plane",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BPMNPlane",
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
///                 association: "A_plane_diagram",
///                 redefined_property_link: Some(
///                     Property(
///                         RedefinedProperty {
///                             href: "DI.cmof#Diagram-rootElement",
///                         },
///                     ),
///                 ),
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNDiagram-labelStyle",
///                 name: "labelStyle",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BPMNLabelStyle",
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
///                 association: "A_labelStyle_diagram",
///                 redefined_property_link: None,
///                 subsetted_property_link: Some(
///                     Property(
///                         SubsettedProperty {
///                             href: "DI.cmof#Diagram-ownedStyle",
///                         },
///                     ),
///                 ),
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct BPMNDiagram {
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#Diagram",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub heritage_diagram : di::Diagram,
}

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
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "BPMN20.cmof#BaseElement",
///                     },
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
pub struct BPMNPlane {
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#Plane",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub heritage_plane : di::Plane,
}

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
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "BPMN20.cmof#BaseElement",
///                     },
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
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
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
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
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
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
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
///                 is_composite: false,
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
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Boolean",
///                     },
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
pub struct BPMNShape {
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#LabeledShape",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub heritage_labeled_shape : di::LabeledShape,
}

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
///                 is_composite: false,
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
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "BPMN20.cmof#BaseElement",
///                     },
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
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "DI.cmof#DiagramElement",
///                     },
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
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "DI.cmof#DiagramElement",
///                     },
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
pub struct BPMNEdge {
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#LabeledEdge",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub heritage_labeled_edge : di::LabeledEdge,
}

/// Conversion of BPMNLabel (Class : BPMNLabel)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNLabel",
///     name: "BPMNLabel",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Label",
///             },
///         ),
///     ),
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNLabel-labelStyle",
///                 name: "labelStyle",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BPMNLabelStyle",
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
///                 association: "A_labelStyle_label",
///                 redefined_property_link: Some(
///                     Property(
///                         RedefinedProperty {
///                             href: "DI.cmof#DiagramElement-style",
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
pub struct BPMNLabel {
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#Label",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub heritage_label : di::Label,
}

/// Conversion of BPMNLabelStyle (Class : BPMNLabelStyle)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNLabelStyle",
///     name: "BPMNLabelStyle",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Style",
///             },
///         ),
///     ),
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNLabelStyle-font",
///                 name: "font",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:DataType",
///                         href: "DC.cmof#Font",
///                     },
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
pub struct BPMNLabelStyle {
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#Style",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub heritage_style : di::Style,
}

/// Conversion of ParticipantBandKind (Enumeration : ParticipantBandKind)
pub enum ParticipantBandKind {
    /// 'TopInitiating' from (id : 'ParticipantBandKind-top_initiating', name : 'top_initiating')
    TopInitiating, 
    /// 'MiddleInitiating' from (id : 'ParticipantBandKind-middle_initiating', name : 'middle_initiating')
    MiddleInitiating, 
    /// 'BottomInitiating' from (id : 'ParticipantBandKind-bottom_initiating', name : 'bottom_initiating')
    BottomInitiating, 
    /// 'TopNonInitiating' from (id : 'ParticipantBandKind-top_non_initiating', name : 'top_non_initiating')
    TopNonInitiating, 
    /// 'MiddleNonInitiating' from (id : 'ParticipantBandKind-middle_non_initiating', name : 'middle_non_initiating')
    MiddleNonInitiating, 
    /// 'BottomNonInitiating' from (id : 'ParticipantBandKind-bottom_non_initiating', name : 'bottom_non_initiating')
    BottomNonInitiating, 
}

/// Conversion of MessageVisibleKind (Enumeration : MessageVisibleKind)
pub enum MessageVisibleKind {
    /// 'Initiating' from (id : 'MessageVisibleKind-initiating', name : 'initiating')
    Initiating, 
    /// 'NonInitiating' from (id : 'MessageVisibleKind-non_initiating', name : 'non_initiating')
    NonInitiating, 
}
