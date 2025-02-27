pub(crate) const ALL: &[SupportedVersion] = &[
	SupportedVersion::V1_18,
	SupportedVersion::V1_19,
	SupportedVersion::V1_20,
	SupportedVersion::V1_21,
	SupportedVersion::V1_22,
	SupportedVersion::V1_23,
	SupportedVersion::V1_24,
	SupportedVersion::V1_25,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
	V1_18,
	V1_19,
	V1_20,
	V1_21,
	V1_22,
	V1_23,
	V1_24,
	V1_25,
}

impl SupportedVersion {
	pub(crate) fn name(self) -> &'static str {
		match self {
			SupportedVersion::V1_18 => "1.18",
			SupportedVersion::V1_19 => "1.19",
			SupportedVersion::V1_20 => "1.20",
			SupportedVersion::V1_21 => "1.21",
			SupportedVersion::V1_22 => "1.22",
			SupportedVersion::V1_23 => "1.23",
			SupportedVersion::V1_24 => "1.24",
			SupportedVersion::V1_25 => "1.25",
		}
	}

	pub(crate) fn mod_root(self) -> &'static str {
		match self {
			SupportedVersion::V1_18 => "v1_18",
			SupportedVersion::V1_19 => "v1_19",
			SupportedVersion::V1_20 => "v1_20",
			SupportedVersion::V1_21 => "v1_21",
			SupportedVersion::V1_22 => "v1_22",
			SupportedVersion::V1_23 => "v1_23",
			SupportedVersion::V1_24 => "v1_24",
			SupportedVersion::V1_25 => "v1_25",
		}
	}

	pub(crate) fn spec_url(self) -> &'static str {
		match self {
			SupportedVersion::V1_18 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.18.20/api/openapi-spec/swagger.json",
			SupportedVersion::V1_19 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.19.16/api/openapi-spec/swagger.json",
			SupportedVersion::V1_20 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.20.15/api/openapi-spec/swagger.json",
			SupportedVersion::V1_21 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.21.14/api/openapi-spec/swagger.json",
			SupportedVersion::V1_22 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.22.15/api/openapi-spec/swagger.json",
			SupportedVersion::V1_23 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.23.12/api/openapi-spec/swagger.json",
			SupportedVersion::V1_24 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.24.6/api/openapi-spec/swagger.json",
			SupportedVersion::V1_25 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.25.2/api/openapi-spec/swagger.json",
		}
	}

	pub(crate) fn fixup(self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		#[allow(clippy::match_same_arms)]
		let upstream_bugs_fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = match self {
			SupportedVersion::V1_18 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::containerimage,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_19 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::containerimage,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_20 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::containerimage,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_21 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::containerimage,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_22 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_23 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_24 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_25 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],
		};

		let special_fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = &[
			crate::fixups::special::json_ty::json_schema_props_or_array,
			crate::fixups::special::json_ty::json_schema_props_or_bool,
			crate::fixups::special::json_ty::json_schema_props_or_string_array,
			crate::fixups::special::create_delete_optional,
			crate::fixups::special::create_optionals,
			crate::fixups::special::patch,
			crate::fixups::special::remove_delete_collection_operations_query_parameters,
			crate::fixups::special::remove_delete_operations_query_parameters,
			crate::fixups::special::remove_read_operations_query_parameters,
			crate::fixups::special::separate_watch_from_list_operations,
			crate::fixups::special::watch_event,
			crate::fixups::special::list, // Must run after separate_watch_from_list_operations
			crate::fixups::special::response_types,
			crate::fixups::special::resource_metadata_not_optional,
		];

		for fixup in upstream_bugs_fixups.iter().chain(special_fixups) {
			fixup(spec)?;
		}

		Ok(())
	}
}
