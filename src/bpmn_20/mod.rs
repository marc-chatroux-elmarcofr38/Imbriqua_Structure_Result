//! bpmn_20

use derive_builder::Builder;
/// Link from _packageImport.1 (PackageImport)
use crate::dc;

/// Association : A_errorRefs_operation
pub mod A_errorRefs_operation;

/// Association : A_inMessageRef_operation
pub mod A_inMessageRef_operation;

/// Association : A_outMessageRef_operation
pub mod A_outMessageRef_operation;

/// Class : Interface
pub mod Interface;

/// Class : Operation
pub mod Operation;

/// Association : A_operations_interface
pub mod A_operations_interface;

/// Class : EndPoint
pub mod EndPoint;

/// Association : A_definitionalCollaborationRef_process
pub mod A_definitionalCollaborationRef_process;

/// Association : A_partitionElement_lane
pub mod A_partitionElement_lane;

/// Association : A_flowNodeRefs_lanes
pub mod A_flowNodeRefs_lanes;

/// Association : A_partitionElementRef_lane
pub mod A_partitionElementRef_lane;

/// Association : A_auditing_process
pub mod A_auditing_process;

/// Association : A_monitoring_process
pub mod A_monitoring_process;

/// Class : Auditing
pub mod Auditing;

/// Class : GlobalTask
pub mod GlobalTask;

/// Class : Monitoring
pub mod Monitoring;

/// Class : Performer
pub mod Performer;

/// Class : Process
pub mod Process;

/// Enumeration : ProcessType
pub mod ProcessType;

/// Association : A_properties_process
pub mod A_properties_process;

/// Class : LaneSet
pub mod LaneSet;

/// Class : Lane
pub mod Lane;

/// Association : A_lanes_laneSet
pub mod A_lanes_laneSet;

/// Association : A_childLaneSet_parentLane
pub mod A_childLaneSet_parentLane;

/// Association : A_resources_globalTask
pub mod A_resources_globalTask;

/// Association : A_supports_process
pub mod A_supports_process;

/// Association : A_resources_process
pub mod A_resources_process;

/// Association : A_artifacts_process
pub mod A_artifacts_process;

/// Association : A_correlationSubscriptions_process
pub mod A_correlationSubscriptions_process;

/// Class : GlobalManualTask
pub mod GlobalManualTask;

/// Class : ManualTask
pub mod ManualTask;

/// Class : UserTask
pub mod UserTask;

/// Class : Rendering
pub mod Rendering;

/// Association : A_renderings_usertask
pub mod A_renderings_usertask;

/// Class : HumanPerformer
pub mod HumanPerformer;

/// Class : PotentialOwner
pub mod PotentialOwner;

/// Class : GlobalUserTask
pub mod GlobalUserTask;

/// Association : A_renderings_globalUserTask
pub mod A_renderings_globalUserTask;

/// Association : A_activationCondition_complexGateway
pub mod A_activationCondition_complexGateway;

/// Class : Gateway
pub mod Gateway;

/// Enumeration : GatewayDirection
pub mod GatewayDirection;

/// Association : A_default_inclusiveGateway
pub mod A_default_inclusiveGateway;

/// Association : A_default_exclusiveGateway
pub mod A_default_exclusiveGateway;

/// Class : EventBasedGateway
pub mod EventBasedGateway;

/// Class : ComplexGateway
pub mod ComplexGateway;

/// Class : ExclusiveGateway
pub mod ExclusiveGateway;

/// Class : InclusiveGateway
pub mod InclusiveGateway;

/// Class : ParallelGateway
pub mod ParallelGateway;

/// Enumeration : EventBasedGatewayType
pub mod EventBasedGatewayType;

/// Association : A_default_complexGateway
pub mod A_default_complexGateway;

/// Class : RootElement
pub mod RootElement;

/// Class : Relationship
pub mod Relationship;

/// Association : A_valueRef_extensionAttributeValue
pub mod A_valueRef_extensionAttributeValue;

/// Association : A_value_extensionAttributeValue
pub mod A_value_extensionAttributeValue;

/// Class : BaseElement
pub mod BaseElement;

/// Class : Extension
pub mod Extension;

/// Class : ExtensionDefinition
pub mod ExtensionDefinition;

