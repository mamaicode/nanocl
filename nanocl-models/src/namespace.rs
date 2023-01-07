#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Namespace {
  pub name: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct NamespacePartial {
  pub name: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct NamespaceSummary {
  pub name: String,
  pub cargoes: i64,
  pub instances: i64,
}
