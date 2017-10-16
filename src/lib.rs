// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of caniuse-serde, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]


///! To get started:-
///! `extern crate caniuse_serde;`
///! `use ::caniuse_serde::{EmbeddedCanIUseDatabase, AgentName, FeatureName}`
///! Look up an agent's details:-
///! let agent: AgentName::MozillaFirefox.agent(EmbeddedCanIUseDatabase).unwrap();
///! Look up a feature's details:-
///! let feature: "transform3d".into().feature(EmbeddedCanIUseDatabase).unwrap();


extern crate chrono;
#[macro_use] extern crate lazy_static;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate url_serde;


use ::chrono::prelude::*;
use ::serde::de;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::MapAccess;
use ::serde::de::Visitor;
use ::std::collections::BTreeMap;
use ::std::collections::HashMap;
use ::std::collections::btree_map::Range;
use ::std::collections::Bound;
use ::std::cmp::Ordering;
use ::std::cmp::Eq;
use ::std::cmp::Ord;
use ::std::cmp::PartialEq;
use ::std::cmp::PartialOrd;
use ::std::fmt;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::LowerExp;
use ::std::fmt::UpperExp;
use ::std::fs::File;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::io::Read;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Deref;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
use ::std::path::Path;
use ::std::str::FromStr;
use ::url::Url;


#[cfg(test)] mod systemTests;


include!("Agent.rs");
include!("AgentDetail.rs");
include!("AgentName.rs");
include!("AgentType.rs");
include!("Bug.rs");
include!("CanIUse.rs");
include!("Category.rs");
include!("Eras.rs");
include!("EraName.rs");
include!("Feature.rs");
include!("FeatureDetail.rs");
include!("FeatureName.rs");
include!("Link.rs");
include!("ParentCategory.rs");
include!("Prefix.rs");
include!("RegionalUsage.rs");
include!("Status.rs");
include!("Support.rs");
include!("SupportDetail.rs");
include!("SupportMaturity.rs");
include!("UsagePercentage.rs");
include!("Version.rs");
include!("VersionPart.rs");
