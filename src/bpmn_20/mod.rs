//! bpmn_20
/// Link from _packageImport.1 (PackageImport)
use crate::dc;

/// Association : A_errorRefs_operation
mod a_error_refs_operation;
pub use a_error_refs_operation::AErrorRefsOperation;

/// Association : A_inMessageRef_operation
mod a_in_message_ref_operation;
pub use a_in_message_ref_operation::AInMessageRefOperation;

/// Association : A_outMessageRef_operation
mod a_out_message_ref_operation;
pub use a_out_message_ref_operation::AOutMessageRefOperation;

/// Class : Interface
mod interface;
pub use interface::Interface;

/// Class : Operation
mod operation;
pub use operation::Operation;

/// Association : A_operations_interface
mod a_operations_interface;
pub use a_operations_interface::AOperationsInterface;

/// Class : EndPoint
mod end_point;
pub use end_point::EndPoint;

/// Association : A_definitionalCollaborationRef_process
mod a_definitional_collaboration_ref_process;
pub use a_definitional_collaboration_ref_process::ADefinitionalCollaborationRefProcess;

/// Association : A_partitionElement_lane
mod a_partition_element_lane;
pub use a_partition_element_lane::APartitionElementLane;

/// Association : A_flowNodeRefs_lanes
mod a_flow_node_refs_lanes;
pub use a_flow_node_refs_lanes::AFlowNodeRefsLanes;

/// Association : A_partitionElementRef_lane
mod a_partition_element_ref_lane;
pub use a_partition_element_ref_lane::APartitionElementRefLane;

/// Association : A_auditing_process
mod a_auditing_process;
pub use a_auditing_process::AAuditingProcess;

/// Association : A_monitoring_process
mod a_monitoring_process;
pub use a_monitoring_process::AMonitoringProcess;

/// Class : Auditing
mod auditing;
pub use auditing::Auditing;

/// Class : GlobalTask
mod global_task;
pub use global_task::GlobalTask;

/// Class : Monitoring
mod monitoring;
pub use monitoring::Monitoring;

/// Class : Performer
mod performer;
pub use performer::Performer;

/// Class : Process
mod process;
pub use process::Process;

/// Enumeration : ProcessType
mod process_type;
pub use process_type::ProcessType;

/// Association : A_properties_process
mod a_properties_process;
pub use a_properties_process::APropertiesProcess;

/// Class : LaneSet
mod lane_set;
pub use lane_set::LaneSet;

/// Class : Lane
mod lane;
pub use lane::Lane;

/// Association : A_lanes_laneSet
mod a_lanes_lane_set;
pub use a_lanes_lane_set::ALanesLaneSet;

/// Association : A_childLaneSet_parentLane
mod a_child_lane_set_parent_lane;
pub use a_child_lane_set_parent_lane::AChildLaneSetParentLane;

/// Association : A_resources_globalTask
mod a_resources_global_task;
pub use a_resources_global_task::AResourcesGlobalTask;

/// Association : A_supports_process
mod a_supports_process;
pub use a_supports_process::ASupportsProcess;

/// Association : A_resources_process
mod a_resources_process;
pub use a_resources_process::AResourcesProcess;

/// Association : A_artifacts_process
mod a_artifacts_process;
pub use a_artifacts_process::AArtifactsProcess;

/// Association : A_correlationSubscriptions_process
mod a_correlation_subscriptions_process;
pub use a_correlation_subscriptions_process::ACorrelationSubscriptionsProcess;

/// Class : GlobalManualTask
mod global_manual_task;
pub use global_manual_task::GlobalManualTask;

/// Class : ManualTask
mod manual_task;
pub use manual_task::ManualTask;

/// Class : UserTask
mod user_task;
pub use user_task::UserTask;

/// Class : Rendering
mod rendering;
pub use rendering::Rendering;

/// Association : A_renderings_usertask
mod a_renderings_usertask;
pub use a_renderings_usertask::ARenderingsUsertask;

/// Class : HumanPerformer
mod human_performer;
pub use human_performer::HumanPerformer;

/// Class : PotentialOwner
mod potential_owner;
pub use potential_owner::PotentialOwner;

/// Class : GlobalUserTask
mod global_user_task;
pub use global_user_task::GlobalUserTask;

/// Association : A_renderings_globalUserTask
mod a_renderings_global_user_task;
pub use a_renderings_global_user_task::ARenderingsGlobalUserTask;

/// Association : A_activationCondition_complexGateway
mod a_activation_condition_complex_gateway;
pub use a_activation_condition_complex_gateway::AActivationConditionComplexGateway;

/// Class : Gateway
mod gateway;
pub use gateway::Gateway;

/// Enumeration : GatewayDirection
mod gateway_direction;
pub use gateway_direction::GatewayDirection;

/// Association : A_default_inclusiveGateway
mod a_default_inclusive_gateway;
pub use a_default_inclusive_gateway::ADefaultInclusiveGateway;

/// Association : A_default_exclusiveGateway
mod a_default_exclusive_gateway;
pub use a_default_exclusive_gateway::ADefaultExclusiveGateway;

/// Class : EventBasedGateway
mod event_based_gateway;
pub use event_based_gateway::EventBasedGateway;

/// Class : ComplexGateway
mod complex_gateway;
pub use complex_gateway::ComplexGateway;

/// Class : ExclusiveGateway
mod exclusive_gateway;
pub use exclusive_gateway::ExclusiveGateway;

/// Class : InclusiveGateway
mod inclusive_gateway;
pub use inclusive_gateway::InclusiveGateway;

/// Class : ParallelGateway
mod parallel_gateway;
pub use parallel_gateway::ParallelGateway;

/// Enumeration : EventBasedGatewayType
mod event_based_gateway_type;
pub use event_based_gateway_type::EventBasedGatewayType;

/// Association : A_default_complexGateway
mod a_default_complex_gateway;
pub use a_default_complex_gateway::ADefaultComplexGateway;

/// Class : RootElement
mod root_element;
pub use root_element::RootElement;

/// Class : Relationship
mod relationship;
pub use relationship::Relationship;

