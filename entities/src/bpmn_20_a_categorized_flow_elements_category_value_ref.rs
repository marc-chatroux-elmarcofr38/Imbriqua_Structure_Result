//! bpmn_20_association_a_categorized_flow_elements_category_value_ref

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_categorized_flow_elements_category_value_ref")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_value_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub flow_element_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_category_value::Entity",
        from = "Column::CategoryValueAId",
        to = "super::bpmn_20_category_value::Column::Id"
    )]
    CategoryValue,
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_element::Entity",
        from = "Column::FlowElementBId",
        to = "super::bpmn_20_flow_element::Column::Id"
    )]
    FlowElement,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: "A_categorizedFlowElements_categoryValueRef",
//     name: "A_categorizedFlowElements_categoryValueRef",
//     visibility: Private,
//     member_end: (
//         "CategoryValue-categorizedFlowElements",
//         "FlowElement-categoryValueRef",
//     ),
//     owned_end: {},
//     is_derived: false,
// }

