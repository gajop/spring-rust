use spring_skirmish_ai_sys::*;
use std::mem::{offset_of,size_of};
fn main() {
 macro_rules! size {($t:ty)=>{println!("{}.size={}",stringify!($t),size_of::<$t>())};}
 macro_rules! offset {($t:ty,$f:ident)=>{println!("{}.{}={}",stringify!($t),stringify!($f),offset_of!($t,$f))};}
 size!(SSkirmishAICallback);offset!(SSkirmishAICallback,Engine_handleCommand);
 offset!(SSkirmishAICallback,Unit_getVel);offset!(SSkirmishAICallback,Unit_getRulesParamFloat);
 offset!(SSkirmishAICallback,getEnemyUnits);offset!(SSkirmishAICallback,SkirmishAI_Info_getValueByKey);
 size!(SInitEvent);offset!(SInitEvent,callback);offset!(SInitEvent,savedGame);
 size!(SUnitDamagedEvent);offset!(SUnitDamagedEvent,dir_posF3);offset!(SUnitDamagedEvent,paralyzer);
 size!(SCustomUnitCommand);offset!(SCustomUnitCommand,options);offset!(SCustomUnitCommand,params);
}
