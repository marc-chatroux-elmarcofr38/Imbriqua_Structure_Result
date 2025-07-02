//! bpmn_20_class_base_element

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_base_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SIMPLE FIELD : BaseElement-id
    pub bpmn_id: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Artifact need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_artifact::Entity")]
    Artifact,
    // SUPER : ONE Assignment need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_assignment::Entity")]
    Assignment,
    // SUPER : ONE Auditing need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_auditing::Entity")]
    Auditing,
    // SUPER : ONE CategoryValue need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_category_value::Entity")]
    CategoryValue,
    // SUPER : ONE ComplexBehaviorDefinition need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_complex_behavior_definition::Entity")]
    ComplexBehaviorDefinition,
    // SUPER : ONE ConversationAssociation need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_conversation_association::Entity")]
    ConversationAssociation,
    // SUPER : ONE ConversationLink need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_conversation_link::Entity")]
    ConversationLink,
    // SUPER : ONE ConversationNode need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_conversation_node::Entity")]
    ConversationNode,
    // SUPER : ONE CorrelationKey need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_correlation_key::Entity")]
    CorrelationKey,
    // SUPER : ONE CorrelationPropertyBinding need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_correlation_property_binding::Entity")]
    CorrelationPropertyBinding,
    // SUPER : ONE CorrelationPropertyRetrievalExpression need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_correlation_property_retrieval_expression::Entity")]
    CorrelationPropertyRetrievalExpression,
    // SUPER : ONE CorrelationSubscription need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_correlation_subscription::Entity")]
    CorrelationSubscription,
    // SUPER : ONE DataAssociation need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_data_association::Entity")]
    DataAssociation,
    // SUPER : ONE DataState need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_data_state::Entity")]
    DataState,
    // SUPER : ONE Definitions need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_definitions::Entity")]
    Definitions,
    // SUPER : ONE Documentation need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_documentation::Entity")]
    Documentation,
    // SUPER : ONE Expression need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_expression::Entity")]
    Expression,
    // SUPER : ONE FlowElement need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_flow_element::Entity")]
    FlowElement,
    // SUPER : ONE FlowElementsContainer need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_flow_elements_container::Entity")]
    FlowElementsContainer,
    // SUPER : ONE InputOutputSpecification need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_input_output_specification::Entity")]
    InputOutputSpecification,
    // SUPER : ONE InputSet need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_input_set::Entity")]
    InputSet,
    // SUPER : ONE ItemAwareElement need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_item_aware_element::Entity")]
    ItemAwareElement,
    // SUPER : ONE Lane need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_lane::Entity")]
    Lane,
    // SUPER : ONE LaneSet need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_lane_set::Entity")]
    LaneSet,
    // SUPER : ONE LoopCharacteristics need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_loop_characteristics::Entity")]
    LoopCharacteristics,
    // SUPER : ONE MessageFlow need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_message_flow::Entity")]
    MessageFlow,
    // SUPER : ONE MessageFlowAssociation need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_message_flow_association::Entity")]
    MessageFlowAssociation,
    // SUPER : ONE Monitoring need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_monitoring::Entity")]
    Monitoring,
    // SUPER : ONE Operation need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_operation::Entity")]
    Operation,
    // SUPER : ONE OutputSet need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_output_set::Entity")]
    OutputSet,
    // SUPER : ONE Participant need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_participant::Entity")]
    Participant,
    // SUPER : ONE ParticipantAssociation need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_participant_association::Entity")]
    ParticipantAssociation,
    // SUPER : ONE Relationship need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_relationship::Entity")]
    Relationship,
    // SUPER : ONE Rendering need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_rendering::Entity")]
    Rendering,
    // SUPER : ONE ResourceParameter need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_resource_parameter::Entity")]
    ResourceParameter,
    // SUPER : ONE ResourceRole need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_resource_role::Entity")]
    ResourceRole,
    // SUPER : ONE RootElement need ONE BaseElement
    #[sea_orm(has_one = "super::bpmn_20_root_element::Entity")]
    RootElement,
}

// SUPER : ONE Artifact need ONE BaseElement
impl Related<super::bpmn_20_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artifact.def()
    }
}

// SUPER : ONE Assignment need ONE BaseElement
impl Related<super::bpmn_20_assignment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assignment.def()
    }
}

// SUPER : ONE Auditing need ONE BaseElement
impl Related<super::bpmn_20_auditing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Auditing.def()
    }
}

// SUPER : ONE CategoryValue need ONE BaseElement
impl Related<super::bpmn_20_category_value::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CategoryValue.def()
    }
}

// SUPER : ONE ComplexBehaviorDefinition need ONE BaseElement
impl Related<super::bpmn_20_complex_behavior_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ComplexBehaviorDefinition.def()
    }
}

// SUPER : ONE ConversationAssociation need ONE BaseElement
impl Related<super::bpmn_20_conversation_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationAssociation.def()
    }
}

