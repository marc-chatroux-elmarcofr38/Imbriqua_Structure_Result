//! bpmn_20_class_expression

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_expression")]
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
    #[sea_orm(has_one = "super::bpmn_20_formal_expression::Entity")]
    FormalExpression,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_formal_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormalExpression.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Expression",
//     name: "Expression",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }

