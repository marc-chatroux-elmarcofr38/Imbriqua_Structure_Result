//! bpmn_20_class_expression

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_expression")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Expression need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE FormalExpression need ONE Expression
    #[sea_orm(has_one = "super::bpmn_20_formal_expression::Entity")]
    FormalExpression,
}

// SUPER : ONE Expression need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE FormalExpression need ONE Expression
impl Related<super::bpmn_20_formal_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormalExpression.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Expression" (bpmn_20_class_expression)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Expression__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Expression__
    ///   * saved in __super_base_element__ field as foreing key
    /// ## Reverse One To One :
    /// * __ComplexGateway__ (__ComplexGatewayModel__) from A_activationCondition_complexGateway
    ///   * one-to-one link : (0-1) __ComplexGateway__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __ComplexGateway__
    ///   * saved in __activation_condition__ field as foreing key
    /// * __AdHocSubProcess__ (__AdHocSubProcessModel__) from A_completionCondition_adHocSubProcess
    ///   * one-to-one link : (1-1) __AdHocSubProcess__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __AdHocSubProcess__
    ///   * saved in __completion_condition__ field as foreing key
    /// * __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_completionCondition_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __completion_condition__ field as foreing key
    /// * __SequenceFlow__ (__SequenceFlowModel__) from A_conditionExpression_sequenceFlow
    ///   * one-to-one link : (0-1) __SequenceFlow__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __SequenceFlow__
    ///   * saved in __condition_expression__ field as foreing key
    /// * __ConditionalEventDefinition__ (__ConditionalEventDefinitionModel__) from A_condition_conditionalEventDefinition
    ///   * one-to-one link : (1-1) __ConditionalEventDefinition__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __ConditionalEventDefinition__
    ///   * saved in __condition__ field as foreing key
    /// * __ResourceAssignmentExpression__ (__ResourceAssignmentExpressionModel__) from A_expression_resourceAssignmentExpression
    ///   * one-to-one link : (1-1) __ResourceAssignmentExpression__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __ResourceAssignmentExpression__
    ///   * saved in __expression__ field as foreing key
    /// * __ResourceParameterBinding__ (__ResourceParameterBindingModel__) from A_expression_resourceParameterBinding
    ///   * one-to-one link : (1-1) __ResourceParameterBinding__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __ResourceParameterBinding__
    ///   * saved in __expression__ field as foreing key
    /// * __Assignment__ (__AssignmentModel__) from A_from_assignment
    ///   * one-to-one link : (1-1) __Assignment__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __Assignment__
    ///   * saved in __from__ field as foreing key
    /// * __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_loopCardinality_multiInstanceLoopCharacteristics
    ///   * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __MultiInstanceLoopCharacteristics__
    ///   * saved in __loop_cardinality__ field as foreing key
    /// * __StandardLoopCharacteristics__ (__StandardLoopCharacteristicsModel__) from A_loopCondition_standardLoopCharacteristics
    ///   * one-to-one link : (0-1) __StandardLoopCharacteristics__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __StandardLoopCharacteristics__
    ///   * saved in __loop_condition__ field as foreing key
    /// * __StandardLoopCharacteristics__ (__StandardLoopCharacteristicsModel__) from A_loopMaximum_standardLoopCharacteristics
    ///   * one-to-one link : (0-1) __StandardLoopCharacteristics__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __StandardLoopCharacteristics__
    ///   * saved in __loop_maximum__ field as foreing key
    /// * __TimerEventDefinition__ (__TimerEventDefinitionModel__) from A_timeCycle_timerEventDefinition
    ///   * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
    ///   * saved in __time_cycle__ field as foreing key
    /// * __TimerEventDefinition__ (__TimerEventDefinitionModel__) from A_timeDate_timerEventDefinition
    ///   * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
    ///   * saved in __time_date__ field as foreing key
    /// * __TimerEventDefinition__ (__TimerEventDefinitionModel__) from A_timeDuration_timerEventDefinition
    ///   * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
    ///   * saved in __time_duration__ field as foreing key
    /// * __Assignment__ (__AssignmentModel__) from A_to_assignment
    ///   * one-to-one link : (1-1) __Assignment__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __Assignment__
    ///   * saved in __to__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __FormalExpression__ (__FormalExpressionModel__)
    ///   * one-to-one link (reverse) : one __FormalExpression__ need one __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __FormalExpression__
    ///   * saved in __super_expression__ field as foreing key in __FormalExpressionModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Expression" (bpmn_20_class_expression)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Expression__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Expression__
  * saved in __super_base_element__ field as foreing key
