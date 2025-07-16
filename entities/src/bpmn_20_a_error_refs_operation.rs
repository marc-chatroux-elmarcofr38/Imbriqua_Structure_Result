//! bpmn_20_association_a_error_refs_operation

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_error_refs_operation")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub error_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub operation_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_error::Entity",
        from = "Column::ErrorAId",
        to = "super::bpmn_20_error::Column::Id"
    )]
    Error,
    #[sea_orm(
        belongs_to = "super::bpmn_20_operation::Entity",
        from = "Column::OperationBId",
        to = "super::bpmn_20_operation::Column::Id"
    )]
    Operation,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: "A_errorRefs_operation",
//     name: "A_errorRefs_operation",
//     visibility: Private,
//     member_end: (
//         "Operation-errorRefs",
//         "A_errorRefs_operation-operation",
//     ),
//     owned_end: {
//         "A_errorRefs_operation-operation": Property(
//             CMOFProperty {
//                 xmi_id: "A_errorRefs_operation-operation",
//                 name: "operation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Operation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_errorRefs_operation",
//                 association: Some(
//                     "A_errorRefs_operation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_errorRefs_operation",
//     table_name: "bpmn_20_a_error_refs_operation",
//     model_name: "AErrorRefsOperation",
//     full_name: "bpmn_20_association_a_error_refs_operation",
// }

