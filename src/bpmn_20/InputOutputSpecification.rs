
/// Conversion of InputOutputSpecification (Class : InputOutputSpecification)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InputOutputSpecification",
///     name: "InputOutputSpecification",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputSpecification-inputSets",
///                 name: "inputSets",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
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
///                 association: "A_inputSets_inputOutputSpecification",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputSpecification-outputSets",
///                 name: "outputSets",
///                 visibility: Public,
///                 simple_type: Some(
///                     "OutputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
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
///                 association: "A_outputSets_inputOutputSpecification",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputSpecification-dataInputs",
///                 name: "dataInputs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "DataInput",
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
///                 association: "A_dataInputs_inputOutputSpecification",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "InputOutputSpecification-dataOutputs",
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
///                 association: "A_dataOutputs_inputOutputSpecification",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct InputOutputSpecification<'a> {
    #[builder(setter(into))]
    pub input_sets: Vec<&'a InputSet<'a>>,
    #[builder(setter(into))]
    pub output_sets: Vec<&'a OutputSet<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub data_inputs: Option<Vec<&'a DataInput<'a>>>,
    #[builder(setter(into, strip_option), default)]
    pub data_outputs: Option<Vec<&'a DataOutput<'a>>>,
}

