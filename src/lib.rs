// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of caniuse-serde, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


//! # caniuse-serde
//!
//! A Rust library crate for working with the <https://caniuse.com> database of browser (agent) features and regional usage data.
//! Comes with a version if the database embedded, and can also be used with external copies (JSON files).
//!
//!
//! ## Getting Going
//!
//!
//! ### To get started
//!
//! ```
//! extern crate caniuse_serde;
//! use ::caniuse_serde::{EmbeddedCanIUseDatabase, AgentName, FeatureName};
//! ```
//!
//!
//! ### To look up an agent's details
//!
//! ```
//! let agent = AgentName::MozillaFirefox.agent(EmbeddedCanIUseDatabase).unwrap();
//! ```
//!
//!
//! ### To look up a feature's details
//!
//! ```
//! let feature = "transform3d".into().feature(EmbeddedCanIUseDatabase).unwrap();
//! ```
//!
//! ## Regional Usage
//!
//! * Use the constants in the `regional_usage` module to get regional, continental and world-wide usage data.
//! * To replicate the functionality of 'browserlist', use the `query()` method on RegionalUsage.
//! * Or read below for a more useful approach.
//! * Use the enum `RegionalUsages` with the method `regional_usage()` to obtain a reference to an embedded RegionalUsage database.
//!
//!
//! ## A strategy for using the caniuse database with [browserlist](https://github.com/ai/browserslist) like behaviour
//! The concept of version is differently understood by the browser vendors (eg IE vs Chrome, say), and so just saying 'last 2 versions' isn't particularly useful.
//! In practice, a combination of selection rules is needed to identify a set of browser and browser versions to support, using the data in the database. These selection rules are likely to be stable for months and years, but not in the long term.
//!
//! I've identified my own selection rules for a professional, international consultant's website written in English with translations to Spanish, French and Italian. I've added this as code to this crate to make sure that the API I've written around the caniuse.com database is actually usable.
//!
//!
//! ### To make use of my choices
//!
//! The quickest way is with either `sensible_choices()` or `sensible_choices_default()`:-
//!
//! ```
//! extern crate caniuse_serde;
//! use ::caniuse_serde::*;
//! use ::caniuse_serde::regional_usage::*;
//!
//! let (can_i_use, choices) = sensible_choices_default();
//!
//! let feature_name = FeatureName("css-focus-ring".to_owned());
//! let mut unique_prefixes = HashSet::new();
//! choices.support_for_a_feature(&can_i_use, &feature_name, |agent, version, support| {
//! 	if support.requires_prefix() {
//! 		unique_prefixes.insert(agent.prefix(version).clone());
//! 	}
//! });
//!
//! assert!(unique_prefixes.contains(&Prefix::moz));
//! assert_eq!(unique_prefixes.len(), 1);
//! ```
//!
//!
//! ### My selection rules
//! 1. Obsolete Browsers still in use
//! 	- We need to support the last version of these until its percentage usage falls below X%
//! 	- The percentage usage should be for a sub-set of the world (ie target audience continents or countries)
//! 	- These browsers are:-
//!			- IE (at version 11)
//! 		- Blackberry Browser (at version 10)
//! 		- IE Mobile (MS has dropped Windows Phone)
//! 2. Browsers with major change of rendering engine
//! 	- This effectively makes the last version with the old rendering engine obsolete
//! 	- Rules as for Obsolete Browsers, but selection needs to be aware that there are 'later' versions
//! 	- These browsers are:-
//!  		- Android Browser (at 4.4.4)
//!			- Opera with Presto
//! 3. Automatically Updated Browsers
//! 	- These browsers have short-lived, sub-yearly versions
//! 	- They are probably best discovered by matching for all released versions after a specific release date (eg 2 years ago)
//! 	- Using a percentage isn't wise as usage of each version will change rapidly (from near zero to a few percentage points, then to near zero again), and certainly likely to change more rapidly than static website rebuilds
//! 	- These browsers are:-
//! 		- Firefox
//! 		- Safari
//! 		- Microsoft Edge
//! 		- Chrome
//! 		- Opera with Webkit Rendering Engine
//! 4. Long-Term Releases of Automatically Updated Browsers
//! 	- These browsers have occasional long-term releases which are intended to be supported for a year or more
//! 	- Usage percentages for these may be very low globally, and they may be 9 or more release versions 'out-of-date', but they represent an important audience
//! 	- In practice the length of time each long term release is supported for changes with each release, even though vendors have 'long term release policies'
//!		- This is because policies change in the long interval between long-term releases
//! 	- These browsers are problematic to identify as the caniuse.com database omits them
//! 	- Some long-term release versions differ slightly in supported features, particularly those of a more experimental nature, to their related short-term release cousins (even though they may share the same major version number)
//!		- For Firefox, ESR releases are supposedly for one year (actually, 54 weeks, '9-cycles', with a 12-week ('2-cycle') overlap between releases (a cycle is a Firefox release cycle, typically 6 weeks), but, as always for these sorts of releases, the policy has changed several times.
//! 	- These browsers are:-
//! 		- Firefox
//! 5. Regionally significant, occasionally automatically updated browsers
//! 	- Support of these browsers is particularly important for the Indian and Asian markets
//! 	- Many cheaper smart phones come with them (I've used them, too)
//! 	- Vendors frequently don't upgrade old firmware installed versions and some older versions may persist and have higher usage for some time than newer ones
//! 	- All of them currently are just more dated versions of the Webkit rendering engine than Chrome
//! 	- These browsers are probably best supported with a 'above X% rule', where X is for any version
//! 	- These browsers are:-
//! 		- UC Browser
//! 		- Samsung Internet
//! 		- QQ Browser
//! 		- Baidu Browser
//! 6. Very different from mainstream and unsupportable
//! 	- Opera Mini is an excellent product, but unless one is explicitly targeting its users' demographic, it is not useful to support
//! 	- If one is targeting its users demographic, its lack of modern features (making it the lowest common denominator) means website development would not make use of caniuse.com data; it's too different.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(unstable_features)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![recursion_limit="1024"]


