//! bpmn_20_class_resource_parameter

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_resource_parameter")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : BPMN20-ResourceParameter-type
    pub r#type: Option<i64>,
    /// SIMPLE FIELD : BPMN20-ResourceParameter-isRequired
    pub is_required: std::primitive::bool,
    /// SIMPLE FIELD : BPMN20-ResourceParameter-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ResourceParameter need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE ResourceParameter need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ResourceParameter" (bpmn_20_class_resource_parameter)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __is_required__ (xmi_id : "BPMN20-ResourceParameter-isRequired")
    ///   * type : __std::primitive::bool__
    /// * __name__ (xmi_id : "BPMN20-ResourceParameter-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Resource__ (__ResourceModel__) from A_resourceParameters_resource
    ///   * one-to-many link : (1-1) __ResourceParameter__ need (0-inf) __Resource__)
    ///   * callable using find_with_related(__ResourceModel__) from __ResourceParameter__
    ///   * named resource in BPMN
    /// * __ItemDefinition__ (__ItemDefinitionModel__) from A_type_resourceParameter
    ///   * one-to-many link : (0-1) __ResourceParameter__ need (0-inf) __ItemDefinition__)
    ///   * callable using find_with_related(__ItemDefinitionModel__) from __ResourceParameter__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __ResourceParameter__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __ResourceParameter__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ResourceParameter" (bpmn_20_class_resource_parameter)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __is_required__ (xmi_id : "BPMN20-ResourceParameter-isRequired")
  * type : __std::primitive::bool__
* __name__ (xmi_id : "BPMN20-ResourceParameter-name")
  * type : __std::string::String__


## Relation : One To Many :
* __Resource__ (__ResourceModel__) from A_resourceParameters_resource
  * one-to-many link : (1-1) __ResourceParameter__ need (0-inf) __Resource__)
  * callable using find_with_related(__ResourceModel__) from __ResourceParameter__
  * named resource in BPMN
* __ItemDefinition__ (__ItemDefinitionModel__) from A_type_resourceParameter
  * one-to-many link : (0-1) __ResourceParameter__ need (0-inf) __ItemDefinition__)
  * callable using find_with_related(__ItemDefinitionModel__) from __ResourceParameter__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __ResourceParameter__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __ResourceParameter__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         local_id: "ResourceParameter",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "ResourceParameter",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-ResourceParameter-isRequired": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ResourceParameter-isRequired",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "isRequired",
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
//         "-ResourceParameter-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ResourceParameter-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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
//         "-ResourceParameter-type": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     local_id: "ResourceParameter-type",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "r#type",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ItemDefinition",
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
//                     "A_type_resourceParameter",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ResourceParameter",
//     table_name: "bpmn_20_resource_parameter",
//     model_name: "ResourceParameter",
//     full_name: "bpmn_20_class_resource_parameter",
// }