/// Class : ExtensionAttributeDefinition
pub mod ExtensionAttributeDefinition;

/// Class : ExtensionAttributeValue
pub mod ExtensionAttributeValue;

/// Association : A_extensionDefinitions_baseElement
pub mod A_extensionDefinitions_baseElement;

/// Association : A_definition_extension
pub mod A_definition_extension;

/// Association : A_extensionAttributeDefinitions_extensionDefinition
pub mod A_extensionAttributeDefinitions_extensionDefinition;

/// Association : A_extensionValues_baseElement
pub mod A_extensionValues_baseElement;

/// Enumeration : RelationshipDirection
pub mod RelationshipDirection;

/// Association : A_extensionAttributeDefinition_extensionAttributeValue
pub mod A_extensionAttributeDefinition_extensionAttributeValue;

/// Class : Documentation
pub mod Documentation;

/// Association : A_documentation_baseElement
pub mod A_documentation_baseElement;

/// Association : A_sources_relationship
pub mod A_sources_relationship;

/// Association : A_targets_relationship
pub mod A_targets_relationship;

/// Association : A_dataInputAssociation_throwEvent
pub mod A_dataInputAssociation_throwEvent;

/// Association : A_dataOutputAssociation_catchEvent
pub mod A_dataOutputAssociation_catchEvent;

/// Class : Event
pub mod Event;

/// Association : A_activityRef_compensateEventDefinition
pub mod A_activityRef_compensateEventDefinition;

/// Association : A_inputSet_throwEvent
pub mod A_inputSet_throwEvent;

/// Association : A_structureRef_signal
pub mod A_structureRef_signal;

/// Association : A_messageRef_messageEventDefinition
pub mod A_messageRef_messageEventDefinition;

/// Association : A_outputSet_catchEvent
pub mod A_outputSet_catchEvent;

/// Association : A_structureRef_escalation
pub mod A_structureRef_escalation;

/// Class : IntermediateCatchEvent
pub mod IntermediateCatchEvent;

/// Class : IntermediateThrowEvent
pub mod IntermediateThrowEvent;

/// Class : EndEvent
pub mod EndEvent;

/// Class : StartEvent
pub mod StartEvent;

/// Class : ThrowEvent
pub mod ThrowEvent;

/// Class : CatchEvent
pub mod CatchEvent;

/// Class : BoundaryEvent
pub mod BoundaryEvent;

/// Class : EventDefinition
pub mod EventDefinition;

/// Association : A_eventDefinitionRefs_throwEvent
pub mod A_eventDefinitionRefs_throwEvent;

/// Association : A_eventDefinitionRefs_catchEvent
pub mod A_eventDefinitionRefs_catchEvent;

/// Class : CancelEventDefinition
pub mod CancelEventDefinition;

/// Class : ErrorEventDefinition
pub mod ErrorEventDefinition;

/// Class : TerminateEventDefinition
pub mod TerminateEventDefinition;

/// Association : A_errorRef_errorEventDefinition
pub mod A_errorRef_errorEventDefinition;

/// Class : EscalationEventDefinition
pub mod EscalationEventDefinition;

/// Class : Escalation
pub mod Escalation;

/// Association : A_escalationRef_escalationEventDefinition
pub mod A_escalationRef_escalationEventDefinition;

/// Class : CompensateEventDefinition
pub mod CompensateEventDefinition;

/// Class : TimerEventDefinition
pub mod TimerEventDefinition;

/// Class : LinkEventDefinition
pub mod LinkEventDefinition;

/// Class : MessageEventDefinition
pub mod MessageEventDefinition;

/// Class : ConditionalEventDefinition
pub mod ConditionalEventDefinition;

/// Class : SignalEventDefinition
pub mod SignalEventDefinition;

/// Class : Signal
pub mod Signal;

/// Association : A_signalRef_signalEventDefinition
pub mod A_signalRef_signalEventDefinition;

/// Class : ImplicitThrowEvent
pub mod ImplicitThrowEvent;

/// Association : A_eventDefinitions_throwEvent
pub mod A_eventDefinitions_throwEvent;

/// Association : A_eventDefinitions_catchEvent
pub mod A_eventDefinitions_catchEvent;

