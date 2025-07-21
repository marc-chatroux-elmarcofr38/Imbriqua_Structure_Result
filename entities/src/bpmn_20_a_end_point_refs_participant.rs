//! bpmn_20_association_a_end_point_refs_participant

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_end_point_refs_participant")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub end_point_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub participant_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bpmn_20_end_point::Entity",
        from = "Column::EndPointAId",
        to = "super::bpmn_20_end_point::Column::Id"
    )]
    EndPoint,
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
//         object_id: "A_endPointRefs_participant",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_endPointRefs_participant",
//     visibility: Private,
//     member_end: (
//         "Participant-endPointRefs",
//         "A_endPointRefs_participant-participant",
//     ),
//     owned_end: {
//         "A_endPointRefs_participant-participant": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "A_endPointRefs_participant-participant",
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
//                 owning_association: "A_endPointRefs_participant",
//                 association: Some(
//                     "A_endPointRefs_participant",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_endPointRefs_participant",
//     table_name: "bpmn_20_a_end_point_refs_participant",
//     model_name: "AEndPointRefsParticipant",
//     full_name: "bpmn_20_association_a_end_point_refs_participant",
// }

