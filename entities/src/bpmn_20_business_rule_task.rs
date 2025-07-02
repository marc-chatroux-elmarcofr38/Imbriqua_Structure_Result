//! bpmn_20_class_business_rule_task

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_business_rule_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : Task
    pub super_task: i32,
    /// SIMPLE FIELD : BusinessRuleTask-implementation
    pub implementation: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_task::Entity",
        from = "Column::SuperTask",
        to = "super::bpmn_20_task::Column::Id"
    )]
    Task,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "BusinessRuleTask",
//     name: "BusinessRuleTask",
//     is_abstract: false,
//     super_class: [
//         "Task",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "BusinessRuleTask-implementation",
//                 name: "implementation",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
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