/// Association : A_dataInputs_throwEvent
pub mod A_dataInputs_throwEvent;

/// Association : A_dataOutputs_catchEvent
pub mod A_dataOutputs_catchEvent;

/// Association : A_operationRef_messageEventDefinition
pub mod A_operationRef_messageEventDefinition;

/// Association : A_condition_conditionalEventDefinition
pub mod A_condition_conditionalEventDefinition;

/// Association : A_timeDate_timerEventDefinition
pub mod A_timeDate_timerEventDefinition;

/// Association : A_timeCycle_timerEventDefinition
pub mod A_timeCycle_timerEventDefinition;

/// Association : A_target_source
pub mod A_target_source;

/// Association : A_properties_event
pub mod A_properties_event;

/// Association : A_timeDuration_timerEventDefinition
pub mod A_timeDuration_timerEventDefinition;

/// Association : A_dataState_itemAwareElement
pub mod A_dataState_itemAwareElement;

/// Class : DataState
pub mod DataState;

/// Class : ItemAwareElement
pub mod ItemAwareElement;

/// Association : A_operationRef_ioBinding
pub mod A_operationRef_ioBinding;

/// Association : A_sourceRef_dataAssociation
pub mod A_sourceRef_dataAssociation;

/// Association : A_targetRef_dataAssociation
pub mod A_targetRef_dataAssociation;

/// Class : DataAssociation
pub mod DataAssociation;

/// Association : A_transformation_dataAssociation
pub mod A_transformation_dataAssociation;

/// Class : DataInput
pub mod DataInput;

/// Class : DataOutput
pub mod DataOutput;

/// Class : InputSet
pub mod InputSet;

/// Class : OutputSet
pub mod OutputSet;

/// Association : A_dataInputRefs_inputSetRefs
pub mod A_dataInputRefs_inputSetRefs;

/// Association : A_dataOutputRefs_outputSetRefs
pub mod A_dataOutputRefs_outputSetRefs;

/// Class : Property
pub mod Property;

/// Class : DataInputAssociation
pub mod DataInputAssociation;

/// Class : DataOutputAssociation
pub mod DataOutputAssociation;

/// Class : InputOutputSpecification
pub mod InputOutputSpecification;

/// Association : A_inputSets_inputOutputSpecification
pub mod A_inputSets_inputOutputSpecification;

/// Association : A_outputSets_inputOutputSpecification
pub mod A_outputSets_inputOutputSpecification;

/// Association : A_dataInputs_inputOutputSpecification
pub mod A_dataInputs_inputOutputSpecification;

/// Association : A_dataOutputs_inputOutputSpecification
pub mod A_dataOutputs_inputOutputSpecification;

/// Class : DataObject
pub mod DataObject;

/// Association : A_inputSetRefs_outputSetRefs
pub mod A_inputSetRefs_outputSetRefs;

/// Class : InputOutputBinding
pub mod InputOutputBinding;

/// Association : A_inputDataRef_inputOutputBinding
pub mod A_inputDataRef_inputOutputBinding;

/// Association : A_outputDataRef_inputOutputBinding
pub mod A_outputDataRef_inputOutputBinding;

/// Association : A_whileExecutingInputRefs_inputSetWithWhileExecuting
pub mod A_whileExecutingInputRefs_inputSetWithWhileExecuting;

/// Association : A_optionalInputRefs_inputSetWithOptional
pub mod A_optionalInputRefs_inputSetWithOptional;

/// Association : A_outputSetWithOptional_optionalOutputRefs
pub mod A_outputSetWithOptional_optionalOutputRefs;

/// Association : A_outputSetWithWhileExecuting_whileExecutingOutputRefs
pub mod A_outputSetWithWhileExecuting_whileExecutingOutputRefs;

/// Class : Assignment
pub mod Assignment;

/// Association : A_assignment_dataAssociation
pub mod A_assignment_dataAssociation;

/// Class : DataStore
pub mod DataStore;

/// Class : DataStoreReference
pub mod DataStoreReference;

/// Association : A_dataStoreRef_dataStoreReference
pub mod A_dataStoreRef_dataStoreReference;

/// Association : A_itemSubjectRef_itemAwareElement
pub mod A_itemSubjectRef_itemAwareElement;