// SUPER : ONE ConversationLink need ONE BaseElement
impl Related<super::bpmn_20_conversation_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationLink.def()
    }
}

// SUPER : ONE ConversationNode need ONE BaseElement
impl Related<super::bpmn_20_conversation_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationNode.def()
    }
}

// SUPER : ONE CorrelationKey need ONE BaseElement
impl Related<super::bpmn_20_correlation_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationKey.def()
    }
}

// SUPER : ONE CorrelationPropertyBinding need ONE BaseElement
impl Related<super::bpmn_20_correlation_property_binding::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationPropertyBinding.def()
    }
}

// SUPER : ONE CorrelationPropertyRetrievalExpression need ONE BaseElement
impl Related<super::bpmn_20_correlation_property_retrieval_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationPropertyRetrievalExpression.def()
    }
}

// SUPER : ONE CorrelationSubscription need ONE BaseElement
impl Related<super::bpmn_20_correlation_subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationSubscription.def()
    }
}

// SUPER : ONE DataAssociation need ONE BaseElement
impl Related<super::bpmn_20_data_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataAssociation.def()
    }
}

// SUPER : ONE DataState need ONE BaseElement
impl Related<super::bpmn_20_data_state::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataState.def()
    }
}

// SUPER : ONE Definitions need ONE BaseElement
impl Related<super::bpmn_20_definitions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Definitions.def()
    }
}

// SUPER : ONE Documentation need ONE BaseElement
impl Related<super::bpmn_20_documentation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Documentation.def()
    }
}

// SUPER : ONE Expression need ONE BaseElement
impl Related<super::bpmn_20_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Expression.def()
    }
}

// SUPER : ONE FlowElement need ONE BaseElement
impl Related<super::bpmn_20_flow_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElement.def()
    }
}

// SUPER : ONE FlowElementsContainer need ONE BaseElement
impl Related<super::bpmn_20_flow_elements_container::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElementsContainer.def()
    }
}

// SUPER : ONE InputOutputSpecification need ONE BaseElement
impl Related<super::bpmn_20_input_output_specification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InputOutputSpecification.def()
    }
}

// SUPER : ONE InputSet need ONE BaseElement
impl Related<super::bpmn_20_input_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InputSet.def()
    }
}

// SUPER : ONE ItemAwareElement need ONE BaseElement
impl Related<super::bpmn_20_item_aware_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemAwareElement.def()
    }
}

// SUPER : ONE Lane need ONE BaseElement
impl Related<super::bpmn_20_lane::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lane.def()
    }
}

// SUPER : ONE LaneSet need ONE BaseElement
impl Related<super::bpmn_20_lane_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LaneSet.def()
    }
}

// SUPER : ONE LoopCharacteristics need ONE BaseElement
impl Related<super::bpmn_20_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoopCharacteristics.def()
    }
}

// SUPER : ONE MessageFlow need ONE BaseElement
impl Related<super::bpmn_20_message_flow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageFlow.def()
    }
}

// SUPER : ONE MessageFlowAssociation need ONE BaseElement
impl Related<super::bpmn_20_message_flow_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageFlowAssociation.def()
    }
}

// SUPER : ONE Monitoring need ONE BaseElement
impl Related<super::bpmn_20_monitoring::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Monitoring.def()
    }
}

// SUPER : ONE Operation need ONE BaseElement
impl Related<super::bpmn_20_operation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Operation.def()
    }
}

// SUPER : ONE OutputSet need ONE BaseElement
impl Related<super::bpmn_20_output_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OutputSet.def()
    }
}

// SUPER : ONE Participant need ONE BaseElement
impl Related<super::bpmn_20_participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Participant.def()
    }
}

// SUPER : ONE ParticipantAssociation need ONE BaseElement
impl Related<super::bpmn_20_participant_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ParticipantAssociation.def()
    }
}

// SUPER : ONE Relationship need ONE BaseElement
impl Related<super::bpmn_20_relationship::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Relationship.def()
    }
}

// SUPER : ONE Rendering need ONE BaseElement
impl Related<super::bpmn_20_rendering::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rendering.def()
    }
}

// SUPER : ONE ResourceParameter need ONE BaseElement
impl Related<super::bpmn_20_resource_parameter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceParameter.def()
    }
}

// SUPER : ONE ResourceRole need ONE BaseElement
impl Related<super::bpmn_20_resource_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceRole.def()
    }
}

// SUPER : ONE RootElement need ONE BaseElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "BaseElement",
//     name: "BaseElement",
//     is_abstract: true,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "BaseElement-id",
//                 name: "id",
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
//                 xmi_id: "BaseElement-extensionDefinitions",
//                 name: "extensionDefinitions",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ExtensionDefinition",
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
//                     "A_extensionDefinitions_baseElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "BaseElement-extensionValues",
//                 name: "extensionValues",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ExtensionAttributeValue",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_extensionValues_baseElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "BaseElement-documentation",
//                 name: "documentation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Documentation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
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
//                     "A_documentation_baseElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

