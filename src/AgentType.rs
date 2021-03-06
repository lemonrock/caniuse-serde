// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


/// Represents whether an agent is used primarily of the desktop or a mobile device
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AgentType
{
	/// This agent is used on the desktop
	desktop,
	
	/// This agent is used on mobile devices,
	mobile,
}
