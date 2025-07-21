//! bpmn_20_association_a_interface_refs_participant

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_interface_refs_participant")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub interface_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub participant_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_interface::Entity",
        from = "Column::InterfaceAId",
        to = "super::bpmn_20_interface::Column::Id"
    )]
    Interface,
    #[sea_orm(
        belongs_to = "super::bpmn_20_participant::Entity",
        from = "Column::ParticipantBId",
        to = "super::bpmn_20_participant::Column::Id"
    )]
    Participant,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdLocalReference {
//         object_id: "A_interfaceRefs_participant",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_interfaceRefs_participant",
//     visibility: Private,
//     member_end: (
//         "Participant-interfaceRefs",
//         "A_interfaceRefs_participant-participant",
//     ),
//     owned_end: {
//         "A_interfaceRefs_participant-participant": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "A_interfaceRefs_participant-participant",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "participant",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
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
//                 owning_association: "A_interfaceRefs_participant",
//                 association: Some(
//                     "A_interfaceRefs_participant",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_interfaceRefs_participant",
//     table_name: "bpmn_20_a_interface_refs_participant",
//     model_name: "AInterfaceRefsParticipant",
//     full_name: "bpmn_20_association_a_interface_refs_participant",
// }