/// Association : A_valueRef_extensionAttributeValue
mod a_value_ref_extension_attribute_value;
pub use a_value_ref_extension_attribute_value::AValueRefExtensionAttributeValue;

/// Association : A_value_extensionAttributeValue
mod a_value_extension_attribute_value;
pub use a_value_extension_attribute_value::AValueExtensionAttributeValue;

/// Class : BaseElement
mod base_element;
pub use base_element::BaseElement;

/// Class : Extension
mod extension;
pub use extension::Extension;

/// Class : ExtensionDefinition
mod extension_definition;
pub use extension_definition::ExtensionDefinition;

/// Class : ExtensionAttributeDefinition
mod extension_attribute_definition;
pub use extension_attribute_definition::ExtensionAttributeDefinition;

/// Class : ExtensionAttributeValue
mod extension_attribute_value;
pub use extension_attribute_value::ExtensionAttributeValue;

/// Association : A_extensionDefinitions_baseElement
mod a_extension_definitions_base_element;
pub use a_extension_definitions_base_element::AExtensionDefinitionsBaseElement;

/// Association : A_definition_extension
mod a_definition_extension;
pub use a_definition_extension::ADefinitionExtension;

/// Association : A_extensionAttributeDefinitions_extensionDefinition
mod a_extension_attribute_definitions_extension_definition;
pub use a_extension_attribute_definitions_extension_definition::AExtensionAttributeDefinitionsExtensionDefinition;

/// Association : A_extensionValues_baseElement
mod a_extension_values_base_element;
pub use a_extension_values_base_element::AExtensionValuesBaseElement;

/// Enumeration : RelationshipDirection
mod relationship_direction;
pub use relationship_direction::RelationshipDirection;

/// Association : A_extensionAttributeDefinition_extensionAttributeValue
mod a_extension_attribute_definition_extension_attribute_value;
pub use a_extension_attribute_definition_extension_attribute_value::AExtensionAttributeDefinitionExtensionAttributeValue;

/// Class : Documentation
mod documentation;
pub use documentation::Documentation;

/// Association : A_documentation_baseElement
mod a_documentation_base_element;
pub use a_documentation_base_element::ADocumentationBaseElement;

/// Association : A_sources_relationship
mod a_sources_relationship;
pub use a_sources_relationship::ASourcesRelationship;

/// Association : A_targets_relationship
mod a_targets_relationship;
pub use a_targets_relationship::ATargetsRelationship;

/// Association : A_dataInputAssociation_throwEvent
mod a_data_input_association_throw_event;
pub use a_data_input_association_throw_event::ADataInputAssociationThrowEvent;

/// Association : A_dataOutputAssociation_catchEvent
mod a_data_output_association_catch_event;
pub use a_data_output_association_catch_event::ADataOutputAssociationCatchEvent;

/// Class : Event
mod event;
pub use event::Event;

/// Association : A_activityRef_compensateEventDefinition
mod a_activity_ref_compensate_event_definition;
pub use a_activity_ref_compensate_event_definition::AActivityRefCompensateEventDefinition;

/// Association : A_inputSet_throwEvent
mod a_input_set_throw_event;
pub use a_input_set_throw_event::AInputSetThrowEvent;

/// Association : A_structureRef_signal
mod a_structure_ref_signal;
pub use a_structure_ref_signal::AStructureRefSignal;

/// Association : A_messageRef_messageEventDefinition
mod a_message_ref_message_event_definition;
pub use a_message_ref_message_event_definition::AMessageRefMessageEventDefinition;

/// Association : A_outputSet_catchEvent
mod a_output_set_catch_event;
pub use a_output_set_catch_event::AOutputSetCatchEvent;

/// Association : A_structureRef_escalation
mod a_structure_ref_escalation;
pub use a_structure_ref_escalation::AStructureRefEscalation;

/// Class : IntermediateCatchEvent
mod intermediate_catch_event;
pub use intermediate_catch_event::IntermediateCatchEvent;

/// Class : IntermediateThrowEvent
mod intermediate_throw_event;
pub use intermediate_throw_event::IntermediateThrowEvent;

/// Class : EndEvent
mod end_event;
pub use end_event::EndEvent;

/// Class : StartEvent
mod start_event;
pub use start_event::StartEvent;

/// Class : ThrowEvent
mod throw_event;
pub use throw_event::ThrowEvent;

/// Class : CatchEvent
mod catch_event;
pub use catch_event::CatchEvent;

/// Class : BoundaryEvent
mod boundary_event;
pub use boundary_event::BoundaryEvent;

/// Class : EventDefinition
mod event_definition;
pub use event_definition::EventDefinition;

/// Association : A_eventDefinitionRefs_throwEvent
mod a_event_definition_refs_throw_event;
pub use a_event_definition_refs_throw_event::AEventDefinitionRefsThrowEvent;

/// Association : A_eventDefinitionRefs_catchEvent
mod a_event_definition_refs_catch_event;
pub use a_event_definition_refs_catch_event::AEventDefinitionRefsCatchEvent;

/// Class : CancelEventDefinition
mod cancel_event_definition;
pub use cancel_event_definition::CancelEventDefinition;

/// Class : ErrorEventDefinition
mod error_event_definition;
pub use error_event_definition::ErrorEventDefinition;

/// Class : TerminateEventDefinition
mod terminate_event_definition;
pub use terminate_event_definition::TerminateEventDefinition;

/// Association : A_errorRef_errorEventDefinition
mod a_error_ref_error_event_definition;
pub use a_error_ref_error_event_definition::AErrorRefErrorEventDefinition;

/// Class : EscalationEventDefinition
mod escalation_event_definition;
pub use escalation_event_definition::EscalationEventDefinition;

/// Class : Escalation
mod escalation;
pub use escalation::Escalation;

/// Association : A_escalationRef_escalationEventDefinition
mod a_escalation_ref_escalation_event_definition;
pub use a_escalation_ref_escalation_event_definition::AEscalationRefEscalationEventDefinition;

/// Class : CompensateEventDefinition
mod compensate_event_definition;
pub use compensate_event_definition::CompensateEventDefinition;

