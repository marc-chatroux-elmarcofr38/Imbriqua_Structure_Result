//! bpmn_20_class_extension_definition

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_extension_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SIMPLE FIELD : ExtensionDefinition-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

// ManyToMany : with BaseElement using A_extensionDefinitions_baseElement
impl Related<super::bpmn_20_a_extension_definitions_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_extension_definitions_base_element::Relation::BaseElement.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_extension_definitions_base_element::Relation::ExtensionDefinition
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ExtensionDefinition" (bpmn_20_class_extension_definition)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "ExtensionDefinition-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Reverse One To One :
    /// * __Extension__ (__ExtensionModel__) from A_definition_extension
    ///   * one-to-one link : one __Extension__ need one __ExtensionDefinition__)
    ///   * callable using find_also_related(__ExtensionDefinitionModel__) from __Extension__
    ///   * saved in __definition__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ExtensionDefinition" (bpmn_20_class_extension_definition)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "ExtensionDefinition-name")
  * type : __std::string::String__


## Reverse One To One :
* __Extension__ (__ExtensionModel__) from A_definition_extension
  * one-to-one link : one __Extension__ need one __ExtensionDefinition__)
  * callable using find_also_related(__ExtensionDefinitionModel__) from __Extension__
  * saved in __definition__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "ExtensionDefinition",
//     name: "ExtensionDefinition",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ExtensionDefinition-name",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "ExtensionDefinition-extensionAttributeDefinitions",
//                 name: "extensionAttributeDefinitions",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ExtensionAttributeDefinition",
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
//                     "A_extensionAttributeDefinitions_extensionDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

