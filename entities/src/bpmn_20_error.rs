//! bpmn_20_class_error

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_error")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
    /// COMPLEX FIELD : BPMN20-Error-structureRef
    pub structure_ref: Option<i64>,
    /// SIMPLE FIELD : BPMN20-Error-errorCode
    pub error_code: std::string::String,
    /// SIMPLE FIELD : BPMN20-Error-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Error need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
}

// SUPER : ONE Error need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

// ManyToMany : with Operation using A_errorRefs_operation
impl Related<super::bpmn_20_a_error_refs_operation::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_error_refs_operation::Relation::Operation.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_error_refs_operation::Relation::Error
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Error" (bpmn_20_class_error)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __error_code__ (xmi_id : "BPMN20-Error-errorCode")
    ///   * type : __std::string::String__
    /// * __name__ (xmi_id : "BPMN20-Error-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __ItemDefinition__ (__ItemDefinitionModel__) from A_structureRef_error
    ///   * one-to-many link : (0-1) __Error__ need (0-inf) __ItemDefinition__)
    ///   * callable using find_with_related(__ItemDefinitionModel__) from __Error__
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __Error__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Error__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Error" (bpmn_20_class_error)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __error_code__ (xmi_id : "BPMN20-Error-errorCode")
  * type : __std::string::String__
* __name__ (xmi_id : "BPMN20-Error-name")
  * type : __std::string::String__


## Relation : One To Many :
* __ItemDefinition__ (__ItemDefinitionModel__) from A_structureRef_error
  * one-to-many link : (0-1) __Error__ need (0-inf) __ItemDefinition__)
  * callable using find_with_related(__ItemDefinitionModel__) from __Error__

## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __Error__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Error__
  * saved in __super_root_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdReference {
//         object_id: "Error",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "Error",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Error-errorCode": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Error-errorCode",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "errorCode",
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
//         "-Error-name": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Error-name",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "name",
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
//         "-Error-structureRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdReference {
//                     object_id: "Error-structureRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "structureRef",
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
//                     "A_structureRef_error",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Error",
//     table_name: "bpmn_20_error",
//     model_name: "Error",
//     full_name: "bpmn_20_class_error",
// }

