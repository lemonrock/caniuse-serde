// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// A feature is a HTML, CSS or like feature that agents may not have support for.
#[derive(Debug, Clone)]
pub struct Feature<'a>
{
	feature_name: &'a FeatureName,
	feature_detail: &'a FeatureDetail,
}

impl<'a> Feature<'a>
{
	/// The name of this feature.
	#[inline(always)]
	pub fn feature_name(&self) -> &'a FeatureName
	{
		self.feature_name
	}
	
	/// The title of this feature.
	#[inline(always)]
	pub fn title(&self) -> &'a str
	{
		&self.feature_detail.title
	}
	
	/// The description of this feature.
	#[inline(always)]
	pub fn description(&self) -> &'a str
	{
		&self.feature_detail.description
	}
	
	/// The URL at which the specification of this feature can be found.
	#[inline(always)]
	pub fn specification_url(&self) -> &'a Url
	{
		&self.feature_detail.specification_url
	}
	
	/// The status of this feature.
	#[inline(always)]
	pub fn status(&self) -> &'a Status
	{
		&self.feature_detail.status
	}
	
	/// Links to additional documents detailing this feature or aspects of it.
	#[inline(always)]
	pub fn links(&self) -> &'a [Link]
	{
		&self.feature_detail.links[..]
	}
	
	/// Any bugs with this feature. Rarely used in the caniuse.com database.
	#[inline(always)]
	pub fn bugs(&self) -> &'a [Bug]
	{
		&self.feature_detail.bugs[..]
	}
	
	/// The caniuse.com database's categorisations of this feature.
	#[inline(always)]
	pub fn categories(&self) -> &'a [Category]
	{
		&self.feature_detail.categories[..]
	}
	
	/// The caniuse.com database's notes on this feature.
	#[inline(always)]
	pub fn general_notes(&self) -> &'a str
	{
		&self.feature_detail.notes
	}
	
	/// implementations; returns None if agent_name has no known usages.
	#[inline(always)]
	pub fn implementations_by_agents(&'a self, agent_name: &AgentName, lower_bound: Bound<&Version>, upper_bound: Bound<&Version>) -> Option<SupportRangeIterator<'a>>
	{
		match self.feature_detail.implementations_by_agents.get(agent_name)
		{
			None => None,
			Some(entry) => Some
			(
				SupportRangeIterator
				{
					feature: self,
					range: entry.range((lower_bound, upper_bound)),
				}
			),
		}
	}
	
	/// implementation; returns None if agent_name has no known usages.
	/// returns Some(None) if agent_name exists but not for the version.
	/// returns Some(Some(support) if agent_name exists and the version has known support
	#[inline(always)]
	pub fn implementation(&'a self, agent_name: &AgentName, version: &Version) -> Option<Option<Support<'a>>>
	{
		match self.feature_detail.implementations_by_agents.get(agent_name)
		{
			None => None,
			Some(entry) =>
			{
				match entry.get(version)
				{
					None => Some(None),
					Some(support_detail) => Some(Some(Support
					{
						support_detail,
						feature: self,
					}))
				}
			}
		}
	}
	
	/// The supported usage of this feature; those agents where the feature is SupportMaturity::SupportedByDefault.
	#[inline(always)]
	pub fn supported_by_default_usage(&self) -> UsagePercentage
	{
		self.feature_detail.supported_by_default_usage
	}
	
	/// The supported usage of this feature; those agents where the feature is SupportMaturity::AlmostSupported.
	#[inline(always)]
	pub fn almost_supported_usage(&self) -> UsagePercentage
	{
		self.feature_detail.almost_supported_usage
	}
	
	/// The parent feature this one belongs to use; not widely used by the caniuse.com's database.
	#[inline(always)]
	pub fn parent_feature_if_any(&self) -> Option<&'a FeatureName>
	{
		self.feature_detail.parent.as_ref()
	}
	
	/// A list of keywords to make it easier to search for this feature.
	#[inline(always)]
	pub fn keywords(&self) -> &'a [String]
	{
		&self.feature_detail.keywords[..]
	}
	
	/// For Opera & Opera Mobile, assumes blink (but not for Opera Mini)
	/// For Opera Mini and Unknown browsers, returns an empty slice
	#[inline(always)]
	pub fn feature_identifiers(&self, agentName: &AgentName) -> &'a [String]
	{
		const NoNoneIdentifiers: [String; 0] = [];
		
		use self::AgentName::*;
		match *agentName
		{
			MicrosoftInternetExplorer => self.internet_explorer_feature_identifiers(),
			MicrosoftEdge => self.internet_explorer_feature_identifiers(),
			MozillaFirefox => self.firefox_feature_identifiers(),
			GoogleChrome => self.blink_feature_identifiers(),
			AppleSafari => self.webkit_feature_identifiers(),
			Opera => &self.feature_detail.blink_feature_identifiers[..],
			AppleSafariIOs => self.webkit_feature_identifiers(),
			OperaMini => &NoNoneIdentifiers[..],
			GoogleAndroidBrowserAndWebComponent => self.webkit_feature_identifiers(),
			Blackberry => self.webkit_feature_identifiers(),
			OperaMobile => self.blink_feature_identifiers(),
			GoogleChromeAndroid => self.blink_feature_identifiers(),
			MozillaFirefoxAndroid => self.firefox_feature_identifiers(),
			MicrosoftInternetExplorerMobile => self.internet_explorer_feature_identifiers(),
			UcBrowserAndroid => self.webkit_feature_identifiers(),
			SamsungBrowserAndroid => self.webkit_feature_identifiers(),
			QqBrowserAndroid => self.webkit_feature_identifiers(),
			BaiduBrowserAndroid => self.webkit_feature_identifiers(),
			Unknown(_) => &NoNoneIdentifiers[..],
			_ => &NoNoneIdentifiers[..],
		}
	}
	
	/// Identifiers to related MSDN sections.
	#[inline(always)]
	pub fn internet_explorer_feature_identifiers(&self) -> &'a [String]
	{
		&self.feature_detail.internet_explorer_feature_identifiers[..]
	}
	
	/// Identifiers to related blink (Google Chrome) bugs
	#[inline(always)]
	pub fn blink_feature_identifiers(&self) -> &'a [String]
	{
		&self.feature_detail.blink_feature_identifiers[..]
	}
	
	/// Identifiers to related Mozilla Firefox bugs
	#[inline(always)]
	pub fn firefox_feature_identifiers(&self) -> &'a [String]
	{
		&self.feature_detail.firefox_feature_identifiers[..]
	}
	
	/// Identifiers to related WebKit bugs
	#[inline(always)]
	pub fn webkit_feature_identifiers(&self) -> &'a [String]
	{
		&self.feature_detail.webkit_feature_identifiers[..]
	}
	
	/// Should any prefix be in uppercase?
	/// Extremely rarely used by the caniuse.com database.
	#[inline(always)]
	pub fn upper_case_prefix(&self) -> bool
	{
		self.feature_detail.upper_case_prefix
	}
	
	/// Effectively, is this feature in 'draft' form?
	/// Extremely rarely used, if at all, by the caniuse.com database, and only for extremely recent features.
	#[inline(always)]
	pub fn this_feature_is_not_yet_complete_or_accurate(&self) -> bool
	{
		!self.feature_detail.shown
	}
}
