//! bpmn_20_class_participant

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_participant")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SUPER FIELD : InteractionNode
    pub super_interaction_node: i64,
    /// COMPLEX FIELD : BPMN20-Participant-participantMultiplicity
    pub participant_multiplicity: Option<i64>,
    /// COMPLEX FIELD : BPMN20-Participant-processRef
    pub process_ref: Option<i64>,
    /// SIMPLE FIELD : BPMN20-Participant-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Participant need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE Participant need ONE InteractionNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_interaction_node::Entity",
        from = "Column::SuperInteractionNode",
        to = "super::bpmn_20_interaction_node::Column::Id",
        on_delete = "Cascade"
    )]
    InteractionNode,
}

// SUPER : ONE Participant need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE Participant need ONE InteractionNode
impl Related<super::bpmn_20_interaction_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InteractionNode.def()
    }
}

// ManyToMany : with EndPoint using A_endPointRefs_participant
impl Related<super::bpmn_20_a_end_point_refs_participant::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_end_point_refs_participant::Relation::EndPoint.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_end_point_refs_participant::Relation::Participant
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with Interface using A_interfaceRefs_participant
impl Related<super::bpmn_20_a_interface_refs_participant::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_interface_refs_participant::Relation::Interface.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_interface_refs_participant::Relation::Participant
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with ChoreographyActivity using A_participantRefs_choreographyActivity
impl Related<super::bpmn_20_a_participant_refs_choreography_activity::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_participant_refs_choreography_activity::Relation::ChoreographyActivity.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_participant_refs_choreography_activity::Relation::Participant
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with ConversationNode using A_participantRefs_conversationNode
impl Related<super::bpmn_20_a_participant_refs_conversation_node::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_participant_refs_conversation_node::Relation::ConversationNode.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_participant_refs_conversation_node::Relation::Participant
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with PartnerEntity using A_partnerEntityRef_participantRef
impl Related<super::bpmn_20_a_partner_entity_ref_participant_ref::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_partner_entity_ref_participant_ref::Relation::PartnerEntity.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_partner_entity_ref_participant_ref::Relation::Participant
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with PartnerRole using A_partnerRoleRef_participantRef
impl Related<super::bpmn_20_a_partner_role_ref_participant_ref::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_partner_role_ref_participant_ref::Relation::PartnerRole.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_partner_role_ref_participant_ref::Relation::Participant
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Participant" (bpmn_20_class_participant)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-Participant-name")
    ///   * type : __std::string::String__
    /// 
    /// ## Direct One To One :
    /// * __ParticipantMultiplicity__ (__ParticipantMultiplicityModel__) from A_participantMultiplicity_participant
    ///   * one-to-one link : (0-1) __Participant__ need (1-1) __ParticipantMultiplicity__)
    ///   * callable using find_also_related(__ParticipantMultiplicityModel__) from __Participant__
    ///   * saved in __participant_multiplicity__ field as foreing key
    /// 
    /// ## Relation : One To Many :
    /// * __Collaboration__ (__CollaborationModel__) from A_participants_collaboration
    ///   * one-to-many link : (1-1) __Participant__ need (0-inf) __Collaboration__)
    ///   * callable using find_with_related(__CollaborationModel__) from __Participant__
    ///   * named collaboration in BPMN
    /// * __Process__ (__ProcessModel__) from A_processRef_participant
    ///   * one-to-many link : (0-1) __Participant__ need (0-inf) __Process__)
    ///   * callable using find_with_related(__ProcessModel__) from __Participant__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Participant__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Participant__
    ///   * saved in __super_base_element__ field as foreing key
    /// * __InteractionNode__ (__InteractionNodeModel__)
    ///   * one-to-one link : one __Participant__ need one __InteractionNode__)
    ///   * callable using find_also_related(__InteractionNodeModel__) from __Participant__
    ///   * saved in __super_interaction_node__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Participant" (bpmn_20_class_participant)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-Participant-name")
  * type : __std::string::String__

## Direct One To One :
* __ParticipantMultiplicity__ (__ParticipantMultiplicityModel__) from A_participantMultiplicity_participant
  * one-to-one link : (0-1) __Participant__ need (1-1) __ParticipantMultiplicity__)
  * callable using find_also_related(__ParticipantMultiplicityModel__) from __Participant__
  * saved in __participant_multiplicity__ field as foreing key

## Relation : One To Many :
* __Collaboration__ (__CollaborationModel__) from A_participants_collaboration
  * one-to-many link : (1-1) __Participant__ need (0-inf) __Collaboration__)
  * callable using find_with_related(__CollaborationModel__) from __Participant__
  * named collaboration in BPMN
* __Process__ (__ProcessModel__) from A_processRef_participant
  * one-to-many link : (0-1) __Participant__ need (0-inf) __Process__)
  * callable using find_with_related(__ProcessModel__) from __Participant__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Participant__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Participant__
  * saved in __super_base_element__ field as foreing key
* __InteractionNode__ (__InteractionNodeModel__)
  * one-to-one link : one __Participant__ need one __InteractionNode__)
  * callable using find_also_related(__InteractionNodeModel__) from __Participant__
  * saved in __super_interaction_node__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "Participant",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Participant",
//     is_abstract: false,
//     super_class: [
//         "InteractionNode",
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Participant-endPointRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Participant-endPointRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "endPointRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EndPoint",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_endPointRefs_participant",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Participant-interfaceRefs": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Participant-interfaceRefs",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "interfaceRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Interface",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_interfaceRefs_participant",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Participant-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Participant-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Participant-participantMultiplicity": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Participant-participantMultiplicity",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "participantMultiplicity",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ParticipantMultiplicity",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_participantMultiplicity_participant",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-Participant-processRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "Participant-processRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "processRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Process",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
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
//                     "A_processRef_participant",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Participant",
//     table_name: "bpmn_20_participant",
//     model_name: "Participant",
//     full_name: "bpmn_20_class_participant",
// }

