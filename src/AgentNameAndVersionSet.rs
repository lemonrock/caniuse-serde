// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Encapsulates choices of Agent and Version of that agent
#[derive(Default, Debug, Clone)]
pub struct AgentNameAndVersionSet(HashSet<(AgentName, Version)>);

impl Deref for AgentNameAndVersionSet
{
	type Target = HashSet<(AgentName, Version)>;
	
	/// Dereferences to HashSet<(AgentName, Version)>
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl AgentNameAndVersionSet
{
	/// Find support for implementations of a feature; useful for downstream applications, eg to find prefixes to autoprefix CSS with
	#[inline(always)]
	pub fn support_for_a_feature<'a, F: FnMut(&Agent, &Version, &Support)>(&self, can_i_use: &'a CanIUse, feature_name: &'a FeatureName, mut support_user: F)
	{
		self.feature(can_i_use, feature_name, |agent, version, support|
		{
			if let Some(Some(ref support)) = support
			{
				support_user(agent, version, support)
			}
		})
	}
	
	/// Find out about a feature
	#[inline(always)]
	pub fn feature<'a, F: FnMut(&Agent, &Version, Option<Option<Support>>)>(&self, can_i_use: &'a CanIUse, feature_name: &'a FeatureName, mut feature_implementation_user: F)
	{
		if let Some(feature) = feature_name.feature(can_i_use)
		{
			for &(ref agent_name, ref version) in self.0.iter()
			{
				if let Some(agent) = agent_name.agent(can_i_use)
				{
					feature_implementation_user(&agent, version, feature.implementation(agent_name, version));
				}
			}
		}
	}
	
	/// Constructor to use if one of the methods below isn't suitable
	#[inline(always)]
	pub fn new(values: HashSet<(AgentName, Version)>) -> Self
	{
		AgentNameAndVersionSet(values)
	}
	
	/// A sensible set of choices for an international website in multiple languages
	#[inline(always)]
	pub fn a_sensible_set_of_choices_for_an_international_website_in_multiple_languages(can_i_use: &CanIUse, maximum_release_age_from_can_i_use_database_last_updated: Duration, minimum_usage_threshold: UsagePercentage, regional_usages: &[&RegionalUsage]) -> Self
	{
		let obsolete_browsers_still_in_use = Self::obsolete_browsers_still_in_use();
		let browsers_which_underwent_a_major_change_of_rendering_engine = Self::browsers_which_underwent_a_major_change_of_rendering_engine();
		let automatically_updated_browsers = Self::automatically_updated_browsers();
		let long_term_releases_of_automatically_updated_browsers = Self::long_term_releases_of_automatically_updated_browsers();
		let regionally_significant_occasionally_automatically_updated_browsers = Self::regionally_significant_occasionally_automatically_updated_browsers();
		
		Self::sensible_choices(can_i_use, maximum_release_age_from_can_i_use_database_last_updated, minimum_usage_threshold, regional_usages, obsolete_browsers_still_in_use, browsers_which_underwent_a_major_change_of_rendering_engine, automatically_updated_browsers, long_term_releases_of_automatically_updated_browsers, regionally_significant_occasionally_automatically_updated_browsers)
	}
	
	/// A sensible set of rules that makes sure:-
	/// - obsolete but still-used browsers are included
	/// - browsers with a major change of rendering engine but still-used are included
	/// - automatically updated or long-term supported browsers are included
	/// - regionally significant and not necessarily frequently updated browsers are included
	pub fn sensible_choices(can_i_use: &CanIUse, maximum_release_age_from_can_i_use_database_last_updated: Duration, minimum_usage_threshold: UsagePercentage, regional_usages: &[&RegionalUsage], obsolete_browsers_still_in_use: Self, browsers_which_underwent_a_major_change_of_rendering_engine: Self, automatically_updated_browsers: HashSet<AgentName>, long_term_releases_of_automatically_updated_browsers: HashSet<AgentName>, regionally_significant_occasionally_automatically_updated_browsers: HashSet<AgentName>) -> Self
	{
		let mut result = Self::default();
		
		for regional_usage in regional_usages.iter()
		{
			result.add_just_these_versions_by_usage_percentage(&obsolete_browsers_still_in_use, regional_usage, minimum_usage_threshold);
			result.add_just_these_versions_by_usage_percentage(&browsers_which_underwent_a_major_change_of_rendering_engine, regional_usage, minimum_usage_threshold);
			result.add_any_current_or_older_version_by_usage_percentage(&regionally_significant_occasionally_automatically_updated_browsers, regional_usage, minimum_usage_threshold, can_i_use);
		}
		
		let oldest_release_date = match can_i_use.last_updated().checked_sub_signed(maximum_release_age_from_can_i_use_database_last_updated)
		{
			Some(oldest_release_date) => oldest_release_date,
			None => Utc.timestamp(0, 0),
		};
		
		result.add_any_current_or_older_version_by_age(&automatically_updated_browsers, oldest_release_date, can_i_use);
		result.add_any_current_or_older_version_by_age(&long_term_releases_of_automatically_updated_browsers, oldest_release_date, can_i_use);
		
		result
	}
	
	/// Adds browser-version combination if it exceeds minimum usage threshold for region
	pub fn add_just_these_versions_by_usage_percentage(&mut self, obsolete_or_changed_rendering_engine: &Self, regional_usage: &RegionalUsage, minimum_usage_threshold: UsagePercentage)
	{
		for &(ref agent_name, ref version) in obsolete_or_changed_rendering_engine.0.iter()
		{
			if let Some(Some(&Some(actual_usage))) = regional_usage.usage_of_version(agent_name, version)
			{
				if actual_usage >= minimum_usage_threshold
				{
					self.0.insert((agent_name.clone(), version.clone()));
				}
			}
		}
	}
	
	/// Adds browser-version combination for any current or older version if it exceeds or equals minimum usage threshold for region
	pub fn add_any_current_or_older_version_by_usage_percentage(&mut self, agent_names: &HashSet<AgentName>, regional_usage: &RegionalUsage, minimum_usage_threshold: UsagePercentage, can_i_use: &CanIUse)
	{
		for agent_name in agent_names.iter()
		{
			if let Some(agent) = agent_name.agent(can_i_use)
			{
				agent.version_details_for_current_and_older_versions().for_each(|(version, _version_detail)|
				{
					if let Some(Some(&Some(actual_usage))) = regional_usage.usage_of_version(agent_name, version)
					{
						if actual_usage >= minimum_usage_threshold
						{
							self.0.insert((agent_name.clone(), version.clone()));
						}
					}
				})
			}
		}
	}
	
	/// Adds browser-version combination if it exceeds or equals oldest release date
	pub fn add_any_current_or_older_version_by_age(&mut self, agent_names: &HashSet<AgentName>, oldest_release_date: DateTime<Utc>, can_i_use: &CanIUse)
	{
		for agent_name in agent_names.iter()
		{
			if let Some(agent) = agent_name.agent(can_i_use)
			{
				agent.version_details_for_current_and_older_versions().for_each(|(version, version_detail)|
				{
					if let Some(release_date) = version_detail.release_date()
					{
						if release_date >= oldest_release_date
						{
							self.0.insert((agent_name.clone(), version.clone()));
						}
					}
				})
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
	/// These browsers have short-lived, sub-yearly versions
	/// They are probably best discovered by matching for all released versions after a specific release date (eg 2 years ago)
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
	/// Usage percentages for these may be very low globally, and they may be 9 or more release versions 'out-of-date', but they represent an important audience.
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
	/// All of them currently are just more dated versions of the Webkit rendering engine than Chrome.
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
