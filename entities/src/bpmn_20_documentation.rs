//! bpmn_20_class_documentation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_documentation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : Documentation-text
    pub text: std::string::String,
    /// SIMPLE FIELD : Documentation-textFormat
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
    /// * __text__ (xmi_id : "Documentation-text")
    ///   * type : __std::string::String__
    /// * __text_format__ (xmi_id : "Documentation-textFormat")
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
* __text__ (xmi_id : "Documentation-text")
  * type : __std::string::String__
* __text_format__ (xmi_id : "Documentation-textFormat")
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
//     xmi_id: "Documentation",
//     name: "Documentation",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Documentation-text",
//                 name: "text",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Documentation-textFormat",
//                 name: "textFormat",
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
//     ],
//     owned_rule: [],
// }

