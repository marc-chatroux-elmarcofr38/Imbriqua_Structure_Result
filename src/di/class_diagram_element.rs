//! DiagramElement
#![allow(unused_imports)]

use crate::di::*;
use crate::Builder;

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
///                     ClassLink(
///                         ClassLink {
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
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
pub struct DiagramElement {
    #[builder(setter(into, strip_option), default)]
    pub owning_diagram: Option<Diagram>,
    #[builder(setter(into, strip_option), default)]
    pub owning_element: Option<DiagramElement>,
    #[builder(setter(into, strip_option), default)]
    pub model_element: Option<i8>,
    #[builder(setter(into, strip_option), default)]
    pub style: Option<Style>,
    #[builder(setter(into, strip_option), default)]
    pub owned_element: Option<Vec<DiagramElement>>,
}

