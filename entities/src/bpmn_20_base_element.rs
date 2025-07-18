//! bpmn_20_class_base_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_base_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SIMPLE FIELD : BPMN20-BaseElement-id
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

// ManyToMany : with ExtensionDefinition using A_extensionDefinitions_baseElement
impl Related<super::bpmn_20_a_extension_definitions_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_extension_definitions_base_element::Relation::ExtensionDefinition.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_extension_definitions_base_element::Relation::BaseElement
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "BaseElement" (bpmn_20_class_base_element)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __bpmn_id__ (xmi_id : "BPMN20-BaseElement-id")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// 
    /// ## Reverse One To One :
    /// * __Lane__ (__LaneModel__) from A_partitionElement_lane
    ///   * one-to-one link : (0-1) __Lane__ need (0-1) __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Lane__
    ///   * saved in __partition_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __Artifact__ (__ArtifactModel__)
    ///   * one-to-one link (reverse) : one __Artifact__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Artifact__
    ///   * saved in __super_base_element__ field as foreing key in __ArtifactModel__
    /// * __Assignment__ (__AssignmentModel__)
    ///   * one-to-one link (reverse) : one __Assignment__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Assignment__
    ///   * saved in __super_base_element__ field as foreing key in __AssignmentModel__
    /// * __Auditing__ (__AuditingModel__)
    ///   * one-to-one link (reverse) : one __Auditing__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Auditing__
    ///   * saved in __super_base_element__ field as foreing key in __AuditingModel__
    /// * __CategoryValue__ (__CategoryValueModel__)
    ///   * one-to-one link (reverse) : one __CategoryValue__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CategoryValue__
    ///   * saved in __super_base_element__ field as foreing key in __CategoryValueModel__
    /// * __ComplexBehaviorDefinition__ (__ComplexBehaviorDefinitionModel__)
    ///   * one-to-one link (reverse) : one __ComplexBehaviorDefinition__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ComplexBehaviorDefinition__
    ///   * saved in __super_base_element__ field as foreing key in __ComplexBehaviorDefinitionModel__
    /// * __ConversationAssociation__ (__ConversationAssociationModel__)
    ///   * one-to-one link (reverse) : one __ConversationAssociation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ConversationAssociation__
    ///   * saved in __super_base_element__ field as foreing key in __ConversationAssociationModel__
    /// * __ConversationLink__ (__ConversationLinkModel__)
    ///   * one-to-one link (reverse) : one __ConversationLink__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ConversationLink__
    ///   * saved in __super_base_element__ field as foreing key in __ConversationLinkModel__
    /// * __ConversationNode__ (__ConversationNodeModel__)
    ///   * one-to-one link (reverse) : one __ConversationNode__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ConversationNode__
    ///   * saved in __super_base_element__ field as foreing key in __ConversationNodeModel__
    /// * __CorrelationKey__ (__CorrelationKeyModel__)
    ///   * one-to-one link (reverse) : one __CorrelationKey__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CorrelationKey__
    ///   * saved in __super_base_element__ field as foreing key in __CorrelationKeyModel__
    /// * __CorrelationPropertyBinding__ (__CorrelationPropertyBindingModel__)
    ///   * one-to-one link (reverse) : one __CorrelationPropertyBinding__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CorrelationPropertyBinding__
    ///   * saved in __super_base_element__ field as foreing key in __CorrelationPropertyBindingModel__
    /// * __CorrelationPropertyRetrievalExpression__ (__CorrelationPropertyRetrievalExpressionModel__)
    ///   * one-to-one link (reverse) : one __CorrelationPropertyRetrievalExpression__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CorrelationPropertyRetrievalExpression__
    ///   * saved in __super_base_element__ field as foreing key in __CorrelationPropertyRetrievalExpressionModel__
    /// * __CorrelationSubscription__ (__CorrelationSubscriptionModel__)
    ///   * one-to-one link (reverse) : one __CorrelationSubscription__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __CorrelationSubscription__
    ///   * saved in __super_base_element__ field as foreing key in __CorrelationSubscriptionModel__
    /// * __DataAssociation__ (__DataAssociationModel__)
    ///   * one-to-one link (reverse) : one __DataAssociation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __DataAssociation__
    ///   * saved in __super_base_element__ field as foreing key in __DataAssociationModel__
    /// * __DataState__ (__DataStateModel__)
    ///   * one-to-one link (reverse) : one __DataState__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __DataState__
    ///   * saved in __super_base_element__ field as foreing key in __DataStateModel__
    /// * __Definitions__ (__DefinitionsModel__)
    ///   * one-to-one link (reverse) : one __Definitions__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Definitions__
    ///   * saved in __super_base_element__ field as foreing key in __DefinitionsModel__
    /// * __Documentation__ (__DocumentationModel__)
    ///   * one-to-one link (reverse) : one __Documentation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Documentation__
    ///   * saved in __super_base_element__ field as foreing key in __DocumentationModel__
    /// * __Expression__ (__ExpressionModel__)
    ///   * one-to-one link (reverse) : one __Expression__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Expression__
    ///   * saved in __super_base_element__ field as foreing key in __ExpressionModel__
    /// * __FlowElement__ (__FlowElementModel__)
    ///   * one-to-one link (reverse) : one __FlowElement__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __FlowElement__
    ///   * saved in __super_base_element__ field as foreing key in __FlowElementModel__
    /// * __FlowElementsContainer__ (__FlowElementsContainerModel__)
    ///   * one-to-one link (reverse) : one __FlowElementsContainer__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __FlowElementsContainer__
    ///   * saved in __super_base_element__ field as foreing key in __FlowElementsContainerModel__
    /// * __InputOutputSpecification__ (__InputOutputSpecificationModel__)
    ///   * one-to-one link (reverse) : one __InputOutputSpecification__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __InputOutputSpecification__
    ///   * saved in __super_base_element__ field as foreing key in __InputOutputSpecificationModel__
    /// * __InputSet__ (__InputSetModel__)
    ///   * one-to-one link (reverse) : one __InputSet__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __InputSet__
    ///   * saved in __super_base_element__ field as foreing key in __InputSetModel__
    /// * __ItemAwareElement__ (__ItemAwareElementModel__)
    ///   * one-to-one link (reverse) : one __ItemAwareElement__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ItemAwareElement__
    ///   * saved in __super_base_element__ field as foreing key in __ItemAwareElementModel__
    /// * __Lane__ (__LaneModel__)
    ///   * one-to-one link (reverse) : one __Lane__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Lane__
    ///   * saved in __super_base_element__ field as foreing key in __LaneModel__
    /// * __LaneSet__ (__LaneSetModel__)
    ///   * one-to-one link (reverse) : one __LaneSet__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __LaneSet__
    ///   * saved in __super_base_element__ field as foreing key in __LaneSetModel__
    /// * __LoopCharacteristics__ (__LoopCharacteristicsModel__)
    ///   * one-to-one link (reverse) : one __LoopCharacteristics__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __LoopCharacteristics__
    ///   * saved in __super_base_element__ field as foreing key in __LoopCharacteristicsModel__
    /// * __MessageFlow__ (__MessageFlowModel__)
    ///   * one-to-one link (reverse) : one __MessageFlow__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __MessageFlow__
    ///   * saved in __super_base_element__ field as foreing key in __MessageFlowModel__
    /// * __MessageFlowAssociation__ (__MessageFlowAssociationModel__)
    ///   * one-to-one link (reverse) : one __MessageFlowAssociation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __MessageFlowAssociation__
    ///   * saved in __super_base_element__ field as foreing key in __MessageFlowAssociationModel__
    /// * __Monitoring__ (__MonitoringModel__)
    ///   * one-to-one link (reverse) : one __Monitoring__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Monitoring__
    ///   * saved in __super_base_element__ field as foreing key in __MonitoringModel__
    /// * __Operation__ (__OperationModel__)
    ///   * one-to-one link (reverse) : one __Operation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Operation__
    ///   * saved in __super_base_element__ field as foreing key in __OperationModel__
    /// * __OutputSet__ (__OutputSetModel__)
    ///   * one-to-one link (reverse) : one __OutputSet__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __OutputSet__
    ///   * saved in __super_base_element__ field as foreing key in __OutputSetModel__
    /// * __Participant__ (__ParticipantModel__)
    ///   * one-to-one link (reverse) : one __Participant__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Participant__
    ///   * saved in __super_base_element__ field as foreing key in __ParticipantModel__
    /// * __ParticipantAssociation__ (__ParticipantAssociationModel__)
    ///   * one-to-one link (reverse) : one __ParticipantAssociation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ParticipantAssociation__
    ///   * saved in __super_base_element__ field as foreing key in __ParticipantAssociationModel__
    /// * __Relationship__ (__RelationshipModel__)
    ///   * one-to-one link (reverse) : one __Relationship__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Relationship__
    ///   * saved in __super_base_element__ field as foreing key in __RelationshipModel__
    /// * __Rendering__ (__RenderingModel__)
    ///   * one-to-one link (reverse) : one __Rendering__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Rendering__
    ///   * saved in __super_base_element__ field as foreing key in __RenderingModel__
    /// * __ResourceParameter__ (__ResourceParameterModel__)
    ///   * one-to-one link (reverse) : one __ResourceParameter__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ResourceParameter__
    ///   * saved in __super_base_element__ field as foreing key in __ResourceParameterModel__
    /// * __ResourceRole__ (__ResourceRoleModel__)
    ///   * one-to-one link (reverse) : one __ResourceRole__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ResourceRole__
    ///   * saved in __super_base_element__ field as foreing key in __ResourceRoleModel__
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link (reverse) : one __RootElement__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __RootElement__
    ///   * saved in __super_base_element__ field as foreing key in __RootElementModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "BaseElement" (bpmn_20_class_base_element)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __bpmn_id__ (xmi_id : "BPMN20-BaseElement-id")
  * type : __std::string::String__