/// Association : A_from_assignment
pub mod A_from_assignment;

/// Association : A_to_assignment
pub mod A_to_assignment;

/// Class : DataObjectReference
pub mod DataObjectReference;

/// Association : A_dataObjectRef_dataObject
pub mod A_dataObjectRef_dataObject;

/// Class : ConversationLink
pub mod ConversationLink;

/// Class : ConversationAssociation
pub mod ConversationAssociation;

/// Association : A_calledCollaborationRef_callConversation
pub mod A_calledCollaborationRef_callConversation;

/// Association : A_participantRefs_conversationNode
pub mod A_participantRefs_conversationNode;

/// Association : A_messageFlowRefs_communication
pub mod A_messageFlowRefs_communication;

/// Association : A_participantAssociations_callConversation
pub mod A_participantAssociations_callConversation;

/// Class : CallConversation
pub mod CallConversation;

/// Class : Conversation
pub mod Conversation;

/// Class : SubConversation
pub mod SubConversation;

/// Class : ConversationNode
pub mod ConversationNode;

/// Class : GlobalConversation
pub mod GlobalConversation;

/// Association : A_correlationKeys_collaboration
pub mod A_correlationKeys_collaboration;

/// Association : A_correlationKeys_conversationNode
pub mod A_correlationKeys_conversationNode;

/// Association : A_innerConversationNodeRef_conversationAssociation
pub mod A_innerConversationNodeRef_conversationAssociation;

/// Association : A_outerConversationNodeRef_conversationAssociation
pub mod A_outerConversationNodeRef_conversationAssociation;

/// Association : A_conversationNodes_subConversation
pub mod A_conversationNodes_subConversation;

/// Association : A_sourceRef_outgoingConversationLinks
pub mod A_sourceRef_outgoingConversationLinks;

/// Association : A_targetRef_incomingConversationLinks
pub mod A_targetRef_incomingConversationLinks;

/// Class : PartnerEntity
pub mod PartnerEntity;

/// Class : PartnerRole
pub mod PartnerRole;

/// Association : A_correlationPropertyRef_correlationKey
pub mod A_correlationPropertyRef_correlationKey;

/// Class : CorrelationProperty
pub mod CorrelationProperty;

/// Enumeration : ItemKind
pub mod ItemKind;

/// Association : A_supportedInterfaceRefs_callableElements
pub mod A_supportedInterfaceRefs_callableElements;

/// Association : A_ioBinding_callableElement
pub mod A_ioBinding_callableElement;

/// Association : A_ioSpecification_callableElement
pub mod A_ioSpecification_callableElement;

/// Association : A_messagePath_correlationset
pub mod A_messagePath_correlationset;

/// Association : A_structureRef_error
pub mod A_structureRef_error;

/// Class : Error
pub mod Error;

/// Association : A_evaluatesToTypeRef_formalExpression
pub mod A_evaluatesToTypeRef_formalExpression;

/// Class : CorrelationKey
pub mod CorrelationKey;

/// Class : Expression
pub mod Expression;

/// Class : FormalExpression
pub mod FormalExpression;

/// Class : Message
pub mod Message;

/// Class : ItemDefinition
pub mod ItemDefinition;

/// Association : A_conditionExpression_sequenceFlow
pub mod A_conditionExpression_sequenceFlow;

/// Class : FlowElement
pub mod FlowElement;

/// Class : SequenceFlow
pub mod SequenceFlow;

/// Class : FlowElementsContainer
pub mod FlowElementsContainer;

/// Association : A_flowElements_container
pub mod A_flowElements_container;

/// Class : CallableElement
pub mod CallableElement;

/// Class : FlowNode
pub mod FlowNode;

/// Association : A_sourceRef_outgoing_flow
pub mod A_sourceRef_outgoing_flow;

/// Association : A_targetRef_incoming_flow
pub mod A_targetRef_incoming_flow;

/// Class : CorrelationPropertyRetrievalExpression
pub mod CorrelationPropertyRetrievalExpression;

/// Class : CorrelationPropertyBinding
pub mod CorrelationPropertyBinding;