/// Class : TimerEventDefinition
mod timer_event_definition;
pub use timer_event_definition::TimerEventDefinition;

/// Class : LinkEventDefinition
mod link_event_definition;
pub use link_event_definition::LinkEventDefinition;

/// Class : MessageEventDefinition
mod message_event_definition;
pub use message_event_definition::MessageEventDefinition;

/// Class : ConditionalEventDefinition
mod conditional_event_definition;
pub use conditional_event_definition::ConditionalEventDefinition;

/// Class : SignalEventDefinition
mod signal_event_definition;
pub use signal_event_definition::SignalEventDefinition;

/// Class : Signal
mod signal;
pub use signal::Signal;

/// Association : A_signalRef_signalEventDefinition
mod a_signal_ref_signal_event_definition;
pub use a_signal_ref_signal_event_definition::ASignalRefSignalEventDefinition;

/// Class : ImplicitThrowEvent
mod implicit_throw_event;
pub use implicit_throw_event::ImplicitThrowEvent;

/// Association : A_eventDefinitions_throwEvent
mod a_event_definitions_throw_event;
pub use a_event_definitions_throw_event::AEventDefinitionsThrowEvent;

/// Association : A_eventDefinitions_catchEvent
mod a_event_definitions_catch_event;
pub use a_event_definitions_catch_event::AEventDefinitionsCatchEvent;

/// Association : A_dataInputs_throwEvent
mod a_data_inputs_throw_event;
pub use a_data_inputs_throw_event::ADataInputsThrowEvent;

/// Association : A_dataOutputs_catchEvent
mod a_data_outputs_catch_event;
pub use a_data_outputs_catch_event::ADataOutputsCatchEvent;

/// Association : A_operationRef_messageEventDefinition
mod a_operation_ref_message_event_definition;
pub use a_operation_ref_message_event_definition::AOperationRefMessageEventDefinition;

/// Association : A_condition_conditionalEventDefinition
mod a_condition_conditional_event_definition;
pub use a_condition_conditional_event_definition::AConditionConditionalEventDefinition;

/// Association : A_timeDate_timerEventDefinition
mod a_time_date_timer_event_definition;
pub use a_time_date_timer_event_definition::ATimeDateTimerEventDefinition;

/// Association : A_timeCycle_timerEventDefinition
mod a_time_cycle_timer_event_definition;
pub use a_time_cycle_timer_event_definition::ATimeCycleTimerEventDefinition;

/// Association : A_target_source
mod a_target_source;
pub use a_target_source::ATargetSource;

/// Association : A_properties_event
mod a_properties_event;
pub use a_properties_event::APropertiesEvent;

/// Association : A_timeDuration_timerEventDefinition
mod a_time_duration_timer_event_definition;
pub use a_time_duration_timer_event_definition::ATimeDurationTimerEventDefinition;

/// Association : A_dataState_itemAwareElement
mod a_data_state_item_aware_element;
pub use a_data_state_item_aware_element::ADataStateItemAwareElement;

/// Class : DataState
mod data_state;
pub use data_state::DataState;

/// Class : ItemAwareElement
mod item_aware_element;
pub use item_aware_element::ItemAwareElement;

/// Association : A_operationRef_ioBinding
mod a_operation_ref_io_binding;
pub use a_operation_ref_io_binding::AOperationRefIoBinding;

/// Association : A_sourceRef_dataAssociation
mod a_source_ref_data_association;
pub use a_source_ref_data_association::ASourceRefDataAssociation;

/// Association : A_targetRef_dataAssociation
mod a_target_ref_data_association;
pub use a_target_ref_data_association::ATargetRefDataAssociation;

/// Class : DataAssociation
mod data_association;
pub use data_association::DataAssociation;

/// Association : A_transformation_dataAssociation
mod a_transformation_data_association;
pub use a_transformation_data_association::ATransformationDataAssociation;

/// Class : DataInput
mod data_input;
pub use data_input::DataInput;

/// Class : DataOutput
mod data_output;
pub use data_output::DataOutput;

/// Class : InputSet
mod input_set;
pub use input_set::InputSet;

/// Class : OutputSet
mod output_set;
pub use output_set::OutputSet;

/// Association : A_dataInputRefs_inputSetRefs
mod a_data_input_refs_input_set_refs;
pub use a_data_input_refs_input_set_refs::ADataInputRefsInputSetRefs;

/// Association : A_dataOutputRefs_outputSetRefs
mod a_data_output_refs_output_set_refs;
pub use a_data_output_refs_output_set_refs::ADataOutputRefsOutputSetRefs;

/// Class : Property
mod property;
pub use property::Property;

/// Class : DataInputAssociation
mod data_input_association;
pub use data_input_association::DataInputAssociation;

/// Class : DataOutputAssociation
mod data_output_association;
pub use data_output_association::DataOutputAssociation;

/// Class : InputOutputSpecification
mod input_output_specification;
pub use input_output_specification::InputOutputSpecification;

/// Association : A_inputSets_inputOutputSpecification
mod a_input_sets_input_output_specification;
pub use a_input_sets_input_output_specification::AInputSetsInputOutputSpecification;

/// Association : A_outputSets_inputOutputSpecification
mod a_output_sets_input_output_specification;
pub use a_output_sets_input_output_specification::AOutputSetsInputOutputSpecification;

/// Association : A_dataInputs_inputOutputSpecification
mod a_data_inputs_input_output_specification;
pub use a_data_inputs_input_output_specification::ADataInputsInputOutputSpecification;

/// Association : A_dataOutputs_inputOutputSpecification
mod a_data_outputs_input_output_specification;
pub use a_data_outputs_input_output_specification::ADataOutputsInputOutputSpecification;

/// Class : DataObject
mod data_object;
pub use data_object::DataObject;

/// Association : A_inputSetRefs_outputSetRefs
mod a_input_set_refs_output_set_refs;
pub use a_input_set_refs_output_set_refs::AInputSetRefsOutputSetRefs;

/// Class : InputOutputBinding
mod input_output_binding;
pub use input_output_binding::InputOutputBinding;

/// Association : A_inputDataRef_inputOutputBinding
mod a_input_data_ref_input_output_binding;
pub use a_input_data_ref_input_output_binding::AInputDataRefInputOutputBinding;

