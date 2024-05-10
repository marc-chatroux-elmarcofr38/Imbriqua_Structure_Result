//! di

// CMOFPackageImport {
//     xmi_type: "cmof:PackageImport",
//     xmi_id: "_packageImport.0",
//     importing_namespace: "_0",
//     imported_package: ImportedPackage {
//         type: "cmof:Package",
//         href: "DC.cmof#_0",
//     },
// }
use crate::dc;

/// Conversion of DiagramElement (DiagramElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DiagramElement",
///     name: "DiagramElement",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DiagramElement-owningDiagram",
///                     name: "owningDiagram",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Diagram",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_rootElement_owningDiagram",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DiagramElement-owningElement",
///                     name: "owningElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "DiagramElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_ownedElement_owningElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DiagramElement-modelElement",
///                     name: "modelElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:Class",
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_modelElement_diagramElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DiagramElement-style",
///                     name: "style",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Style",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_style_diagramElement",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "DiagramElement-ownedElement",
///                     name: "ownedElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "DiagramElement",
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
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_ownedElement_owningElement",
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
pub struct DiagramElement {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Node (Node)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Node",
///     name: "Node",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "DiagramElement",
///     ),
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Node {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "DiagramElement",
    // RAW | )
    // RAW | super_class_link : None
    pub diagram_element : DiagramElement,
}

/// Conversion of Edge (Edge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Edge",
///     name: "Edge",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "DiagramElement",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Edge-source",
///                     name: "source",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "DiagramElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_source_sourceEdge",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Edge-target",
///                     name: "target",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "DiagramElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: Some(
///                         "0",
///                     ),
///                     upper: None,
///                     default: None,
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: None,
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_target_targetEdge",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Edge-waypoint",
///                     name: "waypoint",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:DataType",
///                             href: "DC.cmof#Point",
///                         },
///                     ),
///                     datatype: None,
///                     lower: Some(
///                         "2",
///                     ),
///                     upper: Some(
///                         "*",
///                     ),
///                     default: None,
///                     is_read_only: None,
///                     is_composite: None,
///                     is_unique: Some(
///                         "false",
///                     ),
///                     is_ordered: Some(
///                         "true",
///                     ),
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
pub struct Edge {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "DiagramElement",
    // RAW | )
    // RAW | super_class_link : None
    pub diagram_element : DiagramElement,
}

/// Conversion of Diagram (Diagram)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Diagram",
///     name: "Diagram",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Diagram-rootElement",
///                     name: "rootElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "DiagramElement",
///                     ),
///                     complex_type: None,
///                     datatype: None,
///                     lower: None,
///                     upper: None,
///                     default: None,
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_rootElement_owningDiagram",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Diagram-name",
///                     name: "name",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
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
///                     xmi_id: "Diagram-documentation",
///                     name: "documentation",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#String",
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
///                     xmi_id: "Diagram-resolution",
///                     name: "resolution",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:PrimitiveType",
///                             href: "DC.cmof#Real",
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
///                     xmi_id: "Diagram-ownedStyle",
///                     name: "ownedStyle",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Style",
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
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: None,
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_ownedStyle_owningDiagram",
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
pub struct Diagram {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Shape (Shape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Shape",
///     name: "Shape",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "Node",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Shape-bounds",
///                     name: "bounds",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:DataType",
///                             href: "DC.cmof#Bounds",
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
pub struct Shape {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "Node",
    // RAW | )
    // RAW | super_class_link : None
    pub node : Node,
}

/// Conversion of Plane (Plane)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Plane",
///     name: "Plane",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "Node",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Plane-planeElement",
///                     name: "planeElement",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "DiagramElement",
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
///                     is_ordered: Some(
///                         "true",
///                     ),
///                     is_abstract: None,
///                     is_derived: None,
///                     subsetted_property: Some(
///                         "DiagramElement-ownedElement",
///                     ),
///                     owning_association: None,
///                     is_derived_union: None,
///                     association: Some(
///                         "A_planeElement_plane",
///                     ),
///                     redefined_property_link: None,
///                     subsetted_property_link: None,
///                 },
///             ),
///         ],
///     ),
///     owned_rule: Some(
///         Constraint(
///             CMOFConstraint {
///                 xmi_id: "Plane-plane_element_type",
///                 name: "plane_element_type",
///                 constrained_element: "Plane",
///                 namespace: "Plane",
///                 specification: Specification {
///                     xmi_type: "cmof:OpaqueExpression",
///                     xmi_id: "Plane-plane_element_type-_specification",
///                     language: "OCL",
///                     body: "planeElement->forAll(oclIsKindOf(Shape) or oclIsKindOf(Edge))",
///                 },
///             },
///         ),
///     ),
/// }
/// ```
pub struct Plane {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "Node",
    // RAW | )
    // RAW | super_class_link : None
    pub node : Node,
}

/// Conversion of LabeledEdge (LabeledEdge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LabeledEdge",
///     name: "LabeledEdge",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "Edge",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "LabeledEdge-ownedLabel",
///                     name: "ownedLabel",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Label",
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
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: Some(
///                         "DiagramElement-ownedElement",
///                     ),
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_ownedLabel_owningEdge",
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
pub struct LabeledEdge {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "Edge",
    // RAW | )
    // RAW | super_class_link : None
    pub edge : Edge,
}

/// Conversion of LabeledShape (LabeledShape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LabeledShape",
///     name: "LabeledShape",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "Shape",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "LabeledShape-ownedLabel",
///                     name: "ownedLabel",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: Some(
///                         "Label",
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
///                     is_read_only: Some(
///                         "true",
///                     ),
///                     is_composite: Some(
///                         "true",
///                     ),
///                     is_unique: None,
///                     is_ordered: None,
///                     is_abstract: None,
///                     is_derived: Some(
///                         "true",
///                     ),
///                     subsetted_property: Some(
///                         "DiagramElement-ownedElement",
///                     ),
///                     owning_association: None,
///                     is_derived_union: Some(
///                         "true",
///                     ),
///                     association: Some(
///                         "A_ownedLabel_owningShape",
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
pub struct LabeledShape {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "Shape",
    // RAW | )
    // RAW | super_class_link : None
    pub shape : Shape,
}

/// Conversion of Label (Label)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Label",
///     name: "Label",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: Some(
///         "Node",
///     ),
///     super_class_link: None,
///     owned_attribute: Some(
///         [
///             Property(
///                 CMOFProperty {
///                     xmi_id: "Label-bounds",
///                     name: "bounds",
///                     visibility: Some(
///                         "public",
///                     ),
///                     type: None,
///                     complex_type: Some(
///                         ComplexType {
///                             xmi_type: "cmof:DataType",
///                             href: "DC.cmof#Bounds",
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
///         ],
///     ),
///     owned_rule: None,
/// }
/// ```
pub struct Label {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : Some(
    // RAW |     "Node",
    // RAW | )
    // RAW | super_class_link : None
    pub node : Node,
}

/// Conversion of Style (Style)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Style",
///     name: "Style",
///     is_abstract: Some(
///         "true",
///     ),
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: None,
///     owned_rule: None,
/// }
/// ```
pub struct Style {
    // RAW | is_abstract : Some(
    // RAW |     "true",
    // RAW | )
    // is_abstract : true
    // RAW | super_class : None
    // RAW | super_class_link : None
}

// struct_level : A_target_targetEdge

// struct_level : A_source_sourceEdge

// struct_level : A_ownedElement_owningElement

// struct_level : A_modelElement_diagramElement

// struct_level : A_rootElement_owningDiagram

// struct_level : A_ownedLabel_owningEdge

// struct_level : A_planeElement_plane

// struct_level : A_style_diagramElement

// struct_level : A_ownedStyle_owningDiagram

// struct_level : A_ownedLabel_owningShape
