// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Represents part of a version number.
/// Not a foolproof model but adequate for current caniuse.com data
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum VersionPart
{
	/// Represents a number
	Number(u64),
	
	/// Represents Safari's TP version; sorts after a number
	TechnologyPreview,
	
	/// Represents Opera Mini's all versions
	All,
	
	/// Represents an unknown version part, perhaps a hyphenated release candidate, beta, etc; sorts after a number, which may be inappropriate
	Unknown(String),
}