/// Association : A_outputDataRef_inputOutputBinding
mod a_output_data_ref_input_output_binding;
pub use a_output_data_ref_input_output_binding::AOutputDataRefInputOutputBinding;

/// Association : A_whileExecutingInputRefs_inputSetWithWhileExecuting
mod a_while_executing_input_refs_input_set_with_while_executing;
pub use a_while_executing_input_refs_input_set_with_while_executing::AWhileExecutingInputRefsInputSetWithWhileExecuting;

/// Association : A_optionalInputRefs_inputSetWithOptional
mod a_optional_input_refs_input_set_with_optional;
pub use a_optional_input_refs_input_set_with_optional::AOptionalInputRefsInputSetWithOptional;

/// Association : A_outputSetWithOptional_optionalOutputRefs
mod a_output_set_with_optional_optional_output_refs;
pub use a_output_set_with_optional_optional_output_refs::AOutputSetWithOptionalOptionalOutputRefs;

/// Association : A_outputSetWithWhileExecuting_whileExecutingOutputRefs
mod a_output_set_with_while_executing_while_executing_output_refs;
pub use a_output_set_with_while_executing_while_executing_output_refs::AOutputSetWithWhileExecutingWhileExecutingOutputRefs;

/// Class : Assignment
mod assignment;
pub use assignment::Assignment;

/// Association : A_assignment_dataAssociation
mod a_assignment_data_association;
pub use a_assignment_data_association::AAssignmentDataAssociation;

/// Class : DataStore
mod data_store;
pub use data_store::DataStore;

/// Class : DataStoreReference
mod data_store_reference;
pub use data_store_reference::DataStoreReference;

/// Association : A_dataStoreRef_dataStoreReference
mod a_data_store_ref_data_store_reference;
pub use a_data_store_ref_data_store_reference::ADataStoreRefDataStoreReference;

/// Association : A_itemSubjectRef_itemAwareElement
mod a_item_subject_ref_item_aware_element;
pub use a_item_subject_ref_item_aware_element::AItemSubjectRefItemAwareElement;

/// Association : A_from_assignment
mod a_from_assignment;
pub use a_from_assignment::AFromAssignment;

/// Association : A_to_assignment
mod a_to_assignment;
pub use a_to_assignment::AToAssignment;

/// Class : DataObjectReference
mod data_object_reference;
pub use data_object_reference::DataObjectReference;

/// Association : A_dataObjectRef_dataObject
mod a_data_object_ref_data_object;
pub use a_data_object_ref_data_object::ADataObjectRefDataObject;

/// Class : ConversationLink
mod conversation_link;
pub use conversation_link::ConversationLink;

/// Class : ConversationAssociation
mod conversation_association;
pub use conversation_association::ConversationAssociation;

/// Association : A_calledCollaborationRef_callConversation
mod a_called_collaboration_ref_call_conversation;
pub use a_called_collaboration_ref_call_conversation::ACalledCollaborationRefCallConversation;

/// Association : A_participantRefs_conversationNode
mod a_participant_refs_conversation_node;
pub use a_participant_refs_conversation_node::AParticipantRefsConversationNode;

/// Association : A_messageFlowRefs_communication
mod a_message_flow_refs_communication;
pub use a_message_flow_refs_communication::AMessageFlowRefsCommunication;

/// Association : A_participantAssociations_callConversation
mod a_participant_associations_call_conversation;
pub use a_participant_associations_call_conversation::AParticipantAssociationsCallConversation;

/// Class : CallConversation
mod call_conversation;
pub use call_conversation::CallConversation;

/// Class : Conversation
mod conversation;
pub use conversation::Conversation;

/// Class : SubConversation
mod sub_conversation;
pub use sub_conversation::SubConversation;

/// Class : ConversationNode
mod conversation_node;
pub use conversation_node::ConversationNode;

/// Class : GlobalConversation
mod global_conversation;
pub use global_conversation::GlobalConversation;

/// Association : A_correlationKeys_collaboration
mod a_correlation_keys_collaboration;
pub use a_correlation_keys_collaboration::ACorrelationKeysCollaboration;

/// Association : A_correlationKeys_conversationNode
mod a_correlation_keys_conversation_node;
pub use a_correlation_keys_conversation_node::ACorrelationKeysConversationNode;

/// Association : A_innerConversationNodeRef_conversationAssociation
mod a_inner_conversation_node_ref_conversation_association;
pub use a_inner_conversation_node_ref_conversation_association::AInnerConversationNodeRefConversationAssociation;

/// Association : A_outerConversationNodeRef_conversationAssociation
mod a_outer_conversation_node_ref_conversation_association;
pub use a_outer_conversation_node_ref_conversation_association::AOuterConversationNodeRefConversationAssociation;

/// Association : A_conversationNodes_subConversation
mod a_conversation_nodes_sub_conversation;
pub use a_conversation_nodes_sub_conversation::AConversationNodesSubConversation;

/// Association : A_sourceRef_outgoingConversationLinks
mod a_source_ref_outgoing_conversation_links;
pub use a_source_ref_outgoing_conversation_links::ASourceRefOutgoingConversationLinks;

/// Association : A_targetRef_incomingConversationLinks
mod a_target_ref_incoming_conversation_links;
pub use a_target_ref_incoming_conversation_links::ATargetRefIncomingConversationLinks;

/// Class : PartnerEntity
mod partner_entity;
pub use partner_entity::PartnerEntity;

/// Class : PartnerRole
mod partner_role;
pub use partner_role::PartnerRole;

/// Association : A_correlationPropertyRef_correlationKey
mod a_correlation_property_ref_correlation_key;
pub use a_correlation_property_ref_correlation_key::ACorrelationPropertyRefCorrelationKey;

/// Class : CorrelationProperty
mod correlation_property;
pub use correlation_property::CorrelationProperty;

/// Enumeration : ItemKind
mod item_kind;
pub use item_kind::ItemKind;

/// Association : A_supportedInterfaceRefs_callableElements
mod a_supported_interface_refs_callable_elements;
pub use a_supported_interface_refs_callable_elements::ASupportedInterfaceRefsCallableElements;

