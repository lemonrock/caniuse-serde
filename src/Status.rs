// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Status
{
	W3CRecommendation,
	W3CProposedRecommendation,
	W3CCandidateRecommendation,
	W3CWorkingDraft,
	WhatwgLivingStandard,
	Other,
	UnofficialOrNote,
	
	Unknown(String),
}

impl Default for Status
{
	#[inline(always)]
	fn default() -> Self
	{
		Status::Other
	}
}

impl<'de> Deserialize<'de> for Status
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct StatusVisitor;
		
		impl<'de> Visitor<'de> for StatusVisitor
		{
			type Value = Status;
			
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an era name starting with 'e' followed by a signed integer")
			}
			
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				use self::Status::*;
				
				let result = match v
				{
					"rec" => W3CRecommendation,
					"pr" => W3CProposedRecommendation,
					"cr" => W3CCandidateRecommendation,
					"wd" => W3CWorkingDraft,
					"ls" => WhatwgLivingStandard,
					"other" => Other,
					"unoff" => UnofficialOrNote,
					
					_ => Unknown(v.to_owned()),
				};
				Ok(result)
			}
			
			fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E>
			{
				use self::Status::*;
				
				let result = match &v[..]
				{
					"rec" => W3CRecommendation,
					"pr" => W3CProposedRecommendation,
					"cr" => W3CCandidateRecommendation,
					"wd" => W3CWorkingDraft,
					"ls" => WhatwgLivingStandard,
					"other" => Other,
					"unoff" => UnofficialOrNote,
					
					_ => Unknown(v),
				};
				Ok(result)
			}
		}
		
		deserializer.deserialize_str(StatusVisitor)
	}
}

impl Status
{
	#[inline(always)]
	pub fn description<'a>(&self, canIUse: &'a CanIUse) -> Option<&'a str>
	{
		canIUse.statusDescription(self)
	}
}
