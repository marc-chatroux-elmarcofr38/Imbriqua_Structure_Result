//! bpmn_20_class_loop_characteristics

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_loop_characteristics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : BaseElement
    pub super_base_element: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
    #[sea_orm(has_one = "super::bpmn_20_multi_instance_loop_characteristics::Entity")]
    MultiInstanceLoopCharacteristics,
    #[sea_orm(has_one = "super::bpmn_20_standard_loop_characteristics::Entity")]
    StandardLoopCharacteristics,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_multi_instance_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MultiInstanceLoopCharacteristics.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_standard_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StandardLoopCharacteristics.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "LoopCharacteristics",
//     name: "LoopCharacteristics",
//     is_abstract: true,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }

