//! bpmn_20_class_loop_characteristics

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_loop_characteristics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE LoopCharacteristics need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE MultiInstanceLoopCharacteristics need ONE LoopCharacteristics
    #[sea_orm(has_one = "super::bpmn_20_multi_instance_loop_characteristics::Entity")]
    MultiInstanceLoopCharacteristics,
    // SUPER : ONE StandardLoopCharacteristics need ONE LoopCharacteristics
    #[sea_orm(has_one = "super::bpmn_20_standard_loop_characteristics::Entity")]
    StandardLoopCharacteristics,
}

// SUPER : ONE LoopCharacteristics need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE MultiInstanceLoopCharacteristics need ONE LoopCharacteristics
impl Related<super::bpmn_20_multi_instance_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MultiInstanceLoopCharacteristics.def()
    }
}

// SUPER : ONE StandardLoopCharacteristics need ONE LoopCharacteristics
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