/// Association : A_correlationPropertyRetrievalExpression_correlationproperty
pub mod A_correlationPropertyRetrievalExpression_correlationproperty;

/// Association : A_messageRef_correlationPropertyRetrievalExpression
pub mod A_messageRef_correlationPropertyRetrievalExpression;

/// Association : A_dataPath_correlationPropertyBinding
pub mod A_dataPath_correlationPropertyBinding;

/// Association : A_correlationPropertyRef_correlationPropertyBinding
pub mod A_correlationPropertyRef_correlationPropertyBinding;

/// Class : Resource
pub mod Resource;

/// Class : ResourceParameter
pub mod ResourceParameter;

/// Association : A_resourceParameters_resource
pub mod A_resourceParameters_resource;

/// Association : A_import_itemDefinition
pub mod A_import_itemDefinition;

/// Class : CorrelationSubscription
pub mod CorrelationSubscription;

/// Association : A_correlationKeyRef_correlationSubscription
pub mod A_correlationKeyRef_correlationSubscription;

/// Association : A_correlationPropertyBinding_correlationSubscription
pub mod A_correlationPropertyBinding_correlationSubscription;

/// Association : A_auditing_flowElement
pub mod A_auditing_flowElement;

/// Association : A_monitoring_flowElement
pub mod A_monitoring_flowElement;

/// Association : A_type_correlationProperty
pub mod A_type_correlationProperty;

/// Association : A_type_resourceParameter
pub mod A_type_resourceParameter;

/// Association : A_itemRef_message
pub mod A_itemRef_message;

/// Association : A_laneSets_flowElementsContainer
pub mod A_laneSets_flowElementsContainer;

/// Class : MessageFlow
pub mod MessageFlow;

/// Class : MessageFlowAssociation
pub mod MessageFlowAssociation;

/// Class : InteractionNode
pub mod InteractionNode;

/// Class : Participant
pub mod Participant;

/// Class : ParticipantAssociation
pub mod ParticipantAssociation;

/// Class : ParticipantMultiplicity
pub mod ParticipantMultiplicity;

/// Association : A_messageFlowAssociations_collaboration
pub mod A_messageFlowAssociations_collaboration;

/// Association : A_participantAssociations_collaboration
pub mod A_participantAssociations_collaboration;

/// Association : A_artifacts_collaboration
pub mod A_artifacts_collaboration;

/// Class : Collaboration
pub mod Collaboration;

/// Association : A_conversationAssociations_converstaionAssociations
pub mod A_conversationAssociations_converstaionAssociations;

/// Association : A_choreographyRef_collaboration
pub mod A_choreographyRef_collaboration;

/// Association : A_innerParticipantRef_participantAssociation
pub mod A_innerParticipantRef_participantAssociation;

/// Association : A_outerParticipantRef_participantAssociation
pub mod A_outerParticipantRef_participantAssociation;

/// Association : A_endPointRefs_participant
pub mod A_endPointRefs_participant;

/// Association : A_participantMultiplicity_participant
pub mod A_participantMultiplicity_participant;

/// Association : A_interfaceRefs_participant
pub mod A_interfaceRefs_participant;

/// Association : A_partnerEntityRef_participantRef
pub mod A_partnerEntityRef_participantRef;

/// Association : A_partnerRoleRef_participantRef
pub mod A_partnerRoleRef_participantRef;

/// Association : A_processRef_participant
pub mod A_processRef_participant;

/// Association : A_innerMessageFlowRef_messageFlowAssociation
pub mod A_innerMessageFlowRef_messageFlowAssociation;

/// Association : A_outerMessageFlowRef_messageFlowAssociation
pub mod A_outerMessageFlowRef_messageFlowAssociation;

/// Association : A_targetRef_messageFlow
pub mod A_targetRef_messageFlow;

/// Association : A_messageRef_messageFlow
pub mod A_messageRef_messageFlow;

/// Association : A_sourceRef_messageFlow
pub mod A_sourceRef_messageFlow;

/// Association : A_participants_collaboration
pub mod A_participants_collaboration;

/// Association : A_messageFlows_collaboration
pub mod A_messageFlows_collaboration;

/// Association : A_conversations_collaboration
pub mod A_conversations_collaboration;

/// Association : A_conversationLinks_collaboration
pub mod A_conversationLinks_collaboration;

