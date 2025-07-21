//! bpmn_20_association_a_participant_refs_choreography_activity

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_participant_refs_choreography_activity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub participant_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub choreography_activity_b_id: i64,
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
        belongs_to = "super::bpmn_20_choreography_activity::Entity",
        from = "Column::ChoreographyActivityBId",
        to = "super::bpmn_20_choreography_activity::Column::Id"
    )]
    ChoreographyActivity,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: XMIIdLocalReference {
//         object_id: "A_participantRefs_choreographyActivity",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "A_participantRefs_choreographyActivity",
//     visibility: Private,
//     member_end: (
//         "RefCell of 'BPMN20-ChoreographyActivity-participantRefs' (loaded : true)",
//         "RefCell of 'BPMN20-A_participantRefs_choreographyActivity-choreographyActivity' (loaded : true)",
//     ),
//     owned_end: {
//         "A_participantRefs_choreographyActivity-choreographyActivity": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "A_participantRefs_choreographyActivity-choreographyActivity",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "choreographyActivity",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ChoreographyActivity",
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
//                 owning_association: "A_participantRefs_choreographyActivity",
//                 association: Some(
//                     "A_participantRefs_choreographyActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
//     technical_name: "BPMN20.cmof#A_participantRefs_choreographyActivity",
//     table_name: "bpmn_20_a_participant_refs_choreography_activity",
//     model_name: "AParticipantRefsChoreographyActivity",
//     full_name: "bpmn_20_association_a_participant_refs_choreography_activity",
// }

