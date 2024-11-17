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
///     is_abstract: true,
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
///                 is_read_only: true,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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
///                 is_read_only: true,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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
///                 is_read_only: true,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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
///                 is_read_only: true,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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
///                 is_read_only: true,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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

#[derive(Builder, Debug, Clone)]
pub struct DiagramElement<'a> {
    #[builder(setter(into, strip_option), default)]
    pub owning_diagram: Option<&'a Diagram<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub owning_element: Option<&'a DiagramElement<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub model_element: Option<i8>,
    #[builder(setter(into, strip_option), default)]
    pub style: Option<Style>,
    #[builder(setter(into, strip_option), default)]
    pub owned_element: Option<Vec<&'a DiagramElement<'a>>>,
}

/// Conversion of Node (Class : Node)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Node",
///     name: "Node",
///     is_abstract: true,
///     super_class: Some(
///         "DiagramElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct Node<'a> {
    pub heritage_diagram_element: &'a DiagramElement<'a>, //super_class
}

/// Conversion of Edge (Class : Edge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Edge",
///     name: "Edge",
///     is_abstract: true,
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
///                 is_read_only: true,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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
///                 is_read_only: true,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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
///                 is_ordered: true,
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
pub struct Edge<'a> {
    pub heritage_diagram_element: &'a DiagramElement<'a>, //super_class
    #[builder(setter(into, strip_option), default)]
    pub source: Option<&'a DiagramElement<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub target: Option<&'a DiagramElement<'a>>,
    #[builder(setter(into))]
    pub waypoint: Vec<u8>,
}

/// Conversion of Diagram (Class : Diagram)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Diagram",
///     name: "Diagram",
///     is_abstract: true,
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
///                 is_read_only: true,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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
///                 is_read_only: true,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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

#[derive(Builder, Debug, Clone)]
pub struct Diagram<'a> {
    #[builder(setter(into))]
    pub root_element: &'a DiagramElement<'a>,
    #[builder(setter(into, strip_option), default)]
    pub name: Option<dc::String>,
    #[builder(setter(into, strip_option), default)]
    pub documentation: Option<dc::String>,
    #[builder(setter(into, strip_option), default)]
    pub resolution: Option<dc::Real>,
    #[builder(setter(into))]
    pub owned_style: Vec<Style>,
}

/// Conversion of Shape (Class : Shape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Shape",
///     name: "Shape",
///     is_abstract: true,
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

#[derive(Builder, Debug, Clone)]
pub struct Shape<'a> {
    pub heritage_node: &'a Node<'a>, //super_class
    #[builder(setter(into))]
    pub bounds: u8,
}

/// Conversion of Plane (Class : Plane)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Plane",
///     name: "Plane",
///     is_abstract: true,
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
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: true,
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

#[derive(Builder, Debug, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Plane<'a> {
    pub heritage_node: &'a Node<'a>, //super_class
    #[builder(setter(into))]
    pub plane_element: Vec<&'a DiagramElement<'a>>,
}

impl PlaneBuilder<'_> {
    // Rule :  plane_element_type - Specification { xmi_type: "cmof:OpaqueExpression", xmi_id: "Plane-plane_element_type-_specification", language: "OCL", body: "planeElement->forAll(oclIsKindOf(Shape) or oclIsKindOf(Edge))" }
    pub fn plane_element_type(&self) -> Result<(), String> {
        return Ok(());
    }

    fn validate(&self) -> Result<(), String> {
        // Rule :  plane_element_type - Specification { xmi_type: "cmof:OpaqueExpression", xmi_id: "Plane-plane_element_type-_specification", language: "OCL", body: "planeElement->forAll(oclIsKindOf(Shape) or oclIsKindOf(Edge))" }
        self.plane_element_type()?;

        return Ok(());
    }
}

/// Conversion of LabeledEdge (Class : LabeledEdge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LabeledEdge",
///     name: "LabeledEdge",
///     is_abstract: true,
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
///                 is_read_only: true,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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

#[derive(Builder, Debug, Clone)]
pub struct LabeledEdge<'a> {
    pub heritage_edge: &'a Edge<'a>, //super_class
    #[builder(setter(into))]
    pub owned_label: Vec<&'a Label<'a>>,
}

/// Conversion of LabeledShape (Class : LabeledShape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LabeledShape",
///     name: "LabeledShape",
///     is_abstract: true,
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
///                 is_read_only: true,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
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

#[derive(Builder, Debug, Clone)]
pub struct LabeledShape<'a> {
    pub heritage_shape: &'a Shape<'a>, //super_class
    #[builder(setter(into))]
    pub owned_label: Vec<&'a Label<'a>>,
}

/// Conversion of Label (Class : Label)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Label",
///     name: "Label",
///     is_abstract: true,
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

#[derive(Builder, Debug, Clone)]
pub struct Label<'a> {
    pub heritage_node: &'a Node<'a>, //super_class
    #[builder(setter(into, strip_option), default)]
    pub bounds: Option<u8>,
}

/// Conversion of Style (Class : Style)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Style",
///     name: "Style",
///     is_abstract: true,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct Style {}