## Reverse One To One :
* __Lane__ (__LaneModel__) from A_partitionElement_lane
  * one-to-one link : (0-1) __Lane__ need (0-1) __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Lane__
  * saved in __partition_element__ field as foreing key

## Reverse Super :
* __Artifact__ (__ArtifactModel__)
  * one-to-one link (reverse) : one __Artifact__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Artifact__
  * saved in __super_base_element__ field as foreing key in __ArtifactModel__
* __Assignment__ (__AssignmentModel__)
  * one-to-one link (reverse) : one __Assignment__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Assignment__
  * saved in __super_base_element__ field as foreing key in __AssignmentModel__
* __Auditing__ (__AuditingModel__)
  * one-to-one link (reverse) : one __Auditing__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Auditing__
  * saved in __super_base_element__ field as foreing key in __AuditingModel__
* __CategoryValue__ (__CategoryValueModel__)
  * one-to-one link (reverse) : one __CategoryValue__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CategoryValue__
  * saved in __super_base_element__ field as foreing key in __CategoryValueModel__
* __ComplexBehaviorDefinition__ (__ComplexBehaviorDefinitionModel__)
  * one-to-one link (reverse) : one __ComplexBehaviorDefinition__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ComplexBehaviorDefinition__
  * saved in __super_base_element__ field as foreing key in __ComplexBehaviorDefinitionModel__
