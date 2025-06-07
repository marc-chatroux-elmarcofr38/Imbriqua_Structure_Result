
/// Conversion of CatchEvent (Class : CatchEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CatchEvent",
///     name: "CatchEvent",
///     is_abstract: true,
///     super_class: Some(
///         "Event",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-parallelMultiple",
///                 name: "parallelMultiple",
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
///                 xmi_id: "CatchEvent-outputSet",
///                 name: "outputSet",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_outputSet_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-eventDefinitionRefs",
///                 name: "eventDefinitionRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_eventDefinitionRefs_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-dataOutputAssociation",
///                 name: "dataOutputAssociation",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutputAssociation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataOutputAssociation_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-dataOutputs",
///                 name: "dataOutputs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataOutput",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_dataOutputs_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "CatchEvent-eventDefinitions",
///                 name: "eventDefinitions",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventDefinition",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_eventDefinitions_catchEvent",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct CatchEvent<'a> {
    #[builder(setter(into))]
    pub parallel_multiple: dc::Boolean,
    #[builder(setter(into, strip_option), default)]
    pub output_set: Option<&'a OutputSet<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub event_definition_refs: Option<Vec<&'a EventDefinition<'a>>>,
    #[builder(setter(into, strip_option), default)]
    pub data_output_association: Option<Vec<&'a DataOutputAssociation<'a>>>,
    #[builder(setter(into, strip_option), default)]
    pub data_outputs: Option<Vec<&'a DataOutput<'a>>>,
    #[builder(setter(into, strip_option), default)]
    pub event_definitions: Option<Vec<&'a EventDefinition<'a>>>,
}

