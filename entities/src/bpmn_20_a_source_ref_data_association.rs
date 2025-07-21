//! bpmn_20_association_a_source_ref_data_association

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_source_ref_data_association")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub item_aware_element_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub data_association_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_item_aware_element::Entity",
        from = "Column::ItemAwareElementAId",
        to = "super::bpmn_20_item_aware_element::Column::Id"
    )]
    ItemAwareElement,
    #[sea_orm(
        belongs_to = "super::bpmn_20_data_association::Entity",
        from = "Column::DataAssociationBId",
        to = "super::bpmn_20_data_association::Column::Id"
    )]
    DataAssociation,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdLocalReference {
//         object_id: "A_sourceRef_dataAssociation",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_sourceRef_dataAssociation",
//     visibility: Private,
//     member_end: (
//         "DataAssociation-sourceRef",
//         "A_sourceRef_dataAssociation-dataAssociation",
//     ),
//     owned_end: {
//         "A_sourceRef_dataAssociation-dataAssociation": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "A_sourceRef_dataAssociation-dataAssociation",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "dataAssociation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "DataAssociation",
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
//                 owning_association: "A_sourceRef_dataAssociation",
//                 association: Some(
//                     "A_sourceRef_dataAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_sourceRef_dataAssociation",
//     table_name: "bpmn_20_a_source_ref_data_association",
//     model_name: "ASourceRefDataAssociation",
//     full_name: "bpmn_20_association_a_source_ref_data_association",
// }