* __ConversationAssociation__ (__ConversationAssociationModel__)
  * one-to-one link (reverse) : one __ConversationAssociation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ConversationAssociation__
  * saved in __super_base_element__ field as foreing key in __ConversationAssociationModel__
* __ConversationLink__ (__ConversationLinkModel__)
  * one-to-one link (reverse) : one __ConversationLink__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ConversationLink__
  * saved in __super_base_element__ field as foreing key in __ConversationLinkModel__
* __ConversationNode__ (__ConversationNodeModel__)
  * one-to-one link (reverse) : one __ConversationNode__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ConversationNode__
  * saved in __super_base_element__ field as foreing key in __ConversationNodeModel__
* __CorrelationKey__ (__CorrelationKeyModel__)
  * one-to-one link (reverse) : one __CorrelationKey__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CorrelationKey__
  * saved in __super_base_element__ field as foreing key in __CorrelationKeyModel__
* __CorrelationPropertyBinding__ (__CorrelationPropertyBindingModel__)
  * one-to-one link (reverse) : one __CorrelationPropertyBinding__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CorrelationPropertyBinding__
  * saved in __super_base_element__ field as foreing key in __CorrelationPropertyBindingModel__
* __CorrelationPropertyRetrievalExpression__ (__CorrelationPropertyRetrievalExpressionModel__)
  * one-to-one link (reverse) : one __CorrelationPropertyRetrievalExpression__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CorrelationPropertyRetrievalExpression__
  * saved in __super_base_element__ field as foreing key in __CorrelationPropertyRetrievalExpressionModel__
* __CorrelationSubscription__ (__CorrelationSubscriptionModel__)
  * one-to-one link (reverse) : one __CorrelationSubscription__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __CorrelationSubscription__
  * saved in __super_base_element__ field as foreing key in __CorrelationSubscriptionModel__
* __DataAssociation__ (__DataAssociationModel__)
  * one-to-one link (reverse) : one __DataAssociation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __DataAssociation__
  * saved in __super_base_element__ field as foreing key in __DataAssociationModel__
* __DataState__ (__DataStateModel__)
  * one-to-one link (reverse) : one __DataState__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __DataState__
  * saved in __super_base_element__ field as foreing key in __DataStateModel__
* __Definitions__ (__DefinitionsModel__)
  * one-to-one link (reverse) : one __Definitions__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Definitions__
  * saved in __super_base_element__ field as foreing key in __DefinitionsModel__
* __Documentation__ (__DocumentationModel__)
  * one-to-one link (reverse) : one __Documentation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Documentation__
  * saved in __super_base_element__ field as foreing key in __DocumentationModel__
* __Expression__ (__ExpressionModel__)
  * one-to-one link (reverse) : one __Expression__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Expression__
  * saved in __super_base_element__ field as foreing key in __ExpressionModel__
