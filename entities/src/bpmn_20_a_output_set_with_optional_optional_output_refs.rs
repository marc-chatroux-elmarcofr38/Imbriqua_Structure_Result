//! bpmn_20_association_a_output_set_with_optional_optional_output_refs

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_output_set_with_optional_optional_output_refs")]
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
//     xmi_id: XMIIdReference {
//         local_id: "A_outputSetWithOptional_optionalOutputRefs",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_outputSetWithOptional_optionalOutputRefs",
//     visibility: Private,
//     member_end: (
//         "DataOutput-outputSetWithOptional",
//         "OutputSet-optionalOutputRefs",
//     ),
//     owned_end: {},
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_outputSetWithOptional_optionalOutputRefs",
//     table_name: "bpmn_20_a_output_set_with_optional_optional_output_refs",
//     model_name: "AOutputSetWithOptionalOptionalOutputRefs",
//     full_name: "bpmn_20_association_a_output_set_with_optional_optional_output_refs",
// }

