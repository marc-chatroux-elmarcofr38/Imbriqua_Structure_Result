//! bpmn_20_class_standard_loop_characteristics

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_standard_loop_characteristics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : LoopCharacteristics
    pub super_loop_characteristics: i64,
    /// COMPLEX FIELD : StandardLoopCharacteristics-loopCondition
    pub loop_condition: Option<i64>,
    /// COMPLEX FIELD : StandardLoopCharacteristics-loopMaximum
    pub loop_maximum: Option<i64>,
    /// SIMPLE FIELD : StandardLoopCharacteristics-testBefore
    #[sea_orm(default_value = "false")]
    pub test_before: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE StandardLoopCharacteristics need ONE LoopCharacteristics
    #[sea_orm(
        belongs_to = "super::bpmn_20_loop_characteristics::Entity",
        from = "Column::SuperLoopCharacteristics",
        to = "super::bpmn_20_loop_characteristics::Column::Id",
        on_delete = "Cascade"
    )]
    LoopCharacteristics,
}

// SUPER : ONE StandardLoopCharacteristics need ONE LoopCharacteristics
impl Related<super::bpmn_20_loop_characteristics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoopCharacteristics.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "StandardLoopCharacteristics" (bpmn_20_class_standard_loop_characteristics)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __test_before__ (xmi_id : "StandardLoopCharacteristics-testBefore")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// ## Direct One To One :
    /// * __Expression__ (__ExpressionModel__) from A_loopCondition_standardLoopCharacteristics
    ///   * one-to-one link : (0-1) __StandardLoopCharacteristics__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __StandardLoopCharacteristics__
    ///   * saved in __loop_condition__ field as foreing key
    /// * __Expression__ (__ExpressionModel__) from A_loopMaximum_standardLoopCharacteristics
    ///   * one-to-one link : (0-1) __StandardLoopCharacteristics__ need (0-1) __Expression__)
    ///   * callable using find_also_related(__ExpressionModel__) from __StandardLoopCharacteristics__
    ///   * saved in __loop_maximum__ field as foreing key
    /// 
    /// 
    /// ## Direct Super :
    /// * __LoopCharacteristics__ (__LoopCharacteristicsModel__)
    ///   * one-to-one link : one __StandardLoopCharacteristics__ need one __LoopCharacteristics__)
    ///   * callable using find_also_related(__LoopCharacteristicsModel__) from __StandardLoopCharacteristics__
    ///   * saved in __super_loop_characteristics__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "StandardLoopCharacteristics" (bpmn_20_class_standard_loop_characteristics)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __test_before__ (xmi_id : "StandardLoopCharacteristics-testBefore")
  * type : __std::primitive::bool__
  * default : "false"

## Direct One To One :
* __Expression__ (__ExpressionModel__) from A_loopCondition_standardLoopCharacteristics
  * one-to-one link : (0-1) __StandardLoopCharacteristics__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __StandardLoopCharacteristics__
  * saved in __loop_condition__ field as foreing key
* __Expression__ (__ExpressionModel__) from A_loopMaximum_standardLoopCharacteristics
  * one-to-one link : (0-1) __StandardLoopCharacteristics__ need (0-1) __Expression__)
  * callable using find_also_related(__ExpressionModel__) from __StandardLoopCharacteristics__
  * saved in __loop_maximum__ field as foreing key


## Direct Super :
* __LoopCharacteristics__ (__LoopCharacteristicsModel__)
  * one-to-one link : one __StandardLoopCharacteristics__ need one __LoopCharacteristics__)
  * callable using find_also_related(__LoopCharacteristicsModel__) from __StandardLoopCharacteristics__
  * saved in __super_loop_characteristics__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "StandardLoopCharacteristics",
//     name: "StandardLoopCharacteristics",
//     is_abstract: false,
//     super_class: [
//         "LoopCharacteristics",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "StandardLoopCharacteristics-loopCondition": Property(
//             CMOFProperty {
//                 xmi_id: "StandardLoopCharacteristics-loopCondition",
//                 name: "loopCondition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
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
//                     "A_loopCondition_standardLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "StandardLoopCharacteristics-loopMaximum": Property(
//             CMOFProperty {
//                 xmi_id: "StandardLoopCharacteristics-loopMaximum",
//                 name: "loopMaximum",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Expression",
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
//                     "A_loopMaximum_standardLoopCharacteristics",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "StandardLoopCharacteristics-testBefore": Property(
//             CMOFProperty {
//                 xmi_id: "StandardLoopCharacteristics-testBefore",
//                 name: "testBefore",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "false",
//                 ),
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
//     technical_name: "BPMN20.cmof#StandardLoopCharacteristics",
//     table_name: "bpmn_20_standard_loop_characteristics",
//     model_name: "StandardLoopCharacteristics",
//     full_name: "bpmn_20_class_standard_loop_characteristics",
// }

