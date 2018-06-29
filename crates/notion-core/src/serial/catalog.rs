use super::super::catalog;

use std::collections::BTreeSet;
use std::default::Default;
use std::iter::FromIterator;
use std::string::ToString;
use std::marker::PhantomData;

use notion_fail::{Fallible, ResultExt};

use semver::{SemVerError, Version};

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    #[serde(default)]
    node: NodeCollection,
    #[serde(default)]
    yarn: YarnCollection,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "node")]
pub struct NodeCollection{
    activated: Option<String>,
    versions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "yarn")]
pub struct YarnCollection{
    activated: Option<String>,
    versions: Vec<String>,
}

impl Default for NodeCollection {
    fn default() -> Self {
        NodeCollection {
            activated: None,
            versions: vec![],
        }
    }
}

impl Default for YarnCollection {
    fn default() -> Self {
        YarnCollection {
            activated: None,
            versions: vec![],
        }
    }
}

impl Catalog {
    pub fn into_catalog(self) -> Fallible<catalog::Catalog> {
        Ok(catalog::Catalog {
            node: self.node.into_node_collection().unknown()?,
            yarn: self.yarn.into_yarn_collection().unknown()?,
        })
    }
}

impl NodeCollection {
    fn into_node_collection(self) -> Fallible<catalog::NodeCollection> {
        let activated = match self.activated {
            Some(v) => Some(Version::parse(&v[..]).unknown()?),
            None => None,
        };

        let versions: Result<Vec<Version>, SemVerError> = self.versions
            .into_iter()
            .map(|s| Ok(Version::parse(&s[..])?))
            .collect();

        Ok(catalog::NodeCollection {
            activated: activated,
            versions: BTreeSet::from_iter(versions.unknown()?),
            phantom: PhantomData
        })
    }
}

impl YarnCollection {
    fn into_yarn_collection(self) -> Fallible<catalog::YarnCollection> {
        let activated = match self.activated {
            Some(v) => Some(Version::parse(&v[..]).unknown()?),
            None => None,
        };

        let versions: Result<Vec<Version>, SemVerError> = self.versions
            .into_iter()
            .map(|s| Ok(Version::parse(&s[..])?))
            .collect();

        Ok(catalog::YarnCollection {
            activated,
            versions: BTreeSet::from_iter(versions.unknown()?),
            phantom: PhantomData
        })
    }
}

impl catalog::Catalog {
    pub fn to_serial(&self) -> Catalog {
        Catalog {
            node: self.node.to_serial(),
            yarn: self.yarn.to_serial(),
        }
    }
}
impl catalog::NodeCollection {
    fn to_serial(&self) -> NodeCollection {
        NodeCollection {
            activated: self.activated.clone().map(|v| v.to_string()),
            versions: self.versions.iter().map(|v| v.to_string()).collect(),
        }
    }
}

impl catalog::YarnCollection {
    fn to_serial(&self) -> YarnCollection {
        YarnCollection {
            activated: self.activated.clone().map(|v| v.to_string()),
            versions: self.versions.iter().map(|v| v.to_string()).collect(),
        }
    }
}
