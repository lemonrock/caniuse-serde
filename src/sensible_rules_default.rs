// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Obtain (CanIUse, AgentNameAndVersionSet) for, say, `autoprefix_stylesheet()` in the `css-autoprefix` crate.
#[inline(always)]
pub fn sensible_rules_default() -> (CanIUse, AgentNameAndVersionSet)
{
	use ::regional_usage::RegionalUsages::*;
	
	let maximum_release_age_from_can_i_use_database_last_updated = 54 + 12; // Firefox ESR release cycle + 12 weeks (2x cycles overlap)
	let minimum_usage_threshold = UsagePercentage::OnePerMille;
	let regional_usages = vec!
	[
		Asia.regional_usage(),
		Europe.regional_usage(),
		NorthAmerica.regional_usage(),
		SouthAmerica.regional_usage(),
		AU.regional_usage(),
		NZ.regional_usage(),
	];
	sensible_rules(maximum_release_age_from_can_i_use_database_last_updated, minimum_usage_threshold, &regional_usages)
}
