#![deny(unused)]

//! These fixups correspond to bugs in the upstream swagger spec.

// Path operation annotated with a "x-kubernetes-group-version-kind" that references a type that doesn't exist in the schema.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/66807
#[allow(clippy::if_same_then_else)]
pub(crate) fn connect_options_gvk(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	for operation in &mut spec.operations {
		if let Some(kubernetes_group_kind_version) = &mut operation.kubernetes_group_kind_version {
			if kubernetes_group_kind_version.group.is_empty() && kubernetes_group_kind_version.version == "v1" {
				let kind = &mut kubernetes_group_kind_version.kind;
				if &*kind == "NodeProxyOptions" {
					*kind = "Node".to_string();
					found = true;
				}
				else if &*kind == "PodAttachOptions" {
					*kind = "Pod".to_string();
					found = true;
				}
				else if &*kind == "PodExecOptions" {
					*kind = "Pod".to_string();
					found = true;
				}
				else if &*kind == "PodPortForwardOptions" {
					*kind = "Pod".to_string();
					found = true;
				}
				else if &*kind == "PodProxyOptions" {
					*kind = "Pod".to_string();
					found = true;
				}
				else if &*kind == "ServiceProxyOptions" {
					*kind = "Service".to_string();
					found = true;
				}
			}
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied connect options kubernetes_group_kind_version override".into())
	}
}

// Path operation "connectCoreV1GetNamespacedPodExec" claims to take `command: string` parameter in the query string.
// This is not a single string but an array of strings, encoded as multiple query string parameters.
pub(crate) fn pod_exec_command_parameter_type(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	if let Some(operation) = spec.operations.iter_mut().find(|operation| operation.id == "connectCoreV1GetNamespacedPodExec") {
		if let Some(parameter) = operation.parameters.iter_mut().find(|p| p.name == "command") {
			if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::String { format: None }) = parameter.schema.kind {
				*parameter = std::sync::Arc::new(crate::swagger20::Parameter {
					location: parameter.location,
					name: parameter.name.clone(),
					required: parameter.required,
					schema: crate::swagger20::Schema {
						kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Array {
							items: Box::new(crate::swagger20::Schema {
								description: None,
								kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::String { format: None }),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
								impl_deep_merge: true,
							}),
						}),
						..(parameter.schema.clone())
					},
				});

				found = true;
			}
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied connectCoreV1GetNamespacedPodExec command parameter type override".into())
	}
}

// The spec says that this property is an array, but it can be null.
//
// Override it to be optional to achieve the same effect.
pub(crate) mod optional_properties {
	// `ContainerImage::names`
	//
	// Ref: https://github.com/kubernetes/kubernetes/issues/93606
	pub(crate) fn containerimage(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.ContainerImage".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
				if let Some(property) = properties.get_mut(&crate::swagger20::PropertyName("names".to_string())) {
					if property.1 {
						property.1 = false;
						return Ok(());
					}
				}
			}
		}

		Err("never applied ContainerImage optional properties override".into())
	}
}
