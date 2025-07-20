//! bpmn_20_class_item_definition

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_item_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
    /// COMPLEX FIELD : BPMN20-ItemDefinition-import
    pub import: Option<i64>,
    /// COMPLEX FIELD : BPMN20-ItemDefinition-structureRef
    pub structure_ref: i64,
    /// SIMPLE FIELD : BPMN20-ItemDefinition-isCollection
    #[sea_orm(default_value = "false")]
    pub is_collection: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-ItemDefinition-itemKind
    pub item_kind: ItemKind,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ItemDefinition need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
}

// SUPER : ONE ItemDefinition need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ItemDefinition" (bpmn_20_class_item_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __is_collection__ (xmi_id : "BPMN20-ItemDefinition-isCollection")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// * __item_kind__ (xmi_id : "BPMN20-ItemDefinition-itemKind")
    ///   * type : __ItemKind__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Import__ (__ImportModel__) from A_import_itemDefinition
    ///   * one-to-many link : (0-1) __ItemDefinition__ need (0-inf) __Import__)
    ///   * callable using find_with_related(__ImportModel__) from __ItemDefinition__
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __ItemDefinition__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __ItemDefinition__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ItemDefinition" (bpmn_20_class_item_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __is_collection__ (xmi_id : "BPMN20-ItemDefinition-isCollection")
  * type : __std::primitive::bool__
  * default : "false"
* __item_kind__ (xmi_id : "BPMN20-ItemDefinition-itemKind")
  * type : __ItemKind__


## Relation : One To Many :
* __Import__ (__ImportModel__) from A_import_itemDefinition
  * one-to-many link : (0-1) __ItemDefinition__ need (0-inf) __Import__)
  * callable using find_with_related(__ImportModel__) from __ItemDefinition__

## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __ItemDefinition__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __ItemDefinition__
  * saved in __super_root_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "ItemDefinition",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "ItemDefinition",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-ItemDefinition-import": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "ItemDefinition-import",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "import",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Import",
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
//                     "A_import_itemDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-ItemDefinition-isCollection": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "ItemDefinition-isCollection",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isCollection",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
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
//         "-ItemDefinition-itemKind": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "ItemDefinition-itemKind",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "itemKind",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemKind",
//                 ),
//                 complex_type: None,
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
//         "-ItemDefinition-structureRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "ItemDefinition-structureRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "structureRef",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "Extensibilty.cmof#Element",
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
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ItemDefinition",
//     table_name: "bpmn_20_item_definition",
//     model_name: "ItemDefinition",
//     full_name: "bpmn_20_class_item_definition",
// }

