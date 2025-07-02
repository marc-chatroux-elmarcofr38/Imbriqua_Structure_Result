//! bpmn_20_class_choreography_task

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_choreography_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : ChoreographyActivity
    pub super_choreography_activity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_choreography_activity::Entity",
        from = "Column::SuperChoreographyActivity",
        to = "super::bpmn_20_choreography_activity::Column::Id"
    )]
    ChoreographyActivity,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_choreography_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChoreographyActivity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ChoreographyTask",
//     name: "ChoreographyTask",
//     is_abstract: false,
//     super_class: [
//         "ChoreographyActivity",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ChoreographyTask-messageFlowRef",
//                 name: "messageFlowRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "MessageFlow",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     2,
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
//                     "A_messageFlowRef_choreographyTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

