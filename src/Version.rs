// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Version "3" and "3.0" are not considered equal; "3.0" is greater than "3".
/// The legacy Opera (Presto) version strings "9.5-9.6" and "10.0-10.1" are converted to 9.5 and 10.0.
/// Safari also has "TP" for its latest version, which is not stable across time and is converted to the VersionPart::TechnologyPreview.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Version(VersionPart, Vec<VersionPart>);

impl<'de> Deserialize<'de> for Version
{
	/// Deserialize using Serde
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct SupportVisitor;
		
		impl<'de> Visitor<'de> for SupportVisitor
		{
			type Value = Version;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a string which contains a period-delimited version")
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				Ok(Version::parse(v))
			}
		}
		
		deserializer.deserialize_str(SupportVisitor)
	}
}

impl<'a, I: Into<&'a str>> From<I> for Version
{
	/// Converts into a Version anything that can be converted into '&str'.
	/// Use with `into()`.
	#[inline(always)]
	fn from(value: I) -> Self
	{
		Version::parse(value.into())
	}
}

impl FromStr for Version
{
	type Err = ();
	
	/// Converts into a Version anything from '&str'.
	/// Use with `parse()`.
	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		Ok(Version::parse(s))
	}
}

impl Version
{
	/// Special method to construct a version representing the Opera (Presto) 9.5-9.6 range
	#[inline(always)]
	pub fn opera9Dot5Or9Dot6() -> Version
	{
		use self::VersionPart::*;
		Version(Number(9), vec![Number(5)])
	}
	
	/// Special method to construct a version representing the Opera (Presto) 10.0-10.1 range
	#[inline(always)]
	pub fn opera10Dot0Or10Dot1() -> Version
	{
		use self::VersionPart::*;
		Version(Number(10), vec![Number(0)])
	}
	
	/// Special method to construct a version representing the Safari TP version
	#[inline(always)]
	pub fn safariTechnologyPreview() -> Version
	{
		Version(VersionPart::TechnologyPreview, vec![])
	}
	
	/// Is this version the Safari Technology Preview?
	#[inline(always)]
	pub fn is_safari_technology_preview(&self) -> bool
	{
		match self.0
		{
			VersionPart::TechnologyPreview => true,
			_ => false,
		}
	}
	
	/// Is this version "0" (sometimes found in caniuse.com's Regional data) or Unknown
	#[inline(always)]
	pub fn is_invalid_or_unknown(&self) -> bool
	{
		match self.0
		{
			VersionPart::Number(0) => true,
			VersionPart::Unknown(_) => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	fn parse(v: &str) -> Self
	{
		use self::VersionPart::*;
		
		// Specialized logic to handle legacy Opera Presto ranges and Safari Technology Preview
		match v
		{
			"9.5-9.6" => return Self::opera9Dot5Or9Dot6(),
			"10.0-10.1" => return Self::opera10Dot0Or10Dot1(),
			"TP" => return Self::safariTechnologyPreview(),
			_ => (),
		}
		
		let parts = v.split('.');
		
		let (lower, upper) = parts.size_hint();
		let mut capacity = if let Some(upper) = upper
		{
			upper
		}
		else
		{
			lower
		};
		if capacity != 0
		{
			capacity -= 1;
		}
		
		let mut first = None;
		let mut subsequent = Vec::with_capacity(capacity);
		for part in parts
		{
			let versionPart = match part.parse::<u64>()
			{
				Ok(value) => Number(value),
				Err(_) => Unknown(part.to_owned())
			};
			if first.is_none()
			{
				first = Some(versionPart);
			}
			else
			{
				subsequent.push(versionPart);
			}
		}
		
		subsequent.shrink_to_fit();
		Version(first.unwrap(), subsequent)
	}
}
