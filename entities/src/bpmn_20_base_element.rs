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
    #[sea_orm(has_one = "super::bpmn_20_artifact::Entity")]
    Artifact,
    #[sea_orm(has_one = "super::bpmn_20_assignment::Entity")]
    Assignment,
    #[sea_orm(has_one = "super::bpmn_20_auditing::Entity")]
    Auditing,
    #[sea_orm(has_one = "super::bpmn_20_category_value::Entity")]
    CategoryValue,
    #[sea_orm(has_one = "super::bpmn_20_complex_behavior_definition::Entity")]
    ComplexBehaviorDefinition,
    #[sea_orm(has_one = "super::bpmn_20_conversation_association::Entity")]
    ConversationAssociation,
    #[sea_orm(has_one = "super::bpmn_20_conversation_link::Entity")]
    ConversationLink,
    #[sea_orm(has_one = "super::bpmn_20_conversation_node::Entity")]
    ConversationNode,
    #[sea_orm(has_one = "super::bpmn_20_correlation_key::Entity")]
    CorrelationKey,
    #[sea_orm(has_one = "super::bpmn_20_correlation_property_binding::Entity")]
    CorrelationPropertyBinding,
    #[sea_orm(has_one = "super::bpmn_20_correlation_property_retrieval_expression::Entity")]
    CorrelationPropertyRetrievalExpression,
    #[sea_orm(has_one = "super::bpmn_20_correlation_subscription::Entity")]
    CorrelationSubscription,
    #[sea_orm(has_one = "super::bpmn_20_data_association::Entity")]
    DataAssociation,
    #[sea_orm(has_one = "super::bpmn_20_data_state::Entity")]
    DataState,
    #[sea_orm(has_one = "super::bpmn_20_definitions::Entity")]
    Definitions,
    #[sea_orm(has_one = "super::bpmn_20_documentation::Entity")]
    Documentation,
    #[sea_orm(has_one = "super::bpmn_20_expression::Entity")]
    Expression,
    #[sea_orm(has_one = "super::bpmn_20_flow_element::Entity")]
    FlowElement,
    #[sea_orm(has_one = "super::bpmn_20_flow_elements_container::Entity")]
    FlowElementsContainer,
    #[sea_orm(has_one = "super::bpmn_20_input_output_specification::Entity")]
    InputOutputSpecification,
    #[sea_orm(has_one = "super::bpmn_20_input_set::Entity")]
    InputSet,
    #[sea_orm(has_one = "super::bpmn_20_item_aware_element::Entity")]
    ItemAwareElement,
    #[sea_orm(has_one = "super::bpmn_20_lane::Entity")]
    Lane,
    #[sea_orm(has_one = "super::bpmn_20_lane_set::Entity")]
    LaneSet,
    #[sea_orm(has_one = "super::bpmn_20_loop_characteristics::Entity")]
    LoopCharacteristics,
    #[sea_orm(has_one = "super::bpmn_20_message_flow::Entity")]
    MessageFlow,
    #[sea_orm(has_one = "super::bpmn_20_message_flow_association::Entity")]
    MessageFlowAssociation,
    #[sea_orm(has_one = "super::bpmn_20_monitoring::Entity")]
    Monitoring,
    #[sea_orm(has_one = "super::bpmn_20_operation::Entity")]
    Operation,
    #[sea_orm(has_one = "super::bpmn_20_output_set::Entity")]
    OutputSet,
    #[sea_orm(has_one = "super::bpmn_20_participant::Entity")]
    Participant,
    #[sea_orm(has_one = "super::bpmn_20_participant_association::Entity")]
    ParticipantAssociation,
    #[sea_orm(has_one = "super::bpmn_20_relationship::Entity")]
    Relationship,
    #[sea_orm(has_one = "super::bpmn_20_rendering::Entity")]
    Rendering,
    #[sea_orm(has_one = "super::bpmn_20_resource_parameter::Entity")]
    ResourceParameter,
    #[sea_orm(has_one = "super::bpmn_20_resource_role::Entity")]
    ResourceRole,
    #[sea_orm(has_one = "super::bpmn_20_root_element::Entity")]
    RootElement,
}

// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artifact.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_assignment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assignment.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_auditing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Auditing.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_category_value::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CategoryValue.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_complex_behavior_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ComplexBehaviorDefinition.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_conversation_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationAssociation.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_conversation_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationLink.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_conversation_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationNode.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_correlation_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationKey.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_correlation_property_binding::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationPropertyBinding.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_correlation_property_retrieval_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationPropertyRetrievalExpression.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_correlation_subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CorrelationSubscription.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataAssociation.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_data_state::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataState.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_definitions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Definitions.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_documentation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Documentation.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Expression.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_flow_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_flow_elements_container::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElementsContainer.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_input_output_specification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InputOutputSpecification.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_input_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InputSet.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_item_aware_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemAwareElement.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_lane::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lane.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_lane_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LaneSet.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoopCharacteristics.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_message_flow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageFlow.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_message_flow_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageFlowAssociation.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_monitoring::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Monitoring.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_operation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Operation.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_output_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OutputSet.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Participant.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_participant_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ParticipantAssociation.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_relationship::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Relationship.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_rendering::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rendering.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_resource_parameter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceParameter.def()
    }
}
// `Related` trait has to be implemented by hand
impl Related<super::bpmn_20_resource_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceRole.def()
    }
}
// `Related` trait has to be implemented by hand
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

