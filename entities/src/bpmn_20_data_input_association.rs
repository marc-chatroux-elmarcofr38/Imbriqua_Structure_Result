//! bpmn_20_class_data_input_association

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_data_input_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : DataAssociation
    pub super_data_association: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE DataInputAssociation need ONE DataAssociation
    #[sea_orm(
        belongs_to = "super::bpmn_20_data_association::Entity",
        from = "Column::SuperDataAssociation",
        to = "super::bpmn_20_data_association::Column::Id"
    )]
    DataAssociation,
}

// SUPER : ONE DataInputAssociation need ONE DataAssociation
impl Related<super::bpmn_20_data_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataAssociation.def()
    }
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

