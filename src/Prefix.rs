// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// The prefix put before at-rules, property names and property values
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Prefix
{
	/// -o- prefix (legacy Opera Presto prefix).
	o,
	
	/// -moz- prefix.
	moz,
	
	/// -webkit- prefix (Is sometimes also used by IE, Edge and Blink-based browsers (Chrome and Opera)).
	webkit,
	
	/// -ms- prefix.
	ms,
	
	/// A prefix that did not exist in the caniuse.com data when this library was created
	Unknown(String),
}

impl Default for Prefix
{
	/// Defaults to Prefix::webkit, the commonest prefix
	#[inline(always)]
	fn default() -> Self
	{
		Prefix::webkit
	}
}

impl<'de> Deserialize<'de> for Prefix
{
	/// Deserialize using Serde
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		match deserializer.deserialize_str(PrefixVisitor)?
		{
			None => Ok(Prefix::Unknown("".to_owned())),
			Some(value) => Ok(value),
		}
	}
}
