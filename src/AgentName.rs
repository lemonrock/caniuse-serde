// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// The name of this agent
#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AgentName
{
	MicrosoftInternetExplorer,
	MicrosoftEdge,
	MozillaFirefox,
	GoogleChrome,
	AppleSafari,
	Opera,
	AppleSafariIOs,
	OperaMini,
	GoogleAndroidBrowserAndWebComponent,
	Blackberry,
	OperaMobile,
	GoogleChromeAndroid,
	MozillaFirefoxAndroid,
	MicrosoftInternetExplorerMobile,
	UcBrowserAndroid,
	SamsungBrowserAndroid,
	QqBrowserAndroid,
	BaiduBrowserAndroid,
	
	#[doc(hidden)] __Nonexhaustive,
	
	/// An agent that did not exist in the caniuse.com data when this library was created
	Unknown(String),
}

impl Default for AgentName
{
	/// Defaults to AgentName::GoogleChrome, the (as of October 2017) most common user agent
	#[inline(always)]
	fn default() -> Self
	{
		AgentName::GoogleChrome
	}
}

impl<'de> Deserialize<'de> for AgentName
{
	/// Deserialize using Serde
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct AgentNameVisitor;
		
		impl<'de> Visitor<'de> for AgentNameVisitor
		{
			type Value = AgentName;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an era name starting with 'e' followed by a signed integer")
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				use self::AgentName::*;
				
				let result = match v
				{
					"ie" => MicrosoftInternetExplorer,
					"edge" => MicrosoftEdge,
					"firefox" => MozillaFirefox,
					"chrome" => GoogleChrome,
					"safari" => AppleSafari,
					"opera" => Opera,
					"ios_saf" => AppleSafariIOs,
					"op_mini" => OperaMini,
					"android" => GoogleAndroidBrowserAndWebComponent,
					"bb" => Blackberry,
					"op_mob" => OperaMobile,
					"and_chr" => GoogleChromeAndroid,
					"and_ff" => MozillaFirefoxAndroid,
					"ie_mob" => MicrosoftInternetExplorerMobile,
					"and_uc" => UcBrowserAndroid,
					"samsung" => SamsungBrowserAndroid,
					"and_qq" => QqBrowserAndroid,
					"baidu" => BaiduBrowserAndroid,
					
					_ => Unknown(v.to_owned()),
				};
				Ok(result)
			}
			
			#[inline(always)]
			fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E>
			{
				use self::AgentName::*;
				
				let result = match &v[..]
				{
					"ie" => MicrosoftInternetExplorer,
					"edge" => MicrosoftEdge,
					"firefox" => MozillaFirefox,
					"chrome" => GoogleChrome,
					"safari" => AppleSafari,
					"opera" => Opera,
					"ios_saf" => AppleSafariIOs,
					"op_mini" => OperaMini,
					"android" => GoogleAndroidBrowserAndWebComponent,
					"bb" => Blackberry,
					"op_mob" => OperaMobile,
					"and_chr" => GoogleChromeAndroid,
					"and_ff" => MozillaFirefoxAndroid,
					"ie_mob" => MicrosoftInternetExplorerMobile,
					"and_uc" => UcBrowserAndroid,
					"samsung" => SamsungBrowserAndroid,
					"and_qq" => QqBrowserAndroid,
					"baidu" => BaiduBrowserAndroid,
					
					_ => Unknown(v),
				};
				Ok(result)
			}
		}
		
		deserializer.deserialize_str(AgentNameVisitor)
	}
}

impl AgentName
{
	/// Given an agent name and the CanIUse database, find the associated agent.
	/// Returns None if this agent is not defined (this is typically either due to a typo or different versions of the caniuse.com database).
	#[inline(always)]
	pub fn agent<'a>(&'a self, can_i_use: &'a CanIUse) -> Option<Agent<'a>>
	{
		can_i_use.agent(self)
	}
}

/// X
#[derive(Default, Debug, Clone)]
pub struct AgentNameAndVersionSet(HashSet<(AgentName, Version)>);

impl AgentNameAndVersionSet
{
	/// X
	pub fn add_just_these_versions_usage(&mut self, mut obsolete_or_changed_rendering_engine: Self, regional_usage: &RegionalUsage, minimum_usage_threshold: UsagePercentage)
	{
		use self::Bound::*;
		
		for (agent_name, version) in obsolete_or_changed_rendering_engine.0.drain()
		{
			match regional_usage.usage(&agent_name, Included(&version), Included(&version))
			{
				Some(range) =>
				{
					for (version, actual_usage) in range
					{
						if let &Some(actual_usage) = actual_usage
						{
							if actual_usage >= minimum_usage_threshold
							{
								self.0.insert((agent_name.clone(), version.clone()));
							}
						}
					}
				}
				None => (),
			}
		}
	}
	
