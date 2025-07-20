//! bpmn_20_association_a_correlation_property_ref_correlation_key

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_correlation_property_ref_correlation_key")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub correlation_property_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub correlation_key_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_correlation_property::Entity",
        from = "Column::CorrelationPropertyAId",
        to = "super::bpmn_20_correlation_property::Column::Id"
    )]
    CorrelationProperty,
    #[sea_orm(
        belongs_to = "super::bpmn_20_correlation_key::Entity",
        from = "Column::CorrelationKeyBId",
        to = "super::bpmn_20_correlation_key::Column::Id"
    )]
    CorrelationKey,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdReference {
//         object_id: "A_correlationPropertyRef_correlationKey",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_correlationPropertyRef_correlationKey",
//     visibility: Private,
//     member_end: (
//         "CorrelationKey-correlationPropertyRef",
//         "A_correlationPropertyRef_correlationKey-correlationKey",
//     ),
//     owned_end: {
//         "-A_correlationPropertyRef_correlationKey-correlationKey": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "A_correlationPropertyRef_correlationKey-correlationKey",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "correlationKey",
//                 visibility: Public,
//                 simple_type: Some(
//                     "CorrelationKey",
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
//                 owning_association: "A_correlationPropertyRef_correlationKey",
//                 association: Some(
//                     "A_correlationPropertyRef_correlationKey",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_correlationPropertyRef_correlationKey",
//     table_name: "bpmn_20_a_correlation_property_ref_correlation_key",
//     model_name: "ACorrelationPropertyRefCorrelationKey",
//     full_name: "bpmn_20_association_a_correlation_property_ref_correlation_key",
// }

