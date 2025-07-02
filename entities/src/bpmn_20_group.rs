//! bpmn_20_class_group

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Artifact
    pub super_artifact: i32,
    /// COMPLEX FIELD : Group-categoryValueRef
    pub category_value_ref: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_artifact::Entity",
        from = "Column::SuperArtifact",
        to = "super::bpmn_20_artifact::Column::Id"
    )]
    Artifact,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artifact.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Group",
//     name: "Group",
//     is_abstract: false,
//     super_class: [
//         "Artifact",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Group-categoryValueRef",
//                 name: "categoryValueRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CategoryValue",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                 association: Some(
//                     "A_categoryValueRef_categoryValueRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

