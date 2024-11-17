//! di
use derive_builder::Builder;

/// Conversion of _packageImport.0 (PackageImport)
use crate::dc;

/// Conversion of DiagramElement (Class : DiagramElement)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DiagramElement",
///     name: "DiagramElement",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DiagramElement-owningDiagram",
///                 name: "owningDiagram",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Diagram",
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
///                 association: "A_rootElement_owningDiagram",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DiagramElement-owningElement",
///                 name: "owningElement",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DiagramElement",
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
///                 association: "A_ownedElement_owningElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DiagramElement-modelElement",
///                 name: "modelElement",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:Class",
///                         href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
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
///                 association: "A_modelElement_diagramElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DiagramElement-style",
///                 name: "style",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Style",
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
///                 association: "A_style_diagramElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DiagramElement-ownedElement",
///                 name: "ownedElement",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DiagramElement",
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
///                 association: "A_ownedElement_owningElement",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct DiagramElement {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Node (Class : Node)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Node",
///     name: "Node",
///     is_abstract: false,
///     super_class: Some(
///         "DiagramElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Node {
    // RAW | super_class : Some(
    // RAW |     "DiagramElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_diagram_element : DiagramElement,
}

/// Conversion of Edge (Class : Edge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Edge",
///     name: "Edge",
///     is_abstract: false,
///     super_class: Some(
///         "DiagramElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Edge-source",
///                 name: "source",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DiagramElement",
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
///                 association: "A_source_sourceEdge",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Edge-target",
///                 name: "target",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DiagramElement",
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
///                 association: "A_target_targetEdge",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Edge-waypoint",
///                 name: "waypoint",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:DataType",
///                         href: "DC.cmof#Point",
///                     },
///                 ),
///                 datatype: None,
///                 lower: 2,
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
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Edge {
    // RAW | super_class : Some(
    // RAW |     "DiagramElement",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_diagram_element : DiagramElement,
}

/// Conversion of Diagram (Class : Diagram)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Diagram",
///     name: "Diagram",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Diagram-rootElement",
///                 name: "rootElement",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DiagramElement",
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
///                 association: "A_rootElement_owningDiagram",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Diagram-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
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
///                 xmi_id: "Diagram-documentation",
///                 name: "documentation",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#String",
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
///                 xmi_id: "Diagram-resolution",
///                 name: "resolution",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:PrimitiveType",
///                         href: "DC.cmof#Real",
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
///                 xmi_id: "Diagram-ownedStyle",
///                 name: "ownedStyle",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Style",
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
///                 association: "A_ownedStyle_owningDiagram",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Diagram {
    // RAW | super_class : None
    // RAW | super_class_link : None
}

/// Conversion of Shape (Class : Shape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Shape",
///     name: "Shape",
///     is_abstract: false,
///     super_class: Some(
///         "Node",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Shape-bounds",
///                 name: "bounds",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:DataType",
///                         href: "DC.cmof#Bounds",
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
pub struct Shape {
    // RAW | super_class : Some(
    // RAW |     "Node",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_node : Node,
}

/// Conversion of Plane (Class : Plane)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Plane",
///     name: "Plane",
///     is_abstract: false,
///     super_class: Some(
///         "Node",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Plane-planeElement",
///                 name: "planeElement",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DiagramElement",
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
///                 subsetted_property: Some(
///                     "DiagramElement-ownedElement",
///                 ),
///                 owning_association: "",
///                 association: "A_planeElement_plane",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [
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
///     ],
/// }
/// ```
pub struct Plane {
    // RAW | super_class : Some(
    // RAW |     "Node",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_node : Node,
}

/// Conversion of LabeledEdge (Class : LabeledEdge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LabeledEdge",
///     name: "LabeledEdge",
///     is_abstract: false,
///     super_class: Some(
///         "Edge",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "LabeledEdge-ownedLabel",
///                 name: "ownedLabel",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Label",
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
///                 subsetted_property: Some(
///                     "DiagramElement-ownedElement",
///                 ),
///                 owning_association: "",
///                 association: "A_ownedLabel_owningEdge",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct LabeledEdge {
    // RAW | super_class : Some(
    // RAW |     "Edge",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_edge : Edge,
}

/// Conversion of LabeledShape (Class : LabeledShape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LabeledShape",
///     name: "LabeledShape",
///     is_abstract: false,
///     super_class: Some(
///         "Shape",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "LabeledShape-ownedLabel",
///                 name: "ownedLabel",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Label",
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
///                 subsetted_property: Some(
///                     "DiagramElement-ownedElement",
///                 ),
///                 owning_association: "",
///                 association: "A_ownedLabel_owningShape",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct LabeledShape {
    // RAW | super_class : Some(
    // RAW |     "Shape",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_shape : Shape,
}

/// Conversion of Label (Class : Label)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Label",
///     name: "Label",
///     is_abstract: false,
///     super_class: Some(
///         "Node",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Label-bounds",
///                 name: "bounds",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ComplexType {
///                         xmi_type: "cmof:DataType",
///                         href: "DC.cmof#Bounds",
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
///     ],
///     owned_rule: [],
/// }
/// ```
pub struct Label {
    // RAW | super_class : Some(
    // RAW |     "Node",
    // RAW | )
    // RAW | super_class_link : None
    pub heritage_node : Node,
}

/// Conversion of Style (Class : Style)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Style",
///     name: "Style",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```
pub struct Style {
    // RAW | super_class : None
    // RAW | super_class_link : None
}
