// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SupportDetail
{
	maturity: SupportMaturity,
	requiresPrefix: bool,
	disabledByDefault: bool,
	notes: Vec<u8>,
}

impl<'de> Deserialize<'de> for SupportDetail
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct SupportVisitor;
		
		impl<'de> Visitor<'de> for SupportVisitor
		{
			type Value = SupportDetail;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a string which contains comma separated sub-strings")
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				#[inline(always)]
				fn parseNoteAndSubsequentNotes<'a, I: Iterator<Item=&'a str>, E: de::Error>(mut support: SupportDetail, potentialNote: &str, mut potentialNotes: I) -> Result<SupportDetail, E>
				{
					let mut potentialNote = Some(potentialNote);
					
					while potentialNote.is_some()
					{
						let note = potentialNote.unwrap();
						if note.starts_with('#')
						{
							match (&note[1..]).parse::<u8>()
							{
								Err(parseError) => return Err(E::custom(parseError)),
								Ok(oneBasedNoteNumber) =>
								{
									support.notes.push(oneBasedNoteNumber);
								}
							}
						}
						else
						{
							return Err(E::custom("Expected a note that started with a '#'"));
						}
						potentialNote = potentialNotes.next()
					}
					
					Ok(support)
				}
				
				if v.is_empty()
				{
					return Err(E::custom("can not be empty"));
				}
				
				use self::SupportMaturity::*;
				
				let mut items = v.split(' ');
				let maturity = match items.next()
				{
					None => return Err(E::custom("must have first item")),
					Some(first) => match first
					{
						"y" => SupportedByDefault,
						"a" => AlmostSupported,
						"n" => NotSupportedOrDisabledByDefault,
						"p" => SupportedUsingAPolyfill,
						"u" => SupportUnknown,
						
						_ => return Err(E::custom("there can only be 'y', 'a', 'n', 'p' or 'u' starting stats")),
					}
				};
				
				let mut support = SupportDetail
				{
					maturity,
					requiresPrefix: false,
					disabledByDefault: false,
					notes: Vec::with_capacity(0),
				};
				
				match items.next()
				{
					None => return Ok(support),
					Some(value) => match value
					{
						"x" =>
						{
							support.requiresPrefix = true;
						}
						
						"d" =>
						{
							support.disabledByDefault = true;
						}
						
						_ => return parseNoteAndSubsequentNotes(support, value, items),
					}
				}
				
				match items.next()
				{
					None => return Ok(support),
					Some(value) => match value
					{
						"d" =>
						{
							support.disabledByDefault = true;
						}
						
						_ => return parseNoteAndSubsequentNotes(support, value, items),
					}
				}
				
				match items.next()
				{
					None => Ok(support),
					Some(value) => parseNoteAndSubsequentNotes(support, value, items)
				}
			}
		}
		
		deserializer.deserialize_str(SupportVisitor)
	}
}

impl SupportDetail
{
	#[inline(always)]
	fn maturity(&self) -> SupportMaturity
	{
		self.maturity
	}
	
	#[inline(always)]
	fn requiresPrefix(&self) -> bool
	{
		self.requiresPrefix
	}
	
	#[inline(always)]
	fn disabledByDefault(&self) -> bool
	{
		self.disabledByDefault
	}
	
	#[inline(always)]
	fn oneBasedNoteNumbers(&self) -> &[u8]
	{
		&self.notes[..]
	}
}