extern crate chrono;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate url_serde;


use self::regional_usage::*;
use ::chrono::Duration;
use ::chrono::prelude::*;
use ::serde::de;
use ::serde::de::Deserialize;
use ::serde::de::DeserializeSeed;
use ::serde::de::Deserializer;
use ::serde::de::MapAccess;
use ::serde::de::SeqAccess;
use ::serde::de::Visitor;
use ::std::collections::Bound;
use ::std::collections::BTreeMap;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::collections::hash_map::Keys;
use ::std::collections::btree_map::Range;
use ::std::cmp::Ordering;
use ::std::cmp::Eq;
use ::std::cmp::Ord;
use ::std::cmp::PartialEq;
use ::std::cmp::PartialOrd;
use ::std::fmt;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::io::Read;
use ::std::iter::DoubleEndedIterator;
use ::std::iter::ExactSizeIterator;
use ::std::iter::Iterator;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Deref;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
use ::std::path::Path;
use ::std::str::FromStr;
use ::url::Url;


#[cfg(test)] mod systemTests;

/// Support for Agent regional, continental and world-wide usage by version.
/// Use the `RegionalUsages` enum preferably.
pub mod regional_usage;



include!("Agent.rs");
include!("AgentDetail.rs");
include!("AgentName.rs");
include!("AgentNameIterator.rs");
include!("AgentNameAndVersionSet.rs");
include!("AgentType.rs");
include!("Bug.rs");
include!("CanIUse.rs");
include!("Category.rs");
include!("Feature.rs");
include!("FeatureDetail.rs");
include!("FeatureName.rs");
include!("FeatureNameIterator.rs");
include!("Link.rs");
include!("ParentCategory.rs");
include!("ParentCategoryIterator.rs");
include!("Prefix.rs");
include!("PrefixVisitor.rs");
include!("sensible_choices.rs");
include!("sensible_choices_default.rs");
include!("Status.rs");
include!("StatusIterator.rs");
include!("Support.rs");
include!("SupportDetail.rs");
include!("SupportRangeIterator.rs");
include!("SupportMaturity.rs");
include!("UsagePercentage.rs");
include!("Version.rs");
include!("VersionDetail.rs");
include!("VersionPart.rs");
