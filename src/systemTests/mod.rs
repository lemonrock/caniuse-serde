// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


use super::*;


#[test]
fn can_i_use_default()
{
	CanIUse::default();
}

#[test]
fn regional_usage_default()
{
	RegionalUsage::default();
}

#[test]
fn sensible_rules_to_prefixes()
{
	let can_i_use = CanIUse::default();
	let maximum_release_age_from_can_i_use_database_last_updated = Duration::weeks(54 + 12); // Firefox ESR release cycle + 12 weeks (2x cycles overlap)
	let minimum_usage_threshold = UsagePercentage::OnePerMille;
	let regional_usages = vec!
	[
		Asia.deref(),
		Europe.deref(),
		NorthAmerica.deref(),
		SouthAmerica.deref(),
		AU.deref(),
		NZ.deref(),
	];
	
	let choices = AgentNameAndVersionSet::a_sensible_set_of_choices_for_an_international_website_in_multiple_languages(&can_i_use, maximum_release_age_from_can_i_use_database_last_updated, minimum_usage_threshold, &regional_usages);
	
	let feature_name = FeatureName("css-focus-ring".to_owned());
	let mut unique_prefixes = HashSet::new();
	choices.support_for_a_feature(&can_i_use, &feature_name, |agent, version, support|
	{
		if support.requires_prefix()
		{
			unique_prefixes.insert(agent.prefix(version).clone());
		}
	});
	
	assert!(unique_prefixes.contains(&Prefix::moz));
	assert_eq!(unique_prefixes.len(), 1);
}
