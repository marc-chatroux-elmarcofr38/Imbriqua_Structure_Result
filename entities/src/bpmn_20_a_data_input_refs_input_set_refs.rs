//! bpmn_20_association_a_data_input_refs_input_set_refs

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_data_input_refs_input_set_refs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub data_input_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub input_set_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_data_input::Entity",
        from = "Column::DataInputAId",
        to = "super::bpmn_20_data_input::Column::Id"
    )]
    DataInput,
    #[sea_orm(
        belongs_to = "super::bpmn_20_input_set::Entity",
        from = "Column::InputSetBId",
        to = "super::bpmn_20_input_set::Column::Id"
    )]
    InputSet,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: "A_dataInputRefs_inputSetRefs",
//     name: "A_dataInputRefs_inputSetRefs",
//     visibility: Private,
//     member_end: (
//         "InputSet-dataInputRefs",
//         "DataInput-inputSetRefs",
//     ),
//     owned_end: {},
//     is_derived: false,
// }

