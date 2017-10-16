// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


use super::CanIUse;
use ::serde_json;
use ::std::fs::File;
use ::std::path::Path;


#[test]
fn distance_test()
{
	// Open the file in read-only mode.
	let file = File::open(path)?;
	
	// Read the JSON contents of the file as an instance of `User`.
	let u = serde_json::from_reader(file)?;
	
	let canIUse: CanIUse = serde_json::from_str("hello world").expect("Could not deserialize Can I Use JSON data");
}
