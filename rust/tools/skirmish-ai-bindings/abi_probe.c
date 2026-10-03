/* GPL-2.0-or-later. Compare the native compiler's ABI to the Rust binding layouts. */
#include <stddef.h>
#include <stdio.h>
#include "ExternalAI/Interface/AISEvents.h"
#include "ExternalAI/Interface/AISCommands.h"
#define SIZE(T) printf("%s.size=%zu\n", #T, sizeof(struct T))
#define OFFSET(T,F) printf("%s.%s=%zu\n", #T, #F, offsetof(struct T, F))
int main(void) {
 SIZE(SSkirmishAICallback); OFFSET(SSkirmishAICallback, Engine_handleCommand);
 OFFSET(SSkirmishAICallback, Unit_getVel); OFFSET(SSkirmishAICallback, Unit_getRulesParamFloat);
 OFFSET(SSkirmishAICallback, getEnemyUnits); OFFSET(SSkirmishAICallback, SkirmishAI_Info_getValueByKey);
 SIZE(SInitEvent); OFFSET(SInitEvent, callback); OFFSET(SInitEvent, savedGame);
 SIZE(SUnitDamagedEvent); OFFSET(SUnitDamagedEvent, dir_posF3); OFFSET(SUnitDamagedEvent, paralyzer);
 SIZE(SCustomUnitCommand); OFFSET(SCustomUnitCommand, options); OFFSET(SCustomUnitCommand, params);
 return 0;
}
