// This file is part of caniuse-serde. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of caniuse-serde. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/caniuse-serde/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone)]
pub struct RegionalUsage
{
	id: String,
	name: String,
	//month: "2017-09",
	//access_date: "2017-10-05",
	data: HashMap<AgentName, BTreeMap<Version, Option<UsagePercentage>>>,
	total: UsagePercentage,
}

impl FromStr for RegionalUsage
{
	type Err = ::serde_json::error::Error;
	
	#[inline(always)]
	fn from_str(canIUseRegionalUsageDatabase: &str) -> Result<Self, Self::Err>
	{
		::serde_json::from_str(canIUseRegionalUsageDatabase)
	}
}

lazy_static!
{	pub static ref AD: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AD.json")).unwrap();
	pub static ref AE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AE.json")).unwrap();
	pub static ref AF: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AF.json")).unwrap();
	pub static ref AG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AG.json")).unwrap();
	pub static ref AI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AI.json")).unwrap();
	pub static ref AL: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AL.json")).unwrap();
	pub static ref AM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AM.json")).unwrap();
	pub static ref AN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AN.json")).unwrap();
	pub static ref AO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AO.json")).unwrap();
	pub static ref AR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AR.json")).unwrap();
	pub static ref AS: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AS.json")).unwrap();
	pub static ref AT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AT.json")).unwrap();
	pub static ref AU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AU.json")).unwrap();
	pub static ref AW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AW.json")).unwrap();
	pub static ref AX: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AX.json")).unwrap();
	pub static ref AZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/AZ.json")).unwrap();
	pub static ref BA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BA.json")).unwrap();
	pub static ref BB: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BB.json")).unwrap();
	pub static ref BD: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BD.json")).unwrap();
	pub static ref BE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BE.json")).unwrap();
	pub static ref BF: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BF.json")).unwrap();
	pub static ref BG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BG.json")).unwrap();
	pub static ref BH: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BH.json")).unwrap();
	pub static ref BI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BI.json")).unwrap();
	pub static ref BJ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BJ.json")).unwrap();
	pub static ref BM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BM.json")).unwrap();
	pub static ref BN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BN.json")).unwrap();
	pub static ref BO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BO.json")).unwrap();
	pub static ref BR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BR.json")).unwrap();
	pub static ref BS: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BS.json")).unwrap();
	pub static ref BT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BT.json")).unwrap();
	pub static ref BW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BW.json")).unwrap();
	pub static ref BY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BY.json")).unwrap();
	pub static ref BZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/BZ.json")).unwrap();
	pub static ref CA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CA.json")).unwrap();
	pub static ref CD: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CD.json")).unwrap();
	pub static ref CF: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CF.json")).unwrap();
	pub static ref CG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CG.json")).unwrap();
	pub static ref CH: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CH.json")).unwrap();
	pub static ref CI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CI.json")).unwrap();
	pub static ref CK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CK.json")).unwrap();
	pub static ref CL: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CL.json")).unwrap();
	pub static ref CM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CM.json")).unwrap();
	pub static ref CN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CN.json")).unwrap();
	pub static ref CO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CO.json")).unwrap();
	pub static ref CR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CR.json")).unwrap();
	pub static ref CU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CU.json")).unwrap();
	pub static ref CV: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CV.json")).unwrap();
	pub static ref CX: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CX.json")).unwrap();
	pub static ref CY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CY.json")).unwrap();
	pub static ref CZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/CZ.json")).unwrap();
	pub static ref DE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/DE.json")).unwrap();
	pub static ref DJ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/DJ.json")).unwrap();
	pub static ref DK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/DK.json")).unwrap();
	pub static ref DM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/DM.json")).unwrap();
	pub static ref DO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/DO.json")).unwrap();
	pub static ref DZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/DZ.json")).unwrap();
	pub static ref EC: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/EC.json")).unwrap();
	pub static ref EE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/EE.json")).unwrap();
	pub static ref EG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/EG.json")).unwrap();
	pub static ref ER: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ER.json")).unwrap();
	pub static ref ES: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ES.json")).unwrap();
	pub static ref ET: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ET.json")).unwrap();
	pub static ref FI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/FI.json")).unwrap();
	pub static ref FJ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/FJ.json")).unwrap();
	pub static ref FK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/FK.json")).unwrap();
	pub static ref FM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/FM.json")).unwrap();
	pub static ref FO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/FO.json")).unwrap();
	pub static ref FR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/FR.json")).unwrap();
	pub static ref GA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GA.json")).unwrap();
	pub static ref GB: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GB.json")).unwrap();
	pub static ref GD: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GD.json")).unwrap();
	pub static ref GE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GE.json")).unwrap();
	pub static ref GF: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GF.json")).unwrap();
	pub static ref GG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GG.json")).unwrap();
	pub static ref GH: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GH.json")).unwrap();
	pub static ref GI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GI.json")).unwrap();
	pub static ref GL: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GL.json")).unwrap();
	pub static ref GM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GM.json")).unwrap();
	pub static ref GN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GN.json")).unwrap();
	pub static ref GP: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GP.json")).unwrap();
	pub static ref GQ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GQ.json")).unwrap();
	pub static ref GR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GR.json")).unwrap();
	pub static ref GT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GT.json")).unwrap();
	pub static ref GU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GU.json")).unwrap();
	pub static ref GW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GW.json")).unwrap();
	pub static ref GY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/GY.json")).unwrap();
	pub static ref HK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/HK.json")).unwrap();
	pub static ref HN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/HN.json")).unwrap();
	pub static ref HR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/HR.json")).unwrap();
	pub static ref HT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/HT.json")).unwrap();
	pub static ref HU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/HU.json")).unwrap();
	pub static ref ID: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ID.json")).unwrap();
	pub static ref IE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/IE.json")).unwrap();
	pub static ref IL: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/IL.json")).unwrap();
	pub static ref IM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/IM.json")).unwrap();
	pub static ref IN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/IN.json")).unwrap();
	pub static ref IQ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/IQ.json")).unwrap();
	pub static ref IR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/IR.json")).unwrap();
	pub static ref IS: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/IS.json")).unwrap();
	pub static ref IT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/IT.json")).unwrap();
	pub static ref JE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/JE.json")).unwrap();
	pub static ref JM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/JM.json")).unwrap();
	pub static ref JO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/JO.json")).unwrap();
	pub static ref JP: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/JP.json")).unwrap();
	pub static ref KE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KE.json")).unwrap();
	pub static ref KG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KG.json")).unwrap();
	pub static ref KH: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KH.json")).unwrap();
	pub static ref KI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KI.json")).unwrap();
	pub static ref KM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KM.json")).unwrap();
	pub static ref KN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KN.json")).unwrap();
	pub static ref KP: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KP.json")).unwrap();
	pub static ref KR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KR.json")).unwrap();
	pub static ref KW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KW.json")).unwrap();
	pub static ref KY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KY.json")).unwrap();
	pub static ref KZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/KZ.json")).unwrap();
	pub static ref LA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LA.json")).unwrap();
	pub static ref LB: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LB.json")).unwrap();
	pub static ref LC: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LC.json")).unwrap();
	pub static ref LI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LI.json")).unwrap();
	pub static ref LK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LK.json")).unwrap();
	pub static ref LR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LR.json")).unwrap();
	pub static ref LS: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LS.json")).unwrap();
	pub static ref LT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LT.json")).unwrap();
	pub static ref LU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LU.json")).unwrap();
	pub static ref LV: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LV.json")).unwrap();
	pub static ref LY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/LY.json")).unwrap();
	pub static ref MA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MA.json")).unwrap();
	pub static ref MC: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MC.json")).unwrap();
	pub static ref MD: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MD.json")).unwrap();
	pub static ref ME: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ME.json")).unwrap();
	pub static ref MG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MG.json")).unwrap();
	pub static ref MH: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MH.json")).unwrap();
	pub static ref MK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MK.json")).unwrap();
	pub static ref ML: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ML.json")).unwrap();
	pub static ref MM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MM.json")).unwrap();
	pub static ref MN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MN.json")).unwrap();
	pub static ref MO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MO.json")).unwrap();
	pub static ref MP: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MP.json")).unwrap();
	pub static ref MQ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MQ.json")).unwrap();
	pub static ref MR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MR.json")).unwrap();
	pub static ref MS: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MS.json")).unwrap();
	pub static ref MT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MT.json")).unwrap();
	pub static ref MU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MU.json")).unwrap();
	pub static ref MV: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MV.json")).unwrap();
	pub static ref MW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MW.json")).unwrap();
	pub static ref MX: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MX.json")).unwrap();
	pub static ref MY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MY.json")).unwrap();
	pub static ref MZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/MZ.json")).unwrap();
	pub static ref NA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NA.json")).unwrap();
	pub static ref NC: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NC.json")).unwrap();
	pub static ref NE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NE.json")).unwrap();
	pub static ref NF: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NF.json")).unwrap();
	pub static ref NG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NG.json")).unwrap();
	pub static ref NI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NI.json")).unwrap();
	pub static ref NL: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NL.json")).unwrap();
	pub static ref NO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NO.json")).unwrap();
	pub static ref NP: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NP.json")).unwrap();
	pub static ref NR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NR.json")).unwrap();
	pub static ref NU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NU.json")).unwrap();
	pub static ref NZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/NZ.json")).unwrap();
	pub static ref OM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/OM.json")).unwrap();
	pub static ref PA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PA.json")).unwrap();
	pub static ref PE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PE.json")).unwrap();
	pub static ref PF: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PF.json")).unwrap();
	pub static ref PG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PG.json")).unwrap();
	pub static ref PH: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PH.json")).unwrap();
	pub static ref PK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PK.json")).unwrap();
	pub static ref PL: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PL.json")).unwrap();
	pub static ref PM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PM.json")).unwrap();
	pub static ref PN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PN.json")).unwrap();
	pub static ref PR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PR.json")).unwrap();
	pub static ref PS: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PS.json")).unwrap();
	pub static ref PT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PT.json")).unwrap();
	pub static ref PW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PW.json")).unwrap();
	pub static ref PY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/PY.json")).unwrap();
	pub static ref QA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/QA.json")).unwrap();
	pub static ref RE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/RE.json")).unwrap();
	pub static ref RO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/RO.json")).unwrap();
	pub static ref RS: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/RS.json")).unwrap();
	pub static ref RU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/RU.json")).unwrap();
	pub static ref RW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/RW.json")).unwrap();
	pub static ref SA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SA.json")).unwrap();
	pub static ref SB: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SB.json")).unwrap();
	pub static ref SC: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SC.json")).unwrap();
	pub static ref SD: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SD.json")).unwrap();
	pub static ref SE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SE.json")).unwrap();
	pub static ref SG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SG.json")).unwrap();
	pub static ref SH: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SH.json")).unwrap();
	pub static ref SI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SI.json")).unwrap();
	pub static ref SK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SK.json")).unwrap();
	pub static ref SL: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SL.json")).unwrap();
	pub static ref SM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SM.json")).unwrap();
	pub static ref SN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SN.json")).unwrap();
	pub static ref SO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SO.json")).unwrap();
	pub static ref SR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SR.json")).unwrap();
	pub static ref ST: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ST.json")).unwrap();
	pub static ref SV: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SV.json")).unwrap();
	pub static ref SY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SY.json")).unwrap();
	pub static ref SZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/SZ.json")).unwrap();
	pub static ref TC: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TC.json")).unwrap();
	pub static ref TD: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TD.json")).unwrap();
	pub static ref TG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TG.json")).unwrap();
	pub static ref TH: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TH.json")).unwrap();
	pub static ref TJ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TJ.json")).unwrap();
	pub static ref TK: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TK.json")).unwrap();
	pub static ref TL: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TL.json")).unwrap();
	pub static ref TM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TM.json")).unwrap();
	pub static ref TN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TN.json")).unwrap();
	pub static ref TO: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TO.json")).unwrap();
	pub static ref TR: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TR.json")).unwrap();
	pub static ref TT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TT.json")).unwrap();
	pub static ref TV: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TV.json")).unwrap();
	pub static ref TW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TW.json")).unwrap();
	pub static ref TZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/TZ.json")).unwrap();
	pub static ref UA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/UA.json")).unwrap();
	pub static ref UG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/UG.json")).unwrap();
	pub static ref US: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/US.json")).unwrap();
	pub static ref UY: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/UY.json")).unwrap();
	pub static ref UZ: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/UZ.json")).unwrap();
	pub static ref VA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/VA.json")).unwrap();
	pub static ref VC: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/VC.json")).unwrap();
	pub static ref VE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/VE.json")).unwrap();
	pub static ref VG: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/VG.json")).unwrap();
	pub static ref VI: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/VI.json")).unwrap();
	pub static ref VN: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/VN.json")).unwrap();
	pub static ref VU: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/VU.json")).unwrap();
	pub static ref WF: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/WF.json")).unwrap();
	pub static ref WS: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/WS.json")).unwrap();
	pub static ref YE: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/YE.json")).unwrap();
	pub static ref YT: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/YT.json")).unwrap();
	pub static ref ZA: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ZA.json")).unwrap();
	pub static ref ZM: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ZM.json")).unwrap();
	pub static ref ZW: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/ZW.json")).unwrap();
	pub static ref alt_af: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/alt-af.json")).unwrap();
	pub static ref alt_an: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/alt-an.json")).unwrap();
	pub static ref alt_as: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/alt-as.json")).unwrap();
	pub static ref alt_eu: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/alt-eu.json")).unwrap();
	pub static ref alt_na: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/alt-na.json")).unwrap();
	pub static ref alt_oc: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/alt-oc.json")).unwrap();
	pub static ref alt_sa: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/alt-sa.json")).unwrap();
	pub static ref alt_ww: RegionalUsage = RegionalUsage::from_str(include_str!("region-usage-json/alt-ww.json")).unwrap();
}

