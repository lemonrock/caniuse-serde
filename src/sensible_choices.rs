// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Obtain (CanIUse, AgentNameAndVersionSet) for, say, `autoprefix_stylesheet()` in the `css-autoprefix` crate.
#[inline(always)]
pub fn sensible_choices(maximum_release_age_from_can_i_use_database_last_updated_in_weeks: u16, minimum_usage_threshold: UsagePercentage, regional_usages: &[&RegionalUsage]) -> (CanIUse, AgentNameAndVersionSet)
{
	let can_i_use = CanIUse::default();
	let maximum_release_age_from_can_i_use_database_last_updated = ::chrono::Duration::weeks(maximum_release_age_from_can_i_use_database_last_updated_in_weeks as i64);
	
	let choices = AgentNameAndVersionSet::a_sensible_set_of_choices_for_an_international_website_in_multiple_languages(&can_i_use, maximum_release_age_from_can_i_use_database_last_updated, minimum_usage_threshold, &regional_usages);
	
	(can_i_use, choices)
}
