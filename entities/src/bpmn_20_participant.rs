//! bpmn_20_class_participant

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_participant")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : InteractionNode
    pub super_interaction_node: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : Participant-participantMultiplicity
    pub participant_multiplicity: Option<i64>,
    /// COMPLEX FIELD : Participant-processRef
    pub process_ref: Option<i64>,
    /// SIMPLE FIELD : Participant-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Participant need ONE InteractionNode
    #[sea_orm(
        belongs_to = "super::bpmn_20_interaction_node::Entity",
        from = "Column::SuperInteractionNode",
        to = "super::bpmn_20_interaction_node::Column::Id"
    )]
    InteractionNode,
    // SUPER : ONE Participant need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
}

// SUPER : ONE Participant need ONE InteractionNode
impl Related<super::bpmn_20_interaction_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InteractionNode.def()
    }
}

// SUPER : ONE Participant need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
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

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "Participant",
//     name: "Participant",
//     is_abstract: false,
//     super_class: [
//         "InteractionNode",
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Participant-name",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Participant-interfaceRefs",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Participant-participantMultiplicity",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Participant-endPointRefs",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Participant-processRef",
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
//     ],
//     owned_rule: [],
// }

