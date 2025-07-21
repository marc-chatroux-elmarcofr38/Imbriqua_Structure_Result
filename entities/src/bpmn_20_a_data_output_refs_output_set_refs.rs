//! bpmn_20_association_a_data_output_refs_output_set_refs

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_data_output_refs_output_set_refs")]
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
//     xmi_id: XMIIdLocalReference {
//         object_id: "A_dataOutputRefs_outputSetRefs",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_dataOutputRefs_outputSetRefs",
//     visibility: Private,
//     member_end: (
//         "RefCell of 'BPMN20-OutputSet-dataOutputRefs' (loaded : true)",
//         "RefCell of 'BPMN20-DataOutput-outputSetRefs' (loaded : true)",
//     ),
//     owned_end: {},
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_dataOutputRefs_outputSetRefs",
//     table_name: "bpmn_20_a_data_output_refs_output_set_refs",
//     model_name: "ADataOutputRefsOutputSetRefs",
//     full_name: "bpmn_20_association_a_data_output_refs_output_set_refs",
// }

