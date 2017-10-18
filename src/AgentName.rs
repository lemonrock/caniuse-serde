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
	/// Defaults to AgentName::GoogleChrome, the comments user agent
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
	pub fn agent<'a>(&'a self, canIUse: &'a CanIUse) -> Option<Agent<'a>>
	{
		canIUse.agent(self)
	}
}