/// Association : A_ioBinding_callableElement
mod a_io_binding_callable_element;
pub use a_io_binding_callable_element::AIoBindingCallableElement;

/// Association : A_ioSpecification_callableElement
mod a_io_specification_callable_element;
pub use a_io_specification_callable_element::AIoSpecificationCallableElement;

/// Association : A_messagePath_correlationset
mod a_message_path_correlationset;
pub use a_message_path_correlationset::AMessagePathCorrelationset;

/// Association : A_structureRef_error
mod a_structure_ref_error;
pub use a_structure_ref_error::AStructureRefError;

/// Class : Error
mod error;
pub use error::Error;

/// Association : A_evaluatesToTypeRef_formalExpression
mod a_evaluates_to_type_ref_formal_expression;
pub use a_evaluates_to_type_ref_formal_expression::AEvaluatesToTypeRefFormalExpression;

/// Class : CorrelationKey
mod correlation_key;
pub use correlation_key::CorrelationKey;

/// Class : Expression
mod expression;
pub use expression::Expression;

/// Class : FormalExpression
mod formal_expression;
pub use formal_expression::FormalExpression;

/// Class : Message
mod message;
pub use message::Message;

/// Class : ItemDefinition
mod item_definition;
pub use item_definition::ItemDefinition;

/// Association : A_conditionExpression_sequenceFlow
mod a_condition_expression_sequence_flow;
pub use a_condition_expression_sequence_flow::AConditionExpressionSequenceFlow;

/// Class : FlowElement
mod flow_element;
pub use flow_element::FlowElement;

/// Class : SequenceFlow
mod sequence_flow;
pub use sequence_flow::SequenceFlow;

/// Class : FlowElementsContainer
mod flow_elements_container;
pub use flow_elements_container::FlowElementsContainer;

/// Association : A_flowElements_container
mod a_flow_elements_container;
pub use a_flow_elements_container::AFlowElementsContainer;

/// Class : CallableElement
mod callable_element;
pub use callable_element::CallableElement;

/// Class : FlowNode
mod flow_node;
pub use flow_node::FlowNode;

/// Association : A_sourceRef_outgoing_flow
mod a_source_ref_outgoing_flow;
pub use a_source_ref_outgoing_flow::ASourceRefOutgoingFlow;

/// Association : A_targetRef_incoming_flow
mod a_target_ref_incoming_flow;
pub use a_target_ref_incoming_flow::ATargetRefIncomingFlow;

/// Class : CorrelationPropertyRetrievalExpression
mod correlation_property_retrieval_expression;
pub use correlation_property_retrieval_expression::CorrelationPropertyRetrievalExpression;

/// Class : CorrelationPropertyBinding
mod correlation_property_binding;
pub use correlation_property_binding::CorrelationPropertyBinding;

/// Association : A_correlationPropertyRetrievalExpression_correlationproperty
mod a_correlation_property_retrieval_expression_correlationproperty;
pub use a_correlation_property_retrieval_expression_correlationproperty::ACorrelationPropertyRetrievalExpressionCorrelationproperty;

/// Association : A_messageRef_correlationPropertyRetrievalExpression
mod a_message_ref_correlation_property_retrieval_expression;
pub use a_message_ref_correlation_property_retrieval_expression::AMessageRefCorrelationPropertyRetrievalExpression;

/// Association : A_dataPath_correlationPropertyBinding
mod a_data_path_correlation_property_binding;
pub use a_data_path_correlation_property_binding::ADataPathCorrelationPropertyBinding;

/// Association : A_correlationPropertyRef_correlationPropertyBinding
mod a_correlation_property_ref_correlation_property_binding;
pub use a_correlation_property_ref_correlation_property_binding::ACorrelationPropertyRefCorrelationPropertyBinding;

/// Class : Resource
mod resource;
pub use resource::Resource;

/// Class : ResourceParameter
mod resource_parameter;
pub use resource_parameter::ResourceParameter;

/// Association : A_resourceParameters_resource
mod a_resource_parameters_resource;
pub use a_resource_parameters_resource::AResourceParametersResource;

/// Association : A_import_itemDefinition
mod a_import_item_definition;
pub use a_import_item_definition::AImportItemDefinition;

/// Class : CorrelationSubscription
mod correlation_subscription;
pub use correlation_subscription::CorrelationSubscription;

/// Association : A_correlationKeyRef_correlationSubscription
mod a_correlation_key_ref_correlation_subscription;
pub use a_correlation_key_ref_correlation_subscription::ACorrelationKeyRefCorrelationSubscription;

/// Association : A_correlationPropertyBinding_correlationSubscription
mod a_correlation_property_binding_correlation_subscription;
pub use a_correlation_property_binding_correlation_subscription::ACorrelationPropertyBindingCorrelationSubscription;

/// Association : A_auditing_flowElement
mod a_auditing_flow_element;
pub use a_auditing_flow_element::AAuditingFlowElement;

/// Association : A_monitoring_flowElement
mod a_monitoring_flow_element;
pub use a_monitoring_flow_element::AMonitoringFlowElement;

/// Association : A_type_correlationProperty
mod a_type_correlation_property;
pub use a_type_correlation_property::ATypeCorrelationProperty;

/// Association : A_type_resourceParameter
mod a_type_resource_parameter;
pub use a_type_resource_parameter::ATypeResourceParameter;

/// Association : A_itemRef_message
mod a_item_ref_message;
pub use a_item_ref_message::AItemRefMessage;

/// Association : A_laneSets_flowElementsContainer
mod a_lane_sets_flow_elements_container;
pub use a_lane_sets_flow_elements_container::ALaneSetsFlowElementsContainer;

/// Class : MessageFlow
mod message_flow;
pub use message_flow::MessageFlow;

/// Class : MessageFlowAssociation
mod message_flow_association;
pub use message_flow_association::MessageFlowAssociation;

/// Class : InteractionNode
mod interaction_node;
pub use interaction_node::InteractionNode;

/// Class : Participant
mod participant;
pub use participant::Participant;

