//! di_class_edge

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_edge")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : DiagramElement
    pub super_diagram_element: i64,
    /// COMPLEX FIELD : Edge-source
    pub source: Option<i64>,
    /// COMPLEX FIELD : Edge-target
    pub target: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Edge need ONE DiagramElement
    #[sea_orm(
        belongs_to = "super::di_diagram_element::Entity",
        from = "Column::SuperDiagramElement",
        to = "super::di_diagram_element::Column::Id",
        on_delete = "Cascade"
    )]
    DiagramElement,
    // SUPER : ONE LabeledEdge need ONE Edge
    #[sea_orm(has_one = "super::di_labeled_edge::Entity")]
    LabeledEdge,
}

// SUPER : ONE Edge need ONE DiagramElement
impl Related<super::di_diagram_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DiagramElement.def()
    }
}

// SUPER : ONE LabeledEdge need ONE Edge
impl Related<super::di_labeled_edge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabeledEdge.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Edge",
//     name: "Edge",
//     is_abstract: true,
//     super_class: [
//         "DiagramElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Edge-source",
//                 name: "source",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_source_sourceEdge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Edge-target",
//                 name: "target",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DiagramElement",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: true,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: true,
//                 is_derived_union: true,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_target_targetEdge",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Edge-waypoint",
//                 name: "waypoint",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     DataTypeLink(
//                         DataTypeLink {
//                             href: "DC.cmof#Point",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 2,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: true,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

