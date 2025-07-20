//! bpmndi_class_bpmn_label_style

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_label_style")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Style
    pub super_style: i64,
    /// COMPLEX FIELD : BPMNDI-BPMNLabelStyle-font
    pub font: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnLabelStyle need ONE Style
    #[sea_orm(
        belongs_to = "super::di_style::Entity",
        from = "Column::SuperStyle",
        to = "super::di_style::Column::Id",
        on_delete = "Cascade"
    )]
    Style,
}

// SUPER : ONE BpmnLabelStyle need ONE Style
impl Related<super::di_style::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Style.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "BpmnLabelStyle" (bpmndi_class_bpmn_label_style)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Style__ (__StyleModel__)
    ///   * one-to-one link : one __BpmnLabelStyle__ need one __Style__)
    ///   * callable using find_also_related(__StyleModel__) from __BpmnLabelStyle__
    ///   * saved in __super_style__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "BpmnLabelStyle" (bpmndi_class_bpmn_label_style)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Style__ (__StyleModel__)
  * one-to-one link : one __BpmnLabelStyle__ need one __Style__)
  * callable using find_also_related(__StyleModel__) from __BpmnLabelStyle__
  * saved in __super_style__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMNDI-BPMNLabelStyle" (loaded : false)",
//     name: "BPMNLabelStyle",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         HRefClass(
//             HRefClass {
//                 href: "Weak ref of "DI-Style" (loaded : false)",
//             },
//         ),
//     ],
//     owned_attribute: {
//         "-BPMNLabelStyle-font": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMNDI-BPMNLabelStyle-font" (loaded : false)",
//                 name: "font",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefDataType(
//                         HRefDataType {
//                             href: "Weak ref of "DC-Font" (loaded : false)",
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
//     technical_name: "BPMNDI.cmof#BPMNLabelStyle",
//     table_name: "bpmndi_bpmn_label_style",
//     model_name: "BpmnLabelStyle",
//     full_name: "bpmndi_class_bpmn_label_style",
// }

