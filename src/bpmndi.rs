//! bpmndi

// CMOFPackageImport {
//     xmi_type: "cmof:PackageImport",
//     xmi_id: "_packageImport.0",
//     importing_namespace: "_0",
//     imported_package: ImportedPackage {
//         type: "cmof:Package",
//         href: "DI.cmof#_0",
//     },
// }
use crate::di;

// CMOFPackageImport {
//     xmi_type: "cmof:PackageImport",
//     xmi_id: "_packageImport.1",
//     importing_namespace: "_0",
//     imported_package: ImportedPackage {
//         type: "cmof:Package",
//         href: "DC.cmof#_0",
//     },
// }
use crate::dc;

// CMOFPackageImport {
//     xmi_type: "cmof:PackageImport",
//     xmi_id: "_packageImport.2",
//     importing_namespace: "_0",
//     imported_package: ImportedPackage {
//         type: "cmof:Package",
//         href: "BPMN20.cmof#_0",
//     },
// }
use crate::bpmn_20;

/// Conversion of BPMNDiagram (BPMNDiagram)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNDiagram",
///     name: "BPMNDiagram",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Diagram",
///             },
///         ),
///     ),
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNDiagram-plane",
///                     name: "plane",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "BPMNPlane",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_plane_diagram",
///                     ),
///                     redefined_property_link: Some(
///                         Property(
///                             RedefinedProperty {
///                                 href: "DI.cmof#Diagram-rootElement",
///                             },
///                         ),
///                     ),
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNDiagram-labelStyle",
///                     name: "labelStyle",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "BPMNLabelStyle",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_labelStyle_diagram",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: Some(
///                         Property(
///                             SubsettedProperty {
///                                 href: "DI.cmof#Diagram-ownedStyle",
///                             },
///                         ),
///                     ),
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BPMNDiagram {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#Diagram",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub diagram : di::Diagram,
}

/// Conversion of BPMNPlane (BPMNPlane)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNPlane",
///     name: "BPMNPlane",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Plane",
///             },
///         ),
///     ),
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNPlane-bpmnElement",
///                     name: "bpmnElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "BPMN20.cmof#BaseElement",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_bpmnElement_plane",
///                     ),
///                     redefined_property_link: Some(
///                         Property(
///                             RedefinedProperty {
///                                 href: "DI.cmof#DiagramElement-modelElement",
///                             },
///                         ),
///                     ),
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BPMNPlane {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#Plane",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub plane : di::Plane,
}

/// Conversion of BPMNShape (BPMNShape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNShape",
///     name: "BPMNShape",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#LabeledShape",
///             },
///         ),
///     ),
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNShape-bpmnElement",
///                     name: "bpmnElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "BPMN20.cmof#BaseElement",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_bpmnElement_shape",
///                     ),
///                     redefined_property_link: Some(
///                         Property(
///                             RedefinedProperty {
///                                 href: "DI.cmof#DiagramElement-modelElement",
///                             },
///                         ),
///                     ),
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNShape-isHorizontal",
///                     name: "isHorizontal",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNShape-isExpanded",
///                     name: "isExpanded",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNShape-isMarkerVisible",
///                     name: "isMarkerVisible",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNShape-label",
///                     name: "label",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "BPMNLabel",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_label_shape",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: Some(
///                         Property(
///                             SubsettedProperty {
///                                 href: "DI.cmof#LabeledShape-ownedLabel",
///                             },
///                         ),
///                     ),
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNShape-isMessageVisible",
///                     name: "isMessageVisible",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNShape-participantBandKind",
///                     name: "participantBandKind",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "ParticipantBandKind",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNShape-choreographyActivityShape",
///                     name: "choreographyActivityShape",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "BPMNShape",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_choreographyActivityShape_participantBandShape",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BPMNShape {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#LabeledShape",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub labeled_shape : di::LabeledShape,
}

/// Conversion of BPMNEdge (BPMNEdge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNEdge",
///     name: "BPMNEdge",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#LabeledEdge",
///             },
///         ),
///     ),
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNEdge-label",
///                     name: "label",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "BPMNLabel",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_label_edge",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: Some(
///                         Property(
///                             SubsettedProperty {
///                                 href: "DI.cmof#LabeledEdge-ownedLabel",
///                             },
///                         ),
///                     ),
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNEdge-bpmnElement",
///                     name: "bpmnElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "BPMN20.cmof#BaseElement",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_bpmnElement_edge",
///                     ),
///                     redefined_property_link: Some(
///                         Property(
///                             RedefinedProperty {
///                                 href: "DI.cmof#DiagramElement-modelElement",
///                             },
///                         ),
///                     ),
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNEdge-sourceElement",
///                     name: "sourceElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "DI.cmof#DiagramElement",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_sourceElement_sourceEdge",
///                     ),
///                     redefined_property_link: Some(
///                         Property(
///                             RedefinedProperty {
///                                 href: "DI.cmof#Edge-source",
///                             },
///                         ),
///                     ),
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNEdge-targetElement",
///                     name: "targetElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "DI.cmof#DiagramElement",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_targetElement_targetEdge",
///                     ),
///                     redefined_property_link: Some(
///                         Property(
///                             RedefinedProperty {
///                                 href: "DI.cmof#Edge-target",
///                             },
///                         ),
///                     ),
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNEdge-messageVisibleKind",
///                     name: "messageVisibleKind",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "MessageVisibleKind",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BPMNEdge {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#LabeledEdge",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub labeled_edge : di::LabeledEdge,
}

/// Conversion of BPMNLabel (BPMNLabel)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNLabel",
///     name: "BPMNLabel",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Label",
///             },
///         ),
///     ),
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNLabel-labelStyle",
///                     name: "labelStyle",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "BPMNLabelStyle",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_labelStyle_label",
///                     ),
///                     redefined_property_link: Some(
///                         Property(
///                             RedefinedProperty {
///                                 href: "DI.cmof#DiagramElement-style",
///                             },
///                         ),
///                     ),
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BPMNLabel {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#Label",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub label : di::Label,
}

/// Conversion of BPMNLabelStyle (BPMNLabelStyle)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNLabelStyle",
///     name: "BPMNLabelStyle",
///     is_abstract: None,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Style",
///             },
///         ),
///     ),
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "BPMNLabelStyle-font",
///                     name: "font",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:DataType",
///                             href: "DC.cmof#Font",
///                         },
///                     ),
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: None,
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct BPMNLabelStyle {
    // RAW | is_abstract : None
    // RAW | super_class : None
    // RAW | super_class_link : Some(
    // RAW |     Class(
    // RAW |         SuperClass {
    // RAW |             href: "DI.cmof#Style",
    // RAW |         },
    // RAW |     ),
    // RAW | )
    pub style : di::Style,
}

/// struct_level : ParticipantBandKind
enum ParticipantBandKind {
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

/// struct_level : MessageVisibleKind
enum MessageVisibleKind {
    /// 'Initiating' from (id : 'MessageVisibleKind-initiating', name : 'initiating')
    Initiating, 
    /// 'NonInitiating' from (id : 'MessageVisibleKind-non_initiating', name : 'non_initiating')
    NonInitiating, 
}

// struct_level : A_plane_diagram

// struct_level : A_bpmnElement_edge

// struct_level : A_bpmnElement_shape

// struct_level : A_bpmnElement_plane

// struct_level : A_label_edge

// struct_level : A_label_shape

// struct_level : A_labelStyle_label

// struct_level : A_sourceElement_sourceEdge

// struct_level : A_targetElement_targetEdge

// struct_level : A_labelStyle_diagram

// struct_level : A_choreographyActivityShape_participantBandShape
