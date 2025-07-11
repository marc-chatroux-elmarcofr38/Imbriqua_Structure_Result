//! bpmn_20_class_complex_behavior_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_complex_behavior_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : ComplexBehaviorDefinition-condition
    pub condition: i64,
    /// COMPLEX FIELD : ComplexBehaviorDefinition-event
    pub event: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ComplexBehaviorDefinition need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE ComplexBehaviorDefinition need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ComplexBehaviorDefinition" (bpmn_20_class_complex_behavior_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// ## Direct One To One :
    /// * __FormalExpression__ (__FormalExpressionModel__) from A_condition_complexBehaviorDefinition
    ///   * one-to-one link : one __ComplexBehaviorDefinition__ need one __FormalExpression__)
    ///   * callable using find_also_related(__FormalExpressionModel__) from __ComplexBehaviorDefinition__
    ///   * saved in __condition__ field as foreing key
    /// * __ImplicitThrowEvent__ (__ImplicitThrowEventModel__) from A_event_complexBehaviorDefinition
    ///   * one-to-one link : one __ComplexBehaviorDefinition__ need one __ImplicitThrowEvent__)
    ///   * callable using find_also_related(__ImplicitThrowEventModel__) from __ComplexBehaviorDefinition__
    ///   * saved in __event__ field as foreing key
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __ComplexBehaviorDefinition__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ComplexBehaviorDefinition__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ComplexBehaviorDefinition" (bpmn_20_class_complex_behavior_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__


## Direct One To One :
* __FormalExpression__ (__FormalExpressionModel__) from A_condition_complexBehaviorDefinition
  * one-to-one link : one __ComplexBehaviorDefinition__ need one __FormalExpression__)
  * callable using find_also_related(__FormalExpressionModel__) from __ComplexBehaviorDefinition__
  * saved in __condition__ field as foreing key
* __ImplicitThrowEvent__ (__ImplicitThrowEventModel__) from A_event_complexBehaviorDefinition
  * one-to-one link : one __ComplexBehaviorDefinition__ need one __ImplicitThrowEvent__)
  * callable using find_also_related(__ImplicitThrowEventModel__) from __ComplexBehaviorDefinition__
  * saved in __event__ field as foreing key

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __ComplexBehaviorDefinition__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ComplexBehaviorDefinition__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ComplexBehaviorDefinition",
//     name: "ComplexBehaviorDefinition",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ComplexBehaviorDefinition-condition",
//                 name: "condition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FormalExpression",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                     "A_condition_complexBehaviorDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ComplexBehaviorDefinition-event",
//                 name: "event",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ImplicitThrowEvent",
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
//                     "A_event_complexBehaviorDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