/// Association : A_participantAssociations_callChoreographyActivity
pub mod A_participantAssociations_callChoreographyActivity;

/// Class : ChoreographyActivity
pub mod ChoreographyActivity;

/// Class : CallChoreography
pub mod CallChoreography;

/// Class : SubChoreography
pub mod SubChoreography;

/// Class : ChoreographyTask
pub mod ChoreographyTask;

/// Association : A_calledChoreographyRef_callChoreographyActivity
pub mod A_calledChoreographyRef_callChoreographyActivity;

/// Association : A_messageFlowRef_choreographyTask
pub mod A_messageFlowRef_choreographyTask;

/// Association : A_initiatingParticipantRef_choreographyActivity
pub mod A_initiatingParticipantRef_choreographyActivity;

/// Association : A_participantRefs_choreographyActivity
pub mod A_participantRefs_choreographyActivity;

/// Association : A_artifacts_subChoreography
pub mod A_artifacts_subChoreography;

/// Enumeration : ChoreographyLoopType
pub mod ChoreographyLoopType;

/// Association : A_correlationKeys_choreographyActivity
pub mod A_correlationKeys_choreographyActivity;

/// Association : A_initiatingParticipantRef_globalChoreographyTask
pub mod A_initiatingParticipantRef_globalChoreographyTask;

/// Class : Choreography
pub mod Choreography;

/// Class : GlobalChoreographyTask
pub mod GlobalChoreographyTask;

/// Association : A_sourceRef_outgoing_association
pub mod A_sourceRef_outgoing_association;

/// Association : A_targetRef_incoming_association
pub mod A_targetRef_incoming_association;

/// Class : TextAnnotation
pub mod TextAnnotation;

/// Class : Group
pub mod Group;

/// Class : Association
pub mod Association;

/// Class : Category
pub mod Category;

/// Class : Artifact
pub mod Artifact;

/// Association : A_categoryValueRef_categoryValueRef
pub mod A_categoryValueRef_categoryValueRef;

/// Enumeration : AssociationDirection
pub mod AssociationDirection;

/// Class : CategoryValue
pub mod CategoryValue;

/// Association : A_categoryValue_category
pub mod A_categoryValue_category;

/// Association : A_categorizedFlowElements_categoryValueRef
pub mod A_categorizedFlowElements_categoryValueRef;

/// Association : A_event_complexBehaviorDefinition
pub mod A_event_complexBehaviorDefinition;

/// Association : A_expression_resourceAssignmentExpression
pub mod A_expression_resourceAssignmentExpression;

/// Association : A_expression_resourceParameterBinding
pub mod A_expression_resourceParameterBinding;

/// Association : A_noneBehaviorEventRef_multiInstanceLoopCharacteristics
pub mod A_noneBehaviorEventRef_multiInstanceLoopCharacteristics;

/// Association : A_oneBehaviorEventRef_multiInstanceLoopCharacteristics
pub mod A_oneBehaviorEventRef_multiInstanceLoopCharacteristics;

/// Association : A_completionCondition_multiInstanceLoopCharacteristics
pub mod A_completionCondition_multiInstanceLoopCharacteristics;

/// Association : A_condition_complexBehaviorDefinition
pub mod A_condition_complexBehaviorDefinition;

/// Association : A_resourceRef_activityResource
pub mod A_resourceRef_activityResource;

/// Association : A_messageRef_sendTask
pub mod A_messageRef_sendTask;

/// Association : A_messageRef_receiveTask
pub mod A_messageRef_receiveTask;

/// Class : Activity
pub mod Activity;

/// Association : A_operationRef_serviceTask
pub mod A_operationRef_serviceTask;

/// Association : A_calledElementRef_callActivity
pub mod A_calledElementRef_callActivity;

/// Association : A_loopCardinality_multiInstanceLoopCharacteristics
pub mod A_loopCardinality_multiInstanceLoopCharacteristics;

/// Association : A_properties_activity
pub mod A_properties_activity;

/// Association : A_resources_activity
pub mod A_resources_activity;

/// Association : A_loopCondition_standardLoopCharacteristics
pub mod A_loopCondition_standardLoopCharacteristics;

