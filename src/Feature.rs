// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct Feature<'a>
{
	featureName: &'a FeatureName,
	featureDetail: &'a FeatureDetail,
}

impl<'a> Feature<'a>
{
	#[inline(always)]
	pub fn featureName(&self) -> &'a FeatureName
	{
		self.featureName
	}
	
	#[inline(always)]
	pub fn title(&self) -> &'a str
	{
		&self.featureDetail.title
	}
	
	#[inline(always)]
	pub fn description(&self) -> &'a str
	{
		&self.featureDetail.description
	}
	
	#[inline(always)]
	pub fn specification_url(&self) -> &'a Url
	{
		&self.featureDetail.specification_url
	}
	
	#[inline(always)]
	pub fn status(&self) -> &'a Status
	{
		&self.featureDetail.status
	}
	
	#[inline(always)]
	pub fn links(&self) -> &'a [Link]
	{
		&self.featureDetail.links[..]
	}
	
	#[inline(always)]
	pub fn bugs(&self) -> &'a [Bug]
	{
		&self.featureDetail.bugs[..]
	}
	
	#[inline(always)]
	pub fn categories(&self) -> &'a [Category]
	{
		&self.featureDetail.categories[..]
	}
	
	#[inline(always)]
	pub fn general_notes(&self) -> &'a str
	{
		&self.featureDetail.notes
	}
	
	/// implementations; returns None if agentName has no known usages
	#[inline(always)]
	pub fn implementations_by_agents(&self, agentName: &AgentName, lowerBound: Bound<&Version>, upperBound: Bound<&Version>) -> Option<Range<Version, SupportDetail>>
	{
		match self.featureDetail.implementations_by_agents.get(agentName)
		{
			None => None,
			Some(entry) => Some(entry.range((lowerBound, upperBound)))
		}
	}
	
	#[inline(always)]
	pub fn usage(&self) -> (UsagePercentage, UsagePercentage)
	{
		(self.featureDetail.usage_y, self.featureDetail.usage_a)
	}
	
	#[inline(always)]
	pub fn parent_feature_if_any(&self) -> Option<&'a FeatureName>
	{
		self.featureDetail.parent.as_ref()
	}
	
	#[inline(always)]
	pub fn keywords(&self) -> &'a [String]
	{
		&self.featureDetail.keywords[..]
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
			Opera => &self.featureDetail.blink_feature_identifiers[..],
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
		}
	}
	
	#[inline(always)]
	pub fn internet_explorer_feature_identifiers(&self) -> &'a [String]
	{
		&self.featureDetail.internet_explorer_feature_identifiers[..]
	}
	
	#[inline(always)]
	pub fn blink_feature_identifiers(&self) -> &'a [String]
	{
		&self.featureDetail.blink_feature_identifiers[..]
	}
	
	#[inline(always)]
	pub fn firefox_feature_identifiers(&self) -> &'a [String]
	{
		&self.featureDetail.firefox_feature_identifiers[..]
	}
	
	#[inline(always)]
	pub fn webkit_feature_identifiers(&self) -> &'a [String]
	{
		&self.featureDetail.webkit_feature_identifiers[..]
	}
	
	#[inline(always)]
	pub fn upper_case_prefix(&self) -> bool
	{
		self.featureDetail.upper_case_prefix
	}
	
	#[inline(always)]
	pub fn this_feature_is_not_yet_complete_or_accurate(&self) -> bool
	{
		!self.featureDetail.shown
	}
}