* __FlowElement__ (__FlowElementModel__)
  * one-to-one link (reverse) : one __FlowElement__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __FlowElement__
  * saved in __super_base_element__ field as foreing key in __FlowElementModel__
* __FlowElementsContainer__ (__FlowElementsContainerModel__)
  * one-to-one link (reverse) : one __FlowElementsContainer__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __FlowElementsContainer__
  * saved in __super_base_element__ field as foreing key in __FlowElementsContainerModel__
* __InputOutputSpecification__ (__InputOutputSpecificationModel__)
  * one-to-one link (reverse) : one __InputOutputSpecification__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __InputOutputSpecification__
  * saved in __super_base_element__ field as foreing key in __InputOutputSpecificationModel__
* __InputSet__ (__InputSetModel__)
  * one-to-one link (reverse) : one __InputSet__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __InputSet__
  * saved in __super_base_element__ field as foreing key in __InputSetModel__
* __ItemAwareElement__ (__ItemAwareElementModel__)
  * one-to-one link (reverse) : one __ItemAwareElement__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ItemAwareElement__
  * saved in __super_base_element__ field as foreing key in __ItemAwareElementModel__
* __Lane__ (__LaneModel__)
  * one-to-one link (reverse) : one __Lane__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Lane__
  * saved in __super_base_element__ field as foreing key in __LaneModel__
* __LaneSet__ (__LaneSetModel__)
  * one-to-one link (reverse) : one __LaneSet__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __LaneSet__
  * saved in __super_base_element__ field as foreing key in __LaneSetModel__
* __LoopCharacteristics__ (__LoopCharacteristicsModel__)
  * one-to-one link (reverse) : one __LoopCharacteristics__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __LoopCharacteristics__
  * saved in __super_base_element__ field as foreing key in __LoopCharacteristicsModel__
* __MessageFlow__ (__MessageFlowModel__)
  * one-to-one link (reverse) : one __MessageFlow__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __MessageFlow__
  * saved in __super_base_element__ field as foreing key in __MessageFlowModel__
* __MessageFlowAssociation__ (__MessageFlowAssociationModel__)
  * one-to-one link (reverse) : one __MessageFlowAssociation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __MessageFlowAssociation__
  * saved in __super_base_element__ field as foreing key in __MessageFlowAssociationModel__
* __Monitoring__ (__MonitoringModel__)
  * one-to-one link (reverse) : one __Monitoring__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Monitoring__
  * saved in __super_base_element__ field as foreing key in __MonitoringModel__
* __Operation__ (__OperationModel__)
  * one-to-one link (reverse) : one __Operation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Operation__
  * saved in __super_base_element__ field as foreing key in __OperationModel__
* __OutputSet__ (__OutputSetModel__)
  * one-to-one link (reverse) : one __OutputSet__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __OutputSet__
  * saved in __super_base_element__ field as foreing key in __OutputSetModel__
* __Participant__ (__ParticipantModel__)
  * one-to-one link (reverse) : one __Participant__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Participant__
  * saved in __super_base_element__ field as foreing key in __ParticipantModel__
* __ParticipantAssociation__ (__ParticipantAssociationModel__)
  * one-to-one link (reverse) : one __ParticipantAssociation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ParticipantAssociation__
  * saved in __super_base_element__ field as foreing key in __ParticipantAssociationModel__
* __Relationship__ (__RelationshipModel__)
  * one-to-one link (reverse) : one __Relationship__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Relationship__
  * saved in __super_base_element__ field as foreing key in __RelationshipModel__
* __Rendering__ (__RenderingModel__)
  * one-to-one link (reverse) : one __Rendering__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Rendering__
  * saved in __super_base_element__ field as foreing key in __RenderingModel__
* __ResourceParameter__ (__ResourceParameterModel__)
  * one-to-one link (reverse) : one __ResourceParameter__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ResourceParameter__
  * saved in __super_base_element__ field as foreing key in __ResourceParameterModel__
* __ResourceRole__ (__ResourceRoleModel__)
  * one-to-one link (reverse) : one __ResourceRole__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ResourceRole__
  * saved in __super_base_element__ field as foreing key in __ResourceRoleModel__
* __RootElement__ (__RootElementModel__)
  * one-to-one link (reverse) : one __RootElement__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __RootElement__
  * saved in __super_base_element__ field as foreing key in __RootElementModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "BaseElement",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "BaseElement",
//     is_abstract: true,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-BaseElement-documentation": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BaseElement-documentation",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//         "-BaseElement-extensionDefinitions": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BaseElement-extensionDefinitions",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//         "-BaseElement-extensionValues": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BaseElement-extensionValues",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//         "-BaseElement-id": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "BaseElement-id",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#BaseElement",
//     table_name: "bpmn_20_base_element",
//     model_name: "BaseElement",
//     full_name: "bpmn_20_class_base_element",
// }

