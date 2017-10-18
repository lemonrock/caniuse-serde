// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


use ::serde::de;
use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
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


include!("RegionalUsage.rs");
