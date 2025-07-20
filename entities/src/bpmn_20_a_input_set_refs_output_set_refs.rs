//! bpmn_20_association_a_input_set_refs_output_set_refs

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_input_set_refs_output_set_refs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub input_set_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub output_set_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_input_set::Entity",
        from = "Column::InputSetAId",
        to = "super::bpmn_20_input_set::Column::Id"
    )]
    InputSet,
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
//     xmi_id: "Weak ref of "BPMN20-A_inputSetRefs_outputSetRefs" (loaded : false)",
//     name: "A_inputSetRefs_outputSetRefs",
//     visibility: Private,
//     member_end: (
//         "OutputSet-inputSetRefs",
//         "InputSet-outputSetRefs",
//     ),
//     owned_end: {},
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_inputSetRefs_outputSetRefs",
//     table_name: "bpmn_20_a_input_set_refs_output_set_refs",
//     model_name: "AInputSetRefsOutputSetRefs",
//     full_name: "bpmn_20_association_a_input_set_refs_output_set_refs",
// }

