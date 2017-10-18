// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone)]
struct AgentDetail
{
	#[serde(rename = "browser")] name: String,
	#[serde(rename = "abbr")] abbreviated_name: String,
	prefix: Prefix,
	#[serde(rename = "type")] agent_type: AgentType,
	usage_global: BTreeMap<Version, UsagePercentage>,
	#[serde(rename = "versions")] eras_to_versions: Vec<Option<Version>>,
	#[serde(default)] prefix_exceptions: HashMap<Version, Prefix>
}
