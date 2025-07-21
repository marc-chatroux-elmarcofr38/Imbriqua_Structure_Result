//! bpmndi_class_bpmn_label

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmndi_bpmn_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Label
    pub super_label: i64,
    /// COMPLEX FIELD : BPMNDI-BPMNLabel-labelStyle
    pub label_style: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE BpmnLabel need ONE Label
    #[sea_orm(
        belongs_to = "super::di_label::Entity",
        from = "Column::SuperLabel",
        to = "super::di_label::Column::Id",
        on_delete = "Cascade"
    )]
    Label,
}

// SUPER : ONE BpmnLabel need ONE Label
impl Related<super::di_label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Label.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "BpmnLabel" (bpmndi_class_bpmn_label)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __Label__ (__LabelModel__)
    ///   * one-to-one link : one __BpmnLabel__ need one __Label__)
    ///   * callable using find_also_related(__LabelModel__) from __BpmnLabel__
    ///   * saved in __super_label__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "BpmnLabel" (bpmndi_class_bpmn_label)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__




## Direct Super :
* __Label__ (__LabelModel__)
  * one-to-one link : one __BpmnLabel__ need one __Label__)
  * callable using find_also_related(__LabelModel__) from __BpmnLabel__
  * saved in __super_label__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "BPMNLabel",
//         package_id: "BPMNDI",
//         is_set: true,
//     },
//     name: "BPMNLabel",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [
//         HRefClass(
//             HRefClass {
//                 href: "RefCell of 'DI-Label' (loaded : true)",
//             },
//         ),
//     ],
//     owned_attribute: {
//         "BPMNLabel-labelStyle": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "BPMNLabel-labelStyle",
//                     package_id: "BPMNDI",
//                     is_set: true,
//                 },
//                 name: "labelStyle",
//                 visibility: Public,
//                 simple_type: Some(
//                     "BPMNLabelStyle",
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
//                     "A_labelStyle_label",
//                 ),
//                 redefined_property_link: Some(
//                     Property(
//                         HRefRedefinedProperty {
//                             href: "RefCell of 'DI-DiagramElement-style' (loaded : true)",
//                         },
//                     ),
//                 ),
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMNDI.cmof#BPMNLabel",
//     table_name: "bpmndi_bpmn_label",
//     model_name: "BpmnLabel",
//     full_name: "bpmndi_class_bpmn_label",
// }

