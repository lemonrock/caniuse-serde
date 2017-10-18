// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// An EraName is a caniuse.com concept for modelling past, current and future browsers with a similar version.
/// Not all browsers belong to any particular Era. Particularly so for older versions of Internet Explorer, which has had very few releases.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EraName(i64);

impl<'de> Deserialize<'de> for EraName
{
	/// Deserialize using Serde
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct EraNameVisitor;
		
		impl<'de> Visitor<'de> for EraNameVisitor
		{
			type Value = EraName;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an era name starting with 'e' followed by a signed integer")
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				if !v.starts_with('e')
				{
					return Err(E::custom("should start with 'e'"))
				}
				match (&v[1..]).parse()
				{
					Ok(value) => Ok(EraName(value)),
					Err(parseError) => Err(E::custom(parseError))
				}
			}
		}
		
		deserializer.deserialize_str(EraNameVisitor)
	}
}

impl Default for EraName
{
	/// Returns the current Era
	#[inline(always)]
	fn default() -> Self
	{
		Self::Current
	}
}

impl EraName
{
	/// Returns the oldest known Era
	/// Panics if the CanIUse database has no Eras; this should not be possible unless you're crafting your own input JSON.
	#[inline(always)]
	pub fn oldest<'a>(canIUse: &'a CanIUse) -> Self
	{
		canIUse.eras.oldest().expect("there ought to be at least one era").clone()
	}
	
	/// The current Era
	pub const Current: EraName = EraName(0);
	
	/// Returns the youngest known Era; typically this is also that associated with Safari's Technology Preview so it is not normally an useful concept.
	/// Panics if the CanIUse database has no Eras; this should not be possible unless you're crafting your own input JSON.
	#[inline(always)]
	pub fn youngest<'a>(canIUse: &'a CanIUse) -> Self
	{
		canIUse.eras.youngest().expect("there ought to be at least one era").clone()
	}
	
	/// Returns an Era immediately older that this one.
	/// Returns None if this is the oldest era.
	#[inline(always)]
	pub fn older<'a>(&self, canIUse: &'a CanIUse) -> Option<Self>
	{
		canIUse.eras.has(EraName(self.0 - 1))
	}
	
	/// Returns an Era immediately younger that this one.
	/// Returns None if this is the youngest era.
	#[inline(always)]
	pub fn younger<'a>(&self, canIUse: &'a CanIUse) -> Option<Self>
	{
		canIUse.eras.has(EraName(self.0 + 1))
	}
	
	/// Returns a description of this Era, typically for an UI.
	#[inline(always)]
	pub fn description<'a>(&self, canIUse: &'a CanIUse) -> Option<&'a str>
	{
		canIUse.eras.description(self)
	}
	
	#[inline(always)]
	fn is_negative(&self) -> bool
	{
		self.0 < 0
	}
}
