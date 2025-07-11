//! bpmn_20_class_extension

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_extension")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// COMPLEX FIELD : Extension-definition
    pub definition: i64,
    /// SIMPLE FIELD : Extension-mustUnderstand
    #[sea_orm(default_value = "false")]
    pub must_understand: std::primitive::bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Extension" (bpmn_20_class_extension)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __must_understand__ (xmi_id : "Extension-mustUnderstand")
    ///   * type : __std::primitive::bool__
    ///   * default : "false"
    /// 
    /// ## Direct One To One :
    /// * __ExtensionDefinition__ (__ExtensionDefinitionModel__) from A_definition_extension
    ///   * one-to-one link : one __Extension__ need one __ExtensionDefinition__)
    ///   * callable using find_also_related(__ExtensionDefinitionModel__) from __Extension__
    ///   * saved in __definition__ field as foreing key
    /// 
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Extension" (bpmn_20_class_extension)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __must_understand__ (xmi_id : "Extension-mustUnderstand")
  * type : __std::primitive::bool__
  * default : "false"

## Direct One To One :
* __ExtensionDefinition__ (__ExtensionDefinitionModel__) from A_definition_extension
  * one-to-one link : one __Extension__ need one __ExtensionDefinition__)
  * callable using find_also_related(__ExtensionDefinitionModel__) from __Extension__
  * saved in __definition__ field as foreing key



"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Extension",
//     name: "Extension",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Extension-mustUnderstand",
//                 name: "mustUnderstand",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Extension-definition",
//                 name: "definition",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ExtensionDefinition",
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
//                     "A_definition_extension",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

