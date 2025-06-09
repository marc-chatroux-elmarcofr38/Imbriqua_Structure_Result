//! Edge
#![allow(unused_imports)]

use crate::di::*;
use crate::Builder;

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
///                     DataTypeLink(
///                         DataTypeLink {
///                             href: "DC.cmof#Point",
///                         },
///                     ),
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
pub struct Edge {
    #[builder(setter(into, strip_option), default)]
    pub source: Option<DiagramElement>,
    #[builder(setter(into, strip_option), default)]
    pub target: Option<DiagramElement>,
    #[builder(setter(into))]
    pub waypoint: Vec<i8>,
}

