//! bpmn_20_class_data_input_association

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel, Default)]
#[sea_orm(table_name = "bpmn_20_data_input_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "DataInputAssociation",
//     name: "DataInputAssociation",
//     is_abstract: false,
//     super_class: Some(
//         "DataAssociation",
//     ),
//     super_class_link: None,
//     owned_attribute: [],
//     owned_rule: [],
// }

