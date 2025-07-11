//! bpmn_20_class_resource_role

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_resource_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : ResourceRole-resourceRef
    pub resource_ref: Option<i64>,
    /// COMPLEX FIELD : ResourceRole-resourceAssignmentExpression
    pub resource_assignment_expression: Option<i64>,
    /// SIMPLE FIELD : ResourceRole-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ResourceRole need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
    // SUPER : ONE Performer need ONE ResourceRole
    #[sea_orm(has_one = "super::bpmn_20_performer::Entity")]
    Performer,
}

// SUPER : ONE ResourceRole need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE Performer need ONE ResourceRole
impl Related<super::bpmn_20_performer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Performer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ResourceRole" (bpmn_20_class_resource_role)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "ResourceRole-name")
    ///   * type : __std::string::String__
    /// 
    /// ## Direct One To One :
    /// * __ResourceAssignmentExpression__ (__ResourceAssignmentExpressionModel__) from A_resourceAssignmentExpression_activityResource
    ///   * one-to-one link : one __ResourceRole__ need one __ResourceAssignmentExpression__)
    ///   * callable using find_also_related(__ResourceAssignmentExpressionModel__) from __ResourceRole__
    ///   * saved in __resource_assignment_expression__ field as foreing key
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __ResourceRole__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ResourceRole__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __Performer__ (__PerformerModel__)
    ///   * one-to-one link (reverse) : one __Performer__ need one __ResourceRole__)
    ///   * callable using find_also_related(__ResourceRoleModel__) from __Performer__
    ///   * saved in __super_resource_role__ field as foreing key in __PerformerModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ResourceRole" (bpmn_20_class_resource_role)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "ResourceRole-name")
  * type : __std::string::String__

## Direct One To One :
* __ResourceAssignmentExpression__ (__ResourceAssignmentExpressionModel__) from A_resourceAssignmentExpression_activityResource
  * one-to-one link : one __ResourceRole__ need one __ResourceAssignmentExpression__)
  * callable using find_also_related(__ResourceAssignmentExpressionModel__) from __ResourceRole__
  * saved in __resource_assignment_expression__ field as foreing key

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __ResourceRole__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ResourceRole__
  * saved in __super_base_element__ field as foreing key

## Reverse Super :
* __Performer__ (__PerformerModel__)
  * one-to-one link (reverse) : one __Performer__ need one __ResourceRole__)
  * callable using find_also_related(__ResourceRoleModel__) from __Performer__
  * saved in __super_resource_role__ field as foreing key in __PerformerModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ResourceRole",
//     name: "ResourceRole",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ResourceRole-resourceRef",
//                 name: "resourceRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Resource",
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
//                     "A_resourceRef_activityResource",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ResourceRole-resourceParameterBindings",
//                 name: "resourceParameterBindings",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceParameterBinding",
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
//                     "A_resourceParameterBindings_activityResource",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ResourceRole-resourceAssignmentExpression",
//                 name: "resourceAssignmentExpression",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ResourceAssignmentExpression",
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
//                     "A_resourceAssignmentExpression_activityResource",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "ResourceRole-name",
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
//     ],
//     owned_rule: [],
// }