/// Class : ParticipantAssociation
mod participant_association;
pub use participant_association::ParticipantAssociation;

/// Class : ParticipantMultiplicity
mod participant_multiplicity;
pub use participant_multiplicity::ParticipantMultiplicity;

/// Association : A_messageFlowAssociations_collaboration
mod a_message_flow_associations_collaboration;
pub use a_message_flow_associations_collaboration::AMessageFlowAssociationsCollaboration;

/// Association : A_participantAssociations_collaboration
mod a_participant_associations_collaboration;
pub use a_participant_associations_collaboration::AParticipantAssociationsCollaboration;

/// Association : A_artifacts_collaboration
mod a_artifacts_collaboration;
pub use a_artifacts_collaboration::AArtifactsCollaboration;

/// Class : Collaboration
mod collaboration;
pub use collaboration::Collaboration;

/// Association : A_conversationAssociations_converstaionAssociations
mod a_conversation_associations_converstaion_associations;
pub use a_conversation_associations_converstaion_associations::AConversationAssociationsConverstaionAssociations;

/// Association : A_choreographyRef_collaboration
mod a_choreography_ref_collaboration;
pub use a_choreography_ref_collaboration::AChoreographyRefCollaboration;

/// Association : A_innerParticipantRef_participantAssociation
mod a_inner_participant_ref_participant_association;
pub use a_inner_participant_ref_participant_association::AInnerParticipantRefParticipantAssociation;

/// Association : A_outerParticipantRef_participantAssociation
mod a_outer_participant_ref_participant_association;
pub use a_outer_participant_ref_participant_association::AOuterParticipantRefParticipantAssociation;

/// Association : A_endPointRefs_participant
mod a_end_point_refs_participant;
pub use a_end_point_refs_participant::AEndPointRefsParticipant;

/// Association : A_participantMultiplicity_participant
mod a_participant_multiplicity_participant;
pub use a_participant_multiplicity_participant::AParticipantMultiplicityParticipant;

/// Association : A_interfaceRefs_participant
mod a_interface_refs_participant;
pub use a_interface_refs_participant::AInterfaceRefsParticipant;

/// Association : A_partnerEntityRef_participantRef
mod a_partner_entity_ref_participant_ref;
pub use a_partner_entity_ref_participant_ref::APartnerEntityRefParticipantRef;

/// Association : A_partnerRoleRef_participantRef
mod a_partner_role_ref_participant_ref;
pub use a_partner_role_ref_participant_ref::APartnerRoleRefParticipantRef;

/// Association : A_processRef_participant
mod a_process_ref_participant;
pub use a_process_ref_participant::AProcessRefParticipant;

/// Association : A_innerMessageFlowRef_messageFlowAssociation
mod a_inner_message_flow_ref_message_flow_association;
pub use a_inner_message_flow_ref_message_flow_association::AInnerMessageFlowRefMessageFlowAssociation;

/// Association : A_outerMessageFlowRef_messageFlowAssociation
mod a_outer_message_flow_ref_message_flow_association;
pub use a_outer_message_flow_ref_message_flow_association::AOuterMessageFlowRefMessageFlowAssociation;

/// Association : A_targetRef_messageFlow
mod a_target_ref_message_flow;
pub use a_target_ref_message_flow::ATargetRefMessageFlow;

/// Association : A_messageRef_messageFlow
mod a_message_ref_message_flow;
pub use a_message_ref_message_flow::AMessageRefMessageFlow;

/// Association : A_sourceRef_messageFlow
mod a_source_ref_message_flow;
pub use a_source_ref_message_flow::ASourceRefMessageFlow;

/// Association : A_participants_collaboration
mod a_participants_collaboration;
pub use a_participants_collaboration::AParticipantsCollaboration;

/// Association : A_messageFlows_collaboration
mod a_message_flows_collaboration;
pub use a_message_flows_collaboration::AMessageFlowsCollaboration;

/// Association : A_conversations_collaboration
mod a_conversations_collaboration;
pub use a_conversations_collaboration::AConversationsCollaboration;

/// Association : A_conversationLinks_collaboration
mod a_conversation_links_collaboration;
pub use a_conversation_links_collaboration::AConversationLinksCollaboration;

/// Association : A_participantAssociations_callChoreographyActivity
mod a_participant_associations_call_choreography_activity;
pub use a_participant_associations_call_choreography_activity::AParticipantAssociationsCallChoreographyActivity;

/// Class : ChoreographyActivity
mod choreography_activity;
pub use choreography_activity::ChoreographyActivity;

/// Class : CallChoreography
mod call_choreography;
pub use call_choreography::CallChoreography;

/// Class : SubChoreography
mod sub_choreography;
pub use sub_choreography::SubChoreography;

/// Class : ChoreographyTask
mod choreography_task;
pub use choreography_task::ChoreographyTask;

/// Association : A_calledChoreographyRef_callChoreographyActivity
mod a_called_choreography_ref_call_choreography_activity;
pub use a_called_choreography_ref_call_choreography_activity::ACalledChoreographyRefCallChoreographyActivity;

/// Association : A_messageFlowRef_choreographyTask
mod a_message_flow_ref_choreography_task;
pub use a_message_flow_ref_choreography_task::AMessageFlowRefChoreographyTask;

/// Association : A_initiatingParticipantRef_choreographyActivity
mod a_initiating_participant_ref_choreography_activity;
pub use a_initiating_participant_ref_choreography_activity::AInitiatingParticipantRefChoreographyActivity;

/// Association : A_participantRefs_choreographyActivity
mod a_participant_refs_choreography_activity;
pub use a_participant_refs_choreography_activity::AParticipantRefsChoreographyActivity;

/// Association : A_artifacts_subChoreography
mod a_artifacts_sub_choreography;
pub use a_artifacts_sub_choreography::AArtifactsSubChoreography;

/// Enumeration : ChoreographyLoopType
mod choreography_loop_type;
pub use choreography_loop_type::ChoreographyLoopType;

/// Association : A_correlationKeys_choreographyActivity
mod a_correlation_keys_choreography_activity;
pub use a_correlation_keys_choreography_activity::ACorrelationKeysChoreographyActivity;

