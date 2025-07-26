//! bpmn_20_class_standard_loop_characteristics

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_standard_loop_characteristics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperLoopCharacteristics
    pub super_loop_characteristics: i64,
    /// COMPLEX FIELD : BPMN20-StandardLoopCharacteristics-loopCondition
    pub loop_condition: Option<i64>,
    /// COMPLEX FIELD : BPMN20-StandardLoopCharacteristics-loopMaximum
    pub loop_maximum: Option<i64>,
    /// SIMPLE FIELD : BPMN20-StandardLoopCharacteristics-testBefore
    #[sea_orm(default_value = "false")]
    pub test_before: Boolean,
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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-StandardLoopCharacteristics',
//     name: "StandardLoopCharacteristics",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-LoopCharacteristics',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "StandardLoopCharacteristics-loopCondition": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-StandardLoopCharacteristics-loopCondition',
//                 name: "loopCondition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-Expression',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_loopCondition_standardLoopCharacteristics',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "StandardLoopCharacteristics-loopMaximum": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-StandardLoopCharacteristics-loopMaximum',
//                 name: "loopMaximum",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-Expression',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_loopMaximum_standardLoopCharacteristics',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "StandardLoopCharacteristics-testBefore": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-StandardLoopCharacteristics-testBefore',
//                 name: "testBefore",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Loaded XMIIdReference RefCell of 'DC-Boolean',
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
//                 owning_association: None,
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
//     reverse_super: RefCell {
//         value: [],
//     },
// }

