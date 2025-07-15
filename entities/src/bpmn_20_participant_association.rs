//! bpmn_20_class_participant_association

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_participant_association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : ParticipantAssociation-innerParticipantRef
    pub inner_participant_ref: i64,
    /// COMPLEX FIELD : ParticipantAssociation-outerParticipantRef
    pub outer_participant_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ParticipantAssociation need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE ParticipantAssociation need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ParticipantAssociation" (bpmn_20_class_participant_association)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Participant__ (__ParticipantModel__) from A_innerParticipantRef_participantAssociation
    ///   * one-to-many link : (1-1) __ParticipantAssociation__ need (0-inf) __Participant__)
    ///   * callable using find_with_related(__ParticipantModel__) from __ParticipantAssociation__
    /// * __Participant__ (__ParticipantModel__) from A_outerParticipantRef_participantAssociation
    ///   * one-to-many link : (1-1) __ParticipantAssociation__ need (0-inf) __Participant__)
    ///   * callable using find_with_related(__ParticipantModel__) from __ParticipantAssociation__
    /// * __CallChoreography__ (__CallChoreographyModel__) from A_participantAssociations_callChoreographyActivity
    ///   * one-to-many link : (0-1) __ParticipantAssociation__ need (0-inf) __CallChoreography__)
    ///   * callable using find_with_related(__CallChoreographyModel__) from __ParticipantAssociation__
    ///   * named call_choreography_activity in BPMN
    /// * __CallConversation__ (__CallConversationModel__) from A_participantAssociations_callConversation
    ///   * one-to-many link : (0-1) __ParticipantAssociation__ need (0-inf) __CallConversation__)
    ///   * callable using find_with_related(__CallConversationModel__) from __ParticipantAssociation__
    ///   * named call_conversation in BPMN
    /// * __Collaboration__ (__CollaborationModel__) from A_participantAssociations_collaboration
    ///   * one-to-many link : (0-1) __ParticipantAssociation__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __ParticipantAssociation__
    ///   * named collaboration in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __ParticipantAssociation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ParticipantAssociation__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ParticipantAssociation" (bpmn_20_class_participant_association)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __Participant__ (__ParticipantModel__) from A_innerParticipantRef_participantAssociation
  * one-to-many link : (1-1) __ParticipantAssociation__ need (0-inf) __Participant__)
  * callable using find_with_related(__ParticipantModel__) from __ParticipantAssociation__
* __Participant__ (__ParticipantModel__) from A_outerParticipantRef_participantAssociation
  * one-to-many link : (1-1) __ParticipantAssociation__ need (0-inf) __Participant__)
  * callable using find_with_related(__ParticipantModel__) from __ParticipantAssociation__
* __CallChoreography__ (__CallChoreographyModel__) from A_participantAssociations_callChoreographyActivity
  * one-to-many link : (0-1) __ParticipantAssociation__ need (0-inf) __CallChoreography__)
  * callable using find_with_related(__CallChoreographyModel__) from __ParticipantAssociation__
  * named call_choreography_activity in BPMN
* __CallConversation__ (__CallConversationModel__) from A_participantAssociations_callConversation
  * one-to-many link : (0-1) __ParticipantAssociation__ need (0-inf) __CallConversation__)
  * callable using find_with_related(__CallConversationModel__) from __ParticipantAssociation__
  * named call_conversation in BPMN
* __Collaboration__ (__CollaborationModel__) from A_participantAssociations_collaboration
  * one-to-many link : (0-1) __ParticipantAssociation__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __ParticipantAssociation__
  * named collaboration in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __ParticipantAssociation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ParticipantAssociation__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ParticipantAssociation",
//     name: "ParticipantAssociation",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ParticipantAssociation-innerParticipantRef",
//                 name: "innerParticipantRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_innerParticipantRef_participantAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ParticipantAssociation-outerParticipantRef",
//                 name: "outerParticipantRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_outerParticipantRef_participantAssociation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