/// Association : A_initiatingParticipantRef_globalChoreographyTask
mod a_initiating_participant_ref_global_choreography_task;
pub use a_initiating_participant_ref_global_choreography_task::AInitiatingParticipantRefGlobalChoreographyTask;

/// Class : Choreography
mod choreography;
pub use choreography::Choreography;

/// Class : GlobalChoreographyTask
mod global_choreography_task;
pub use global_choreography_task::GlobalChoreographyTask;

/// Association : A_sourceRef_outgoing_association
mod a_source_ref_outgoing_association;
pub use a_source_ref_outgoing_association::ASourceRefOutgoingAssociation;

/// Association : A_targetRef_incoming_association
mod a_target_ref_incoming_association;
pub use a_target_ref_incoming_association::ATargetRefIncomingAssociation;

/// Class : TextAnnotation
mod text_annotation;
pub use text_annotation::TextAnnotation;

/// Class : Group
mod group;
pub use group::Group;

/// Class : Association
mod association;
pub use association::Association;

/// Class : Category
mod category;
pub use category::Category;

/// Class : Artifact
mod artifact;
pub use artifact::Artifact;

/// Association : A_categoryValueRef_categoryValueRef
mod a_category_value_ref_category_value_ref;
pub use a_category_value_ref_category_value_ref::ACategoryValueRefCategoryValueRef;

/// Enumeration : AssociationDirection
mod association_direction;
pub use association_direction::AssociationDirection;

/// Class : CategoryValue
mod category_value;
pub use category_value::CategoryValue;

/// Association : A_categoryValue_category
mod a_category_value_category;
pub use a_category_value_category::ACategoryValueCategory;

/// Association : A_categorizedFlowElements_categoryValueRef
mod a_categorized_flow_elements_category_value_ref;
pub use a_categorized_flow_elements_category_value_ref::ACategorizedFlowElementsCategoryValueRef;

/// Association : A_event_complexBehaviorDefinition
mod a_event_complex_behavior_definition;
pub use a_event_complex_behavior_definition::AEventComplexBehaviorDefinition;

/// Association : A_expression_resourceAssignmentExpression
mod a_expression_resource_assignment_expression;
pub use a_expression_resource_assignment_expression::AExpressionResourceAssignmentExpression;

/// Association : A_expression_resourceParameterBinding
mod a_expression_resource_parameter_binding;
pub use a_expression_resource_parameter_binding::AExpressionResourceParameterBinding;

/// Association : A_noneBehaviorEventRef_multiInstanceLoopCharacteristics
mod a_none_behavior_event_ref_multi_instance_loop_characteristics;
pub use a_none_behavior_event_ref_multi_instance_loop_characteristics::ANoneBehaviorEventRefMultiInstanceLoopCharacteristics;

/// Association : A_oneBehaviorEventRef_multiInstanceLoopCharacteristics
mod a_one_behavior_event_ref_multi_instance_loop_characteristics;
pub use a_one_behavior_event_ref_multi_instance_loop_characteristics::AOneBehaviorEventRefMultiInstanceLoopCharacteristics;

/// Association : A_completionCondition_multiInstanceLoopCharacteristics
mod a_completion_condition_multi_instance_loop_characteristics;
pub use a_completion_condition_multi_instance_loop_characteristics::ACompletionConditionMultiInstanceLoopCharacteristics;

/// Association : A_condition_complexBehaviorDefinition
mod a_condition_complex_behavior_definition;
pub use a_condition_complex_behavior_definition::AConditionComplexBehaviorDefinition;

/// Association : A_resourceRef_activityResource
mod a_resource_ref_activity_resource;
pub use a_resource_ref_activity_resource::AResourceRefActivityResource;

/// Association : A_messageRef_sendTask
mod a_message_ref_send_task;
pub use a_message_ref_send_task::AMessageRefSendTask;

/// Association : A_messageRef_receiveTask
mod a_message_ref_receive_task;
pub use a_message_ref_receive_task::AMessageRefReceiveTask;

/// Class : Activity
mod activity;
pub use activity::Activity;

/// Association : A_operationRef_serviceTask
mod a_operation_ref_service_task;
pub use a_operation_ref_service_task::AOperationRefServiceTask;

/// Association : A_calledElementRef_callActivity
mod a_called_element_ref_call_activity;
pub use a_called_element_ref_call_activity::ACalledElementRefCallActivity;

/// Association : A_loopCardinality_multiInstanceLoopCharacteristics
mod a_loop_cardinality_multi_instance_loop_characteristics;
pub use a_loop_cardinality_multi_instance_loop_characteristics::ALoopCardinalityMultiInstanceLoopCharacteristics;

/// Association : A_properties_activity
mod a_properties_activity;
pub use a_properties_activity::APropertiesActivity;

/// Association : A_resources_activity
mod a_resources_activity;
pub use a_resources_activity::AResourcesActivity;

/// Association : A_loopCondition_standardLoopCharacteristics
mod a_loop_condition_standard_loop_characteristics;
pub use a_loop_condition_standard_loop_characteristics::ALoopConditionStandardLoopCharacteristics;

/// Association : A_loopCharacteristics_activity
mod a_loop_characteristics_activity;
pub use a_loop_characteristics_activity::ALoopCharacteristicsActivity;

/// Association : A_ioSpecification_activity
mod a_io_specification_activity;
pub use a_io_specification_activity::AIoSpecificationActivity;

/// Association : A_completionCondition_adHocSubProcess
mod a_completion_condition_ad_hoc_sub_process;
pub use a_completion_condition_ad_hoc_sub_process::ACompletionConditionAdHocSubProcess;

/// Class : ServiceTask
mod service_task;
pub use service_task::ServiceTask;

/// Class : SubProcess
mod sub_process;
pub use sub_process::SubProcess;

/// Association : A_operationRef_receiveTask
mod a_operation_ref_receive_task;
pub use a_operation_ref_receive_task::AOperationRefReceiveTask;

/// Association : A_operationRef_sendTask
mod a_operation_ref_send_task;
pub use a_operation_ref_send_task::AOperationRefSendTask;

