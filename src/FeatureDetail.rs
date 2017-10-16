// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone)]
pub struct FeatureDetail
{
	#[serde(default)] title: String,
	#[serde(default)] description: String,
	#[serde(with = "url_serde", rename = "spec")] specification_url: Url,
	#[serde(default)] status: Status,
	#[serde(default)] links: Vec<Link>,
	#[serde(default)] bugs: Vec<Bug>,
	#[serde(default)] categories: Vec<Category>,
	#[serde(default, rename = "stats")] implementations_by_agents: HashMap<AgentName, BTreeMap<Version, SupportDetail>>,
	#[serde(default)] notes: String,
	#[serde(default)] notes_by_num: BTreeMap<u8, String>,
	#[serde(default, deserialize_with = "FeatureDetail::deserialize_parent")] parent: Option<FeatureName>,
	#[serde(default, rename="usage_perc_y")] usage_y: UsagePercentage,
	#[serde(default, rename="usage_perc_a")] usage_a: UsagePercentage,
	#[serde(default, rename="ucprefix")] upper_case_prefix: bool,
	#[serde(default, deserialize_with = "FeatureDetail::deserialize_comma_separated_strings")] keywords: Vec<String>,
	#[serde(default, deserialize_with = "FeatureDetail::deserialize_comma_separated_strings", rename="ie_id")] internet_explorer_feature_identifiers: Vec<String>,
	#[serde(default, deserialize_with = "FeatureDetail::deserialize_comma_separated_strings", rename="chrome_id")] blink_feature_identifiers: Vec<String>,
	#[serde(default, deserialize_with = "FeatureDetail::deserialize_comma_separated_strings", rename="firefox_id")] firefox_feature_identifiers: Vec<String>,
	#[serde(default, deserialize_with = "FeatureDetail::deserialize_comma_separated_strings", rename="webkit_id")] webkit_feature_identifiers: Vec<String>,
	#[serde(default = "FeatureDetail::shown_default")] shown: bool,
}

impl FeatureDetail
{
	#[inline(always)]
	fn deserialize_parent<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<FeatureName>, D::Error>
	{
		struct ParentVisitor;
		
		impl<'de> Visitor<'de> for ParentVisitor
		{
			type Value = Option<FeatureName>;
			
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a string which contains comma separated sub-strings")
			}
			
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				if v.is_empty()
				{
					Ok(None)
				}
				else
				{
					Ok(Some(FeatureName(v.to_owned())))
				}
			}
			
			fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E>
			{
				if v.is_empty()
				{
					Ok(None)
				}
				else
				{
					Ok(Some(FeatureName(v)))
				}
			}
		}
		
		deserializer.deserialize_str(ParentVisitor)
	}
	
	#[inline(always)]
	fn deserialize_comma_separated_strings<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<String>, D::Error>
	{
		struct CommaSeparatedStringsVisitor;
		
		impl<'de> Visitor<'de> for CommaSeparatedStringsVisitor
		{
			type Value = Vec<String>;
			
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a string which contains comma separated sub-strings")
			}
			
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				if v.is_empty()
				{
					return Ok(vec![]);
				}
				
				let mut strings = Vec::with_capacity(16);
				for string in v.split(',')
				{
					let trimmed = string.trim();
					if !trimmed.is_empty()
					{
						strings.push(trimmed.to_owned());
					}
				}
				strings.shrink_to_fit();
				Ok(strings)
			}
		}
		
		deserializer.deserialize_str(CommaSeparatedStringsVisitor)
	}
	
	#[inline(always)]
	fn shown_default() -> bool
	{
		true
	}
}
