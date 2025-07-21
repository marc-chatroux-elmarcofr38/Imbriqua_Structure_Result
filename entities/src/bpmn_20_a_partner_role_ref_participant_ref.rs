//! bpmn_20_association_a_partner_role_ref_participant_ref

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_partner_role_ref_participant_ref")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub participant_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub partner_role_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_participant::Entity",
        from = "Column::ParticipantAId",
        to = "super::bpmn_20_participant::Column::Id"
    )]
    Participant,
    #[sea_orm(
        belongs_to = "super::bpmn_20_partner_role::Entity",
        from = "Column::PartnerRoleBId",
        to = "super::bpmn_20_partner_role::Column::Id"
    )]
    PartnerRole,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdLocalReference {
//         object_id: "A_partnerRoleRef_participantRef",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_partnerRoleRef_participantRef",
//     visibility: Private,
//     member_end: (
//         "RefCell of 'BPMN20-A_partnerRoleRef_participantRef-partnerRoleRef' (loaded : true)",
//         "RefCell of 'BPMN20-PartnerRole-participantRef' (loaded : true)",
//     ),
//     owned_end: {
//         "A_partnerRoleRef_participantRef-partnerRoleRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "A_partnerRoleRef_participantRef-partnerRoleRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "partnerRoleRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "PartnerRole",
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
//                 is_derived: true,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_partnerRoleRef_participantRef",
//                 association: Some(
//                     "A_partnerRoleRef_participantRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_partnerRoleRef_participantRef",
//     table_name: "bpmn_20_a_partner_role_ref_participant_ref",
//     model_name: "APartnerRoleRefParticipantRef",
//     full_name: "bpmn_20_association_a_partner_role_ref_participant_ref",
// }