	/// Obsolete browsers still in use.
	/// We need to support the last version of these until its percentage usage falls below X%.
	/// The percentage usage (X%) should be for a sub-set of the world (ie target audience continents or countries).
	/// Returns a list of (Agent, Last-Known-Version) pairs.
	#[inline(always)]
	pub fn obsolete_browsers_still_in_use() -> Self
	{
		use self::AgentName::*;
		
		AgentNameAndVersionSet
		(
			hashset!
			(
				(MicrosoftInternetExplorer, Version::major(11)),
				(Blackberry, Version::major(10)),
				(MicrosoftInternetExplorerMobile, Version::major(11)),
			)
		)
	}
	
	/// Browsers which underwent a major change of rendering engine.
	/// We need to support the last version of these until its percentage usage falls below X%.
	/// The percentage usage (X%) should be for a sub-set of the world (ie target audience continents or countries).
	/// Returns a list of (Agent, Last-Known-Version-before-change-of-rendering-engine) pairs.
	#[inline(always)]
	pub fn browsers_which_underwent_a_major_change_of_rendering_engine() -> Self
	{
		use self::AgentName::*;
		
		AgentNameAndVersionSet
		(
			hashset!
			(
				(Opera, Version::major_minor(12, 1)),
				(GoogleAndroidBrowserAndWebComponent, Version::major_minor_revision(4, 4, 4)),
				(OperaMobile, Version::major_minor(12, 1)),
			)
		)
	}
	
	/// Browsers which are regularly updated, automatically and so which do not 'hang around'.
	/// These browsers have short-lived, sub-yearly versions and so a simple 'last X versions' rule is sufficient.
	/// X can be the same for all browsers.
	/// Using a percentage isn't wise as usage of each version will change rapidly (from near zero to a few percentage points, then to near zero again), and certainly likely to change more rapidly than static website rebuilds.
	#[inline(always)]
	pub fn automatically_updated_browsers() -> HashSet<AgentName>
	{
		use self::AgentName::*;
		
		hashset!
		(
			MicrosoftEdge,
			MozillaFirefox,
			GoogleChrome,
			AppleSafari,
			Opera,
			AppleSafariIOs,
			GoogleAndroidBrowserAndWebComponent,
			OperaMobile,
			GoogleChromeAndroid,
			MozillaFirefoxAndroid,
		)
	}
	
	/// Long-Term Releases of Automatically Updated Browsers.
	/// These browsers have occasional long-term releases which are intended to be supported for a year or more.
	/// Usage percentages for these may be very low globally, and they may be 9 or more release versions 'out-of-date', but they represent an important audience
	/// In practice the length of time each long term release is supported for changes with each release, even though vendors have 'long term release policies'.
	/// This is because policies change in the long interval between long-term releases.
	/// These browsers are problematic to identify as the caniuse.com database omits them.
	/// Some long-term release versions differ slightly in supported features, particularly those of a more experimental nature, to their related short-term release cousins (even though they may share the same major version number).
	/// For Firefox, ESR releases are supposedly for one year (actually, 54 weeks, '9-cycles', with a 12-week ('2-cycle') overlap between releases (a cycle is a Firefox release cycle, typically 6 weeks), but, as always for these sorts of releases, the policy has changed several times.
	#[inline(always)]
	pub fn long_term_releases_of_automatically_updated_browsers() -> HashSet<AgentName>
	{
		use self::AgentName::*;
		
		hashset!
		(
			MozillaFirefox,
		)
	}
	
	/// Regionally significant, occasionally automatically updated browsers.
	/// Support of these browsers is particularly important for the Indian and Asian markets.
	/// Many cheaper smart phones come with them (I've used them, too).
	/// Vendors frequently don't upgrade old firmware installed versions and some older versions may persist and have higher usage for some time than newer ones.
	/// All of them currently are just more dated versions of the Webkit rendering engine.
	/// These browsers are probably best supported with a 'above X% rule', where X is for any version.
	#[inline(always)]
	pub fn regionally_significant_occasionally_automatically_updated_browsers() -> HashSet<AgentName>
	{
		use self::AgentName::*;
		
		hashset!
		(
			UcBrowserAndroid,
			SamsungBrowserAndroid,
			QqBrowserAndroid,
			BaiduBrowserAndroid,
		)
	}
}
