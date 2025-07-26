//! bpmn_20_class_base_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_base_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SIMPLE FIELD : BPMN20-BaseElement-id
    pub bpmn_id: String,
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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-BaseElement',
//     name: "BaseElement",
//     is_abstract: true,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "BaseElement-documentation": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-BaseElement-documentation',
//                 name: "documentation",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-Documentation',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_documentation_baseElement',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "BaseElement-extensionDefinitions": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-BaseElement-extensionDefinitions',
//                 name: "extensionDefinitions",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-ExtensionDefinition',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_extensionDefinitions_baseElement',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "BaseElement-extensionValues": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-BaseElement-extensionValues',
//                 name: "extensionValues",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-ExtensionAttributeValue',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_extensionValues_baseElement',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "BaseElement-id": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-BaseElement-id',
//                 name: "id",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Loaded XMIIdReference RefCell of 'DC-String',
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
//                 owning_association: None,
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
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//         ],
//     },
// }

