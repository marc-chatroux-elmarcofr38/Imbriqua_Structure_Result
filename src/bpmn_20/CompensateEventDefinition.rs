
/// Conversion of CompensateEventDefinition (Class : CompensateEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CompensateEventDefinition",
///     name: "CompensateEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CompensateEventDefinition-waitForCompletion",
///                 name: "waitForCompletion",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     PrimitiveTypeLink(
///                         PrimitiveTypeLink {
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CompensateEventDefinition-activityRef",
///                 name: "activityRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Activity",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_activityRef_compensateEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct CompensateEventDefinition<'a> {
    #[builder(setter(into))]
    pub wait_for_completion: dc::Boolean,
    #[builder(setter(into, strip_option), default)]
    pub activity_ref: Option<&'a Activity<'a>>,
}

