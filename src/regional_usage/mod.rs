// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


use super::*;
use ::chrono::naive::NaiveDate;
use ::std::collections::BTreeMap;
use ::std::collections::HashMap;
use ::std::collections::btree_map::Range;
use ::std::collections::Bound;
use ::std::fs::File;
use ::std::io::Read;
use ::std::path::Path;
use ::std::str::FromStr;


include!("RegionalUsage.rs");
include!("YearMonth.rs");