## Reverse One To One :
* __ComplexGateway__ (__ComplexGatewayModel__) from A_activationCondition_complexGateway
  * one-to-one link : (0-1) __ComplexGateway__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __ComplexGateway__
  * saved in __activation_condition__ field as foreing key
* __AdHocSubProcess__ (__AdHocSubProcessModel__) from A_completionCondition_adHocSubProcess
  * one-to-one link : (1-1) __AdHocSubProcess__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __AdHocSubProcess__
  * saved in __completion_condition__ field as foreing key
* __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_completionCondition_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __completion_condition__ field as foreing key
* __SequenceFlow__ (__SequenceFlowModel__) from A_conditionExpression_sequenceFlow
  * one-to-one link : (0-1) __SequenceFlow__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __SequenceFlow__
  * saved in __condition_expression__ field as foreing key
* __ConditionalEventDefinition__ (__ConditionalEventDefinitionModel__) from A_condition_conditionalEventDefinition
  * one-to-one link : (1-1) __ConditionalEventDefinition__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __ConditionalEventDefinition__
  * saved in __condition__ field as foreing key
* __ResourceAssignmentExpression__ (__ResourceAssignmentExpressionModel__) from A_expression_resourceAssignmentExpression
  * one-to-one link : (1-1) __ResourceAssignmentExpression__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __ResourceAssignmentExpression__
  * saved in __expression__ field as foreing key
* __ResourceParameterBinding__ (__ResourceParameterBindingModel__) from A_expression_resourceParameterBinding
  * one-to-one link : (1-1) __ResourceParameterBinding__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __ResourceParameterBinding__
  * saved in __expression__ field as foreing key
* __Assignment__ (__AssignmentModel__) from A_from_assignment
  * one-to-one link : (1-1) __Assignment__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __Assignment__
  * saved in __from__ field as foreing key
* __MultiInstanceLoopCharacteristics__ (__MultiInstanceLoopCharacteristicsModel__) from A_loopCardinality_multiInstanceLoopCharacteristics
  * one-to-one link : (0-1) __MultiInstanceLoopCharacteristics__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __MultiInstanceLoopCharacteristics__
  * saved in __loop_cardinality__ field as foreing key
* __StandardLoopCharacteristics__ (__StandardLoopCharacteristicsModel__) from A_loopCondition_standardLoopCharacteristics
  * one-to-one link : (0-1) __StandardLoopCharacteristics__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __StandardLoopCharacteristics__
  * saved in __loop_condition__ field as foreing key
* __StandardLoopCharacteristics__ (__StandardLoopCharacteristicsModel__) from A_loopMaximum_standardLoopCharacteristics
  * one-to-one link : (0-1) __StandardLoopCharacteristics__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __StandardLoopCharacteristics__
  * saved in __loop_maximum__ field as foreing key
* __TimerEventDefinition__ (__TimerEventDefinitionModel__) from A_timeCycle_timerEventDefinition
  * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
  * saved in __time_cycle__ field as foreing key
* __TimerEventDefinition__ (__TimerEventDefinitionModel__) from A_timeDate_timerEventDefinition
  * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
  * saved in __time_date__ field as foreing key
* __TimerEventDefinition__ (__TimerEventDefinitionModel__) from A_timeDuration_timerEventDefinition
  * one-to-one link : (0-1) __TimerEventDefinition__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __TimerEventDefinition__
  * saved in __time_duration__ field as foreing key
* __Assignment__ (__AssignmentModel__) from A_to_assignment
  * one-to-one link : (1-1) __Assignment__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __Assignment__
  * saved in __to__ field as foreing key

## Reverse Super :
* __FormalExpression__ (__FormalExpressionModel__)
  * one-to-one link (reverse) : one __FormalExpression__ need one __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __FormalExpression__
  * saved in __super_expression__ field as foreing key in __FormalExpressionModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-Expression" (loaded : false)",
//     name: "Expression",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {},
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Expression",
//     table_name: "bpmn_20_expression",
//     model_name: "Expression",
//     full_name: "bpmn_20_class_expression",
// }

