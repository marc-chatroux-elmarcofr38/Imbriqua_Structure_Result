//! bpmn_20_class_documentation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_documentation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : BPMN20-Documentation-text
    pub text: std::string::String,
    /// SIMPLE FIELD : BPMN20-Documentation-textFormat
    #[sea_orm(default_value = "text/plain")]
    pub text_format: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Documentation need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE Documentation need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Documentation" (bpmn_20_class_documentation)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __text__ (xmi_id : "BPMN20-Documentation-text")
    ///   * type : __std::string::String__
    /// * __text_format__ (xmi_id : "BPMN20-Documentation-textFormat")
    ///   * type : __std::string::String__
    ///   * default : "text/plain"
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __BaseElement__ (__BaseElementModel__) from A_documentation_baseElement
    ///   * one-to-many link : (1-1) __Documentation__ need (0-inf) __BaseElement__)
    ///   * callable using find_with_related(__BaseElementModel__) from __Documentation__
    ///   * named base_element in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Documentation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Documentation__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Documentation" (bpmn_20_class_documentation)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __text__ (xmi_id : "BPMN20-Documentation-text")
  * type : __std::string::String__
* __text_format__ (xmi_id : "BPMN20-Documentation-textFormat")
  * type : __std::string::String__
  * default : "text/plain"


## Relation : One To Many :
* __BaseElement__ (__BaseElementModel__) from A_documentation_baseElement
  * one-to-many link : (1-1) __Documentation__ need (0-inf) __BaseElement__)
  * callable using find_with_related(__BaseElementModel__) from __Documentation__
  * named base_element in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Documentation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Documentation__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "Documentation",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Documentation",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Documentation-text": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Documentation-text",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "text",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-Documentation-textFormat": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Documentation-textFormat",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "textFormat",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "text/plain",
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
//     technical_name: "BPMN20.cmof#Documentation",
//     table_name: "bpmn_20_documentation",
//     model_name: "Documentation",
//     full_name: "bpmn_20_class_documentation",
// }