/// Association : A_loopCharacteristics_activity
pub mod A_loopCharacteristics_activity;

/// Association : A_ioSpecification_activity
pub mod A_ioSpecification_activity;

/// Association : A_completionCondition_adHocSubProcess
pub mod A_completionCondition_adHocSubProcess;

/// Class : ServiceTask
pub mod ServiceTask;

/// Class : SubProcess
pub mod SubProcess;

/// Association : A_operationRef_receiveTask
pub mod A_operationRef_receiveTask;

/// Association : A_operationRef_sendTask
pub mod A_operationRef_sendTask;

/// Class : LoopCharacteristics
pub mod LoopCharacteristics;

/// Enumeration : MultiInstanceBehavior
pub mod MultiInstanceBehavior;

/// Class : MultiInstanceLoopCharacteristics
pub mod MultiInstanceLoopCharacteristics;

/// Class : StandardLoopCharacteristics
pub mod StandardLoopCharacteristics;

/// Class : CallActivity
pub mod CallActivity;

/// Class : Task
pub mod Task;

/// Class : SendTask
pub mod SendTask;

/// Class : ReceiveTask
pub mod ReceiveTask;

/// Class : ScriptTask
pub mod ScriptTask;

/// Class : BusinessRuleTask
pub mod BusinessRuleTask;

/// Class : AdHocSubProcess
pub mod AdHocSubProcess;

/// Enumeration : AdHocOrdering
pub mod AdHocOrdering;

/// Class : Transaction
pub mod Transaction;

/// Class : GlobalScriptTask
pub mod GlobalScriptTask;

/// Class : GlobalBusinessRuleTask
pub mod GlobalBusinessRuleTask;

/// Class : ComplexBehaviorDefinition
pub mod ComplexBehaviorDefinition;

/// Association : A_complexBehaviorDefinition_multiInstanceLoopCharacteristics
pub mod A_complexBehaviorDefinition_multiInstanceLoopCharacteristics;

/// Class : ResourceRole
pub mod ResourceRole;

/// Class : ResourceParameterBinding
pub mod ResourceParameterBinding;

/// Class : ResourceAssignmentExpression
pub mod ResourceAssignmentExpression;

/// Association : A_resourceParameterBindings_activityResource
pub mod A_resourceParameterBindings_activityResource;

/// Association : A_resourceAssignmentExpression_activityResource
pub mod A_resourceAssignmentExpression_activityResource;

/// Association : A_loopMaximum_standardLoopCharacteristics
pub mod A_loopMaximum_standardLoopCharacteristics;

/// Association : A_dataInputAssociations_activity
pub mod A_dataInputAssociations_activity;

/// Association : A_dataOutputAssociations_activity
pub mod A_dataOutputAssociations_activity;

/// Association : A_parameterRef_resourceParameterBinding
pub mod A_parameterRef_resourceParameterBinding;

/// Association : A_loopDataInputRef_multiInstanceLoopCharacteristics
pub mod A_loopDataInputRef_multiInstanceLoopCharacteristics;

/// Association : A_loopDataOutputRef_multiInstanceLoopCharacteristics
pub mod A_loopDataOutputRef_multiInstanceLoopCharacteristics;

/// Association : A_inputDataItem_multiInstanceLoopCharacteristics
pub mod A_inputDataItem_multiInstanceLoopCharacteristics;

/// Association : A_outputDataItem_multiInstanceLoopCharacteristics
pub mod A_outputDataItem_multiInstanceLoopCharacteristics;

/// Association : A_boundaryEventRefs_attachedToRef
pub mod A_boundaryEventRefs_attachedToRef;

/// Association : A_default_activity
pub mod A_default_activity;

/// Association : A_artifacts_subProcess
pub mod A_artifacts_subProcess;

/// Class : Import
pub mod Import;

/// Class : Definitions
pub mod Definitions;

/// Association : A_diagrams_definitions
pub mod A_diagrams_definitions;

/// Association : A_imports_definition
pub mod A_imports_definition;

/// Association : A_extensions_definitions
pub mod A_extensions_definitions;

/// Association : A_relationships_definition
pub mod A_relationships_definition;

/// Association : A_rootElements_definition
pub mod A_rootElements_definition;