/// Class : LoopCharacteristics
mod loop_characteristics;
pub use loop_characteristics::LoopCharacteristics;

/// Enumeration : MultiInstanceBehavior
mod multi_instance_behavior;
pub use multi_instance_behavior::MultiInstanceBehavior;

/// Class : MultiInstanceLoopCharacteristics
mod multi_instance_loop_characteristics;
pub use multi_instance_loop_characteristics::MultiInstanceLoopCharacteristics;

/// Class : StandardLoopCharacteristics
mod standard_loop_characteristics;
pub use standard_loop_characteristics::StandardLoopCharacteristics;

/// Class : CallActivity
mod call_activity;
pub use call_activity::CallActivity;

/// Class : Task
mod task;
pub use task::Task;

/// Class : SendTask
mod send_task;
pub use send_task::SendTask;

/// Class : ReceiveTask
mod receive_task;
pub use receive_task::ReceiveTask;

/// Class : ScriptTask
mod script_task;
pub use script_task::ScriptTask;

/// Class : BusinessRuleTask
mod business_rule_task;
pub use business_rule_task::BusinessRuleTask;

/// Class : AdHocSubProcess
mod ad_hoc_sub_process;
pub use ad_hoc_sub_process::AdHocSubProcess;

/// Enumeration : AdHocOrdering
mod ad_hoc_ordering;
pub use ad_hoc_ordering::AdHocOrdering;

/// Class : Transaction
mod transaction;
pub use transaction::Transaction;

/// Class : GlobalScriptTask
mod global_script_task;
pub use global_script_task::GlobalScriptTask;

/// Class : GlobalBusinessRuleTask
mod global_business_rule_task;
pub use global_business_rule_task::GlobalBusinessRuleTask;

/// Class : ComplexBehaviorDefinition
mod complex_behavior_definition;
pub use complex_behavior_definition::ComplexBehaviorDefinition;

/// Association : A_complexBehaviorDefinition_multiInstanceLoopCharacteristics
mod a_complex_behavior_definition_multi_instance_loop_characteristics;
pub use a_complex_behavior_definition_multi_instance_loop_characteristics::AComplexBehaviorDefinitionMultiInstanceLoopCharacteristics;

/// Class : ResourceRole
mod resource_role;
pub use resource_role::ResourceRole;

/// Class : ResourceParameterBinding
mod resource_parameter_binding;
pub use resource_parameter_binding::ResourceParameterBinding;

/// Class : ResourceAssignmentExpression
mod resource_assignment_expression;
pub use resource_assignment_expression::ResourceAssignmentExpression;

/// Association : A_resourceParameterBindings_activityResource
mod a_resource_parameter_bindings_activity_resource;
pub use a_resource_parameter_bindings_activity_resource::AResourceParameterBindingsActivityResource;

/// Association : A_resourceAssignmentExpression_activityResource
mod a_resource_assignment_expression_activity_resource;
pub use a_resource_assignment_expression_activity_resource::AResourceAssignmentExpressionActivityResource;

/// Association : A_loopMaximum_standardLoopCharacteristics
mod a_loop_maximum_standard_loop_characteristics;
pub use a_loop_maximum_standard_loop_characteristics::ALoopMaximumStandardLoopCharacteristics;

/// Association : A_dataInputAssociations_activity
mod a_data_input_associations_activity;
pub use a_data_input_associations_activity::ADataInputAssociationsActivity;

/// Association : A_dataOutputAssociations_activity
mod a_data_output_associations_activity;
pub use a_data_output_associations_activity::ADataOutputAssociationsActivity;

/// Association : A_parameterRef_resourceParameterBinding
mod a_parameter_ref_resource_parameter_binding;
pub use a_parameter_ref_resource_parameter_binding::AParameterRefResourceParameterBinding;

/// Association : A_loopDataInputRef_multiInstanceLoopCharacteristics
mod a_loop_data_input_ref_multi_instance_loop_characteristics;
pub use a_loop_data_input_ref_multi_instance_loop_characteristics::ALoopDataInputRefMultiInstanceLoopCharacteristics;

/// Association : A_loopDataOutputRef_multiInstanceLoopCharacteristics
mod a_loop_data_output_ref_multi_instance_loop_characteristics;
pub use a_loop_data_output_ref_multi_instance_loop_characteristics::ALoopDataOutputRefMultiInstanceLoopCharacteristics;

/// Association : A_inputDataItem_multiInstanceLoopCharacteristics
mod a_input_data_item_multi_instance_loop_characteristics;
pub use a_input_data_item_multi_instance_loop_characteristics::AInputDataItemMultiInstanceLoopCharacteristics;

/// Association : A_outputDataItem_multiInstanceLoopCharacteristics
mod a_output_data_item_multi_instance_loop_characteristics;
pub use a_output_data_item_multi_instance_loop_characteristics::AOutputDataItemMultiInstanceLoopCharacteristics;

/// Association : A_boundaryEventRefs_attachedToRef
mod a_boundary_event_refs_attached_to_ref;
pub use a_boundary_event_refs_attached_to_ref::ABoundaryEventRefsAttachedToRef;

/// Association : A_default_activity
mod a_default_activity;
pub use a_default_activity::ADefaultActivity;

/// Association : A_artifacts_subProcess
mod a_artifacts_sub_process;
pub use a_artifacts_sub_process::AArtifactsSubProcess;

/// Class : Import
mod import;
pub use import::Import;

/// Class : Definitions
mod definitions;
pub use definitions::Definitions;

/// Association : A_diagrams_definitions
mod a_diagrams_definitions;
pub use a_diagrams_definitions::ADiagramsDefinitions;

/// Association : A_imports_definition
mod a_imports_definition;
pub use a_imports_definition::AImportsDefinition;

/// Association : A_extensions_definitions
mod a_extensions_definitions;
pub use a_extensions_definitions::AExtensionsDefinitions;

/// Association : A_relationships_definition
mod a_relationships_definition;
pub use a_relationships_definition::ARelationshipsDefinition;

/// Association : A_rootElements_definition
mod a_root_elements_definition;
pub use a_root_elements_definition::ARootElementsDefinition;
