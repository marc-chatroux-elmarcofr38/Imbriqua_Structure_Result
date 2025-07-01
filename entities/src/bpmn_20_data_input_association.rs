//! bpmn_20_class_data_input_association

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_input_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pk_id: i32,
    /// SIMPLE FIELD : DataAssociation
    pub super_data_association: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "DataInputAssociation",
//     name: "DataInputAssociation",
//     is_abstract: false,
//     super_class: [
//         "DataAssociation",
//     ],
//     super_class_link: [],
//     owned_attribute: [],
//     owned_rule: [],
// }