impl RegionalUsage
{
	#[inline(always)]
	pub fn from_path<P: AsRef<Path>>(canIUseRegionalUsageDatabaseFilePath: P) -> Result<Self, Box<::std::error::Error>>
	{
		Self::from_reader(File::open(canIUseRegionalUsageDatabaseFilePath)?)
	}
	
	#[inline(always)]
	pub fn from_reader<R: Read>(readerOfStreamOfCanIUseJsonBytes: R) -> Result<Self, Box<::std::error::Error>>
	{
		Ok(serde_json::from_reader(readerOfStreamOfCanIUseJsonBytes)?)
	}
	
	#[inline(always)]
	pub fn from_slice(rawCanIUseJsonBytes: &[u8]) -> Result<Self, ::serde_json::error::Error>
	{
		Ok(serde_json::from_slice(rawCanIUseJsonBytes)?)
	}
	
	/// ISO-like code
	#[inline(always)]
	pub fn identifier(&self) -> &str
	{
		&self.id
	}
	
	/// Country name or similar
	#[inline(always)]
	pub fn country_or_region_name(&self) -> &str
	{
		&self.name
	}
	
	/// Total usage; may not add up to 100% (eg for Andorra, adds up to about 95%)
	#[inline(always)]
	pub fn total(&self) -> UsagePercentage
	{
		self.total
	}
	
	/// Usage; returns None if agentName has no known usages
	#[inline(always)]
	pub fn usage(&self, agentName: &AgentName, lowerBound: Bound<&Version>, upperBound: Bound<&Version>) -> Option<Range<Version, Option<UsagePercentage>>>
	{
		match self.data.get(agentName)
		{
			None => None,
			Some(entry) => Some(entry.range((lowerBound, upperBound)))
		}
	}
}
