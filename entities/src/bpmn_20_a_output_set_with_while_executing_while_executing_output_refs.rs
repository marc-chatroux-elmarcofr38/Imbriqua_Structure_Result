//! bpmn_20_association_a_output_set_with_while_executing_while_executing_output_refs

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_output_set_with_while_executing_while_executing_output_refs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub data_output_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub output_set_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_data_output::Entity",
        from = "Column::DataOutputAId",
        to = "super::bpmn_20_data_output::Column::Id"
    )]
    DataOutput,
    #[sea_orm(
        belongs_to = "super::bpmn_20_output_set::Entity",
        from = "Column::OutputSetBId",
        to = "super::bpmn_20_output_set::Column::Id"
    )]
    OutputSet,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: "A_outputSetWithWhileExecuting_whileExecutingOutputRefs",
//     name: "A_outputSetWithWhileExecuting_whileExecutingOutputRefs",
//     visibility: Private,
//     member_end: (
//         "DataOutput-outputSetWithWhileExecuting",
//         "OutputSet-whileExecutingOutputRefs",
//     ),
//     owned_end: {},
//     is_derived: false,
// }

