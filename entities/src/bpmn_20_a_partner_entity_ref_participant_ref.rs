//! bpmn_20_association_a_partner_entity_ref_participant_ref

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_partner_entity_ref_participant_ref")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub participant_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub partner_entity_b_id: i64,
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
        belongs_to = "super::bpmn_20_partner_entity::Entity",
        from = "Column::PartnerEntityBId",
        to = "super::bpmn_20_partner_entity::Column::Id"
    )]
    PartnerEntity,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: "A_partnerEntityRef_participantRef",
//     name: "A_partnerEntityRef_participantRef",
//     visibility: Private,
//     member_end: (
//         "A_partnerEntityRef_participantRef-partnerEntityRef",
//         "PartnerEntity-participantRef",
//     ),
//     owned_end: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "A_partnerEntityRef_participantRef-partnerEntityRef",
//                 name: "partnerEntityRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "PartnerEntity",
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
//                 owning_association: "A_partnerEntityRef_participantRef",
//                 association: Some(
//                     "A_partnerEntityRef_participantRef",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     is_derived: false,
// }

