//! di_class_shape

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "di_shape")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Node
    pub super_node: i64,
    /// COMPLEX FIELD : Shape-bounds
    pub bounds: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Shape need ONE Node
    #[sea_orm(
        belongs_to = "super::di_node::Entity",
        from = "Column::SuperNode",
        to = "super::di_node::Column::Id"
    )]
    Node,
    // SUPER : ONE LabeledShape need ONE Shape
    #[sea_orm(has_one = "super::di_labeled_shape::Entity")]
    LabeledShape,
}

// SUPER : ONE Shape need ONE Node
impl Related<super::di_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Node.def()
    }
}

// SUPER : ONE LabeledShape need ONE Shape
impl Related<super::di_labeled_shape::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabeledShape.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Shape",
//     name: "Shape",
//     is_abstract: true,
//     super_class: [
//         "Node",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Shape-bounds",
//                 name: "bounds",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     DataTypeLink(
//                         DataTypeLink {
//                             href: "DC.cmof#Bounds",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
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

