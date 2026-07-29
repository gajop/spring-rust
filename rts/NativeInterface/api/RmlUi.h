#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RmlCreateContextQuery { const char* name; };
struct RmlCreateContextResult { const Error* error; uint64_t contextHandle; bool success; };

struct RmlGetContextQuery { const char* name; };
struct RmlGetContextResult { const Error* error; uint64_t contextHandle; bool exists; };

struct RmlRemoveContextQuery { uint64_t contextHandle; };
struct RmlRemoveContextResult { const Error* error; bool success; };

struct RmlRemoveContextByNameQuery { const char* name; };
struct RmlRemoveContextByNameResult { const Error* error; bool success; };

struct RmlSetDebugContextQuery { uint64_t contextHandle; };
struct RmlSetDebugContextResult { const Error* error; bool success; };

struct RmlSetDebugContextByNameQuery { const char* name; };
struct RmlSetDebugContextByNameResult { const Error* error; bool success; };

struct RmlLoadFontFaceQuery { const char* filePath; bool fallback; int32_t weight; bool hasWeight; };
struct RmlLoadFontFaceResult { const Error* error; bool success; };

struct RmlRegisterEventTypeQuery { const char* eventType; bool interruptible; bool bubbles; int32_t defaultPhase; bool hasDefaultPhase; };
struct RmlRegisterEventTypeResult { const Error* error; int32_t eventID; };

struct RmlAddTranslationStringQuery { const char* key; const char* translation; };
struct RmlAddTranslationStringResult { const Error* error; bool success; };

struct RmlClearTranslationsQuery { uint8_t _unused; };
struct RmlClearTranslationsResult { const Error* error; bool success; };

struct RmlSetMouseCursorAliasQuery { const char* rmlName; const char* recoilName; };
struct RmlSetMouseCursorAliasResult { const Error* error; bool success; };

struct RmlGetVersionQuery { uint8_t _unused; };
struct RmlGetVersionResult { const Error* error; const char* version; };

struct RmlIsReadyQuery { uint8_t _unused; };
struct RmlIsReadyResult { const Error* error; bool ready; };

struct RmlContextCreateDocumentQuery { uint64_t contextHandle; const char* tag; };
struct RmlContextCreateDocumentResult { const Error* error; uint64_t documentHandle; bool success; };

struct RmlContextLoadDocumentQuery { uint64_t contextHandle; const char* documentPath; };
struct RmlContextLoadDocumentResult { const Error* error; uint64_t documentHandle; bool success; };

struct RmlContextGetDocumentQuery { uint64_t contextHandle; const char* name; };
struct RmlContextGetDocumentResult { const Error* error; uint64_t documentHandle; bool exists; };

struct RmlContextBoolResult { const Error* error; bool success; };
struct RmlContextHandleQuery { uint64_t contextHandle; };
struct RmlContextDocumentQuery { uint64_t contextHandle; uint64_t documentHandle; };
struct RmlContextBoolQuery { uint64_t contextHandle; bool value; };
struct RmlContextStringBoolQuery { uint64_t contextHandle; const char* name; bool value; };
struct RmlContextStringQuery { uint64_t contextHandle; const char* name; };
struct RmlContextMouseMoveQuery { uint64_t contextHandle; float x; float y; int32_t keyModifierState; };
struct RmlContextMouseButtonQuery { uint64_t contextHandle; int32_t button; int32_t keyModifierState; };
struct RmlContextMouseWheelQuery { uint64_t contextHandle; float x; float y; int32_t keyModifierState; };
struct RmlContextKeyQuery { uint64_t contextHandle; int32_t key; int32_t keyModifierState; };
struct RmlContextTextInputQuery { uint64_t contextHandle; const char* text; };
struct RmlContextGetElementAtPointQuery { uint64_t contextHandle; float x; float y; uint64_t ignoreElementHandle; };
struct RmlContextGetElementAtPointResult { const Error* error; uint64_t elementHandle; bool exists; };
struct RmlContextGetElementResult { const Error* error; uint64_t elementHandle; bool exists; };
struct RmlContextGetNameResult { const Error* error; const char* name; };
struct RmlContextGetDimensionsResult { const Error* error; int32_t x; int32_t y; };
struct RmlContextSetDimensionsQuery { uint64_t contextHandle; int32_t x; int32_t y; };
struct RmlContextPointerCaptureQuery { uint64_t contextHandle; int32_t anchorX; int32_t anchorY; bool active; };
// status: 0 = none, 1 = active, 2 = released, 3 = cancelled.
struct RmlContextPointerDeltaResult { const Error* error; int32_t deltaX; int32_t deltaY; int32_t status; };
struct RmlContextGetFloatResult { const Error* error; float value; };
struct RmlContextSetFloatQuery { uint64_t contextHandle; float value; };

struct RmlDocumentHandleQuery { uint64_t documentHandle; };
struct RmlDocumentBoolResult { const Error* error; bool success; };
struct RmlDocumentShowQuery { uint64_t documentHandle; int32_t modal; bool hasModal; int32_t focus; bool hasFocus; };
struct RmlDocumentCreateElementQuery { uint64_t documentHandle; const char* tagName; };
struct RmlDocumentCreateElementResult { const Error* error; uint64_t elementPtrHandle; bool success; };
struct RmlDocumentStringQuery { uint64_t documentHandle; const char* value; };
struct RmlDocumentInlineScriptQuery { uint64_t documentHandle; const char* content; const char* sourcePath; int32_t sourceLine; };
struct RmlDocumentGetStringResult { const Error* error; const char* value; };
struct RmlDocumentSetTitleQuery { uint64_t documentHandle; const char* title; };
struct RmlDocumentGetContextResult { const Error* error; uint64_t contextHandle; bool exists; };

struct RmlElementHandleQuery { uint64_t elementHandle; };
struct RmlElementBoolResult { const Error* error; bool success; };
struct RmlElementStringQuery { uint64_t elementHandle; const char* value; };
struct RmlElementStringBoolQuery { uint64_t elementHandle; const char* name; bool value; };
struct RmlElementSetAttributeQuery { uint64_t elementHandle; const char* name; const char* value; };
struct RmlElementGetAttributeQuery { uint64_t elementHandle; const char* name; };
struct RmlElementGetAttributeResult { const Error* error; const char* value; bool exists; };
struct RmlElementGetByStringQuery { uint64_t elementHandle; const char* value; };
struct RmlElementGetElementResult { const Error* error; uint64_t elementHandle; bool exists; };
struct RmlElementGetChildQuery { uint64_t elementHandle; int32_t index; };
struct RmlElementPointQuery { uint64_t elementHandle; float x; float y; };
struct RmlElementAppendChildQuery { uint64_t elementHandle; uint64_t elementPtrHandle; };
struct RmlElementInsertBeforeQuery { uint64_t elementHandle; uint64_t elementPtrHandle; uint64_t adjacentElementHandle; };
struct RmlElementReplaceChildQuery { uint64_t elementHandle; uint64_t elementPtrHandle; uint64_t replacedElementHandle; };
struct RmlElementChildQuery { uint64_t elementHandle; uint64_t childElementHandle; };
struct RmlElementScrollIntoViewQuery { uint64_t elementHandle; bool alignWithTop; };
struct RmlElementGetStringResult { const Error* error; const char* value; };
struct RmlElementStringListResult { const Error* error; const char** values; uint32_t valueCount; };
struct RmlElementHandleListResult { const Error* error; uint64_t* elementHandles; uint32_t elementHandleCount; };
struct RmlElementSetStringQuery { uint64_t elementHandle; const char* value; };
struct RmlElementGetIntResult { const Error* error; int32_t value; };
/// Element geometry, in context (screen) coordinates. `left`/`top` are the
/// element's absolute offset; `width`/`height` are its border box.
struct RmlElementGetRectResult { const Error* error; float left; float top; float width; float height; };
struct RmlElementSetIntQuery { uint64_t elementHandle; int32_t value; };
struct RmlElementGetFloatResult { const Error* error; float value; };
struct RmlElementSetFloatQuery { uint64_t elementHandle; float value; };

struct RmlElementFormSubmitQuery { uint64_t elementHandle; const char* name; const char* value; };
struct RmlElementFormControlSelectAddQuery { uint64_t elementHandle; uint64_t elementPtrHandle; int32_t before; };
struct RmlElementFormControlSelectRemoveQuery { uint64_t elementHandle; int32_t index; };
struct RmlElementFormControlSelectionQuery { uint64_t elementHandle; int32_t start; int32_t end; };
struct RmlElementFormControlSelectionResult { const Error* error; int32_t start; int32_t end; bool success; };
struct RmlElementTabSetIndexStringQuery { uint64_t elementHandle; int32_t index; const char* rml; };
struct RmlElementTabSetIndexQuery { uint64_t elementHandle; int32_t index; };
struct RmlVector2fNewQuery { float x; float y; };
struct RmlVector2fNewResult { const Error* error; float x; float y; };
struct RmlVector2iNewQuery { int32_t x; int32_t y; };
struct RmlVector2iNewResult { const Error* error; int32_t x; int32_t y; };
struct RmlElementDispatchEventQuery { uint64_t elementHandle; const char* event; };
// `destroyCallback` releases the callback data after RmlUi detaches the
// listener. It must run before the module that supplied `callback` is unloaded.
struct RmlEventListenerCallbackQuery { uint64_t elementHandle; const char* event; bool inCapturePhase; NativeCallback callback; void* userData; NativeCallback destroyCallback; };
struct RmlEventListenerCallbackResult { const Error* error; uint64_t eventListenerHandle; bool success; };
struct RmlContextEventListenerCallbackQuery { uint64_t contextHandle; const char* event; bool inCapturePhase; NativeCallback callback; void* userData; NativeCallback destroyCallback; };
struct RmlEventListenerHandleQuery { uint64_t eventListenerHandle; };
struct RmlEventListenerElementQuery { uint64_t eventListenerHandle; uint64_t elementHandle; };
struct RmlEventListenerEventQuery { uint64_t eventListenerHandle; uint64_t eventHandle; };
struct RmlEventHandleQuery { uint64_t eventHandle; };
struct RmlEventCurrentQuery { uint8_t _unused; };
struct RmlEventCurrentResult { const Error* error; uint64_t eventHandle; uint64_t elementHandle; uint64_t documentHandle; bool exists; };
struct RmlEventParameterQuery { uint64_t eventHandle; const char* name; };
struct RmlEventGetStringResult { const Error* error; const char* value; bool exists; };
struct RmlEventGetIntResult { const Error* error; int32_t value; bool exists; };
struct RmlEventGetFloatResult { const Error* error; float value; bool exists; };
struct RmlEventGetBoolResult { const Error* error; bool value; bool exists; };
struct RmlElementProcessDefaultActionQuery { uint64_t elementHandle; uint64_t eventHandle; };
struct RmlContextOpenDataModelQuery { uint64_t contextHandle; const char* name; };
struct RmlContextOpenDataModelResult { const Error* error; uint64_t dataModelHandle; bool success; };
struct RmlSolLuaDataModelSetDirtyQuery { uint64_t dataModelHandle; const char* property; };
// Typed data-model values are owned by the engine.  The native module only
// receives opaque handles, so it can never lend Rust allocations across the
// ABI or encode a model as a string payload.
struct RmlContextCreateDataModelQuery { uint64_t contextHandle; const char* name; };
struct RmlDataModelBindBoolQuery { uint64_t dataModelHandle; const char* name; bool initialValue; };
struct RmlDataModelBindIntQuery { uint64_t dataModelHandle; const char* name; int32_t initialValue; };
struct RmlDataModelBindFloatQuery { uint64_t dataModelHandle; const char* name; float initialValue; };
struct RmlDataModelBindStringQuery { uint64_t dataModelHandle; const char* name; const char* initialValue; };
struct RmlDataModelBindResult { const Error* error; uint64_t variableHandle; bool success; };
// A native-owned collection of text rows. The engine copies every row during
// SetTextRows, so pointers are valid only for that call and never become part
// of the data model's lifetime.
struct RmlDataTextRow { const char* text; bool muted; bool visible; };
struct RmlDataModelBindTextRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetTextRowsQuery { uint64_t rowsHandle; const RmlDataTextRow* rows; uint64_t count; };
struct RmlDataModelTextRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
// A native-owned console log row. Severity and selection are semantic fields,
// so callers do not compose presentation classes or row markup.
struct RmlDataLogRow { const char* text; uint8_t severity; bool selected; };
struct RmlDataModelBindLogRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetLogRowsQuery { uint64_t rowsHandle; const RmlDataLogRow* rows; uint64_t count; };
struct RmlDataModelLogRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
// A native-owned collection for toast notifications. `hasProgress` makes an
// optional progress bar explicit without overloading a sentinel float value.
struct RmlDataNotificationRow { const char* title; const char* body; bool warning; bool hasProgress; float progress; };
struct RmlDataModelBindNotificationRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetNotificationRowsQuery { uint64_t rowsHandle; const RmlDataNotificationRow* rows; uint64_t count; };
struct RmlDataModelNotificationRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
// A native-owned collection for controls that combine a caption, icon, and
// tooltip. This is a reusable visual row shape; it avoids generated RML for
// toolbars and icon-button pickers while retaining typed native values.
struct RmlDataIconRow { const char* label; const char* icon; const char* tooltip; bool pressed; bool disabled; };
struct RmlDataModelBindIconRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetIconRowsQuery { uint64_t rowsHandle; const RmlDataIconRow* rows; uint64_t count; };
struct RmlDataModelIconRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
// A native-owned collection for select options. Labels and stored values are
// separate so UI wording never leaks into the command value.
struct RmlDataOptionRow { const char* value; const char* label; };
struct RmlDataModelBindOptionRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetOptionRowsQuery { uint64_t rowsHandle; const RmlDataOptionRow* rows; uint64_t count; };
struct RmlDataModelOptionRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
// A native-owned collection for a selectable label/detail list. This is useful
// for command palettes and other two-column choices without generating RML.
struct RmlDataChoiceRow { const char* label; const char* detail; bool selected; bool highlighted; };
struct RmlDataModelBindChoiceRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetChoiceRowsQuery { uint64_t rowsHandle; const RmlDataChoiceRow* rows; uint64_t count; };
struct RmlDataModelChoiceRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
// A labelled boolean status is intentionally distinct from a choice: callers
// cannot accidentally interpret a display-only availability fact as input.
struct RmlDataStatusRow { const char* label; bool positive; };
struct RmlDataModelBindStatusRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetStatusRowsQuery { uint64_t rowsHandle; const RmlDataStatusRow* rows; uint64_t count; };
struct RmlDataModelStatusRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
// A native-owned presentation row with a label, colour swatch, and optional
// actions. Colours cross the ABI as channels, never as CSS snippets; the
// engine turns them into RmlUi's native colour value when the model is read.
struct RmlDataSwatchRow { const char* label; uint8_t red; uint8_t green; uint8_t blue; uint8_t alpha; bool actionsEnabled; };
struct RmlDataModelBindSwatchRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetSwatchRowsQuery { uint64_t rowsHandle; const RmlDataSwatchRow* rows; uint64_t count; };
struct RmlDataModelSwatchRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
// A native-owned image-grid row. The image path remains a normal string value,
// while rendering mode and presentation state are explicit booleans rather
// than generated RML or CSS fragments.
struct RmlDataGridRow { const char* label; const char* image; float cellSize; bool hasImage; bool nativeImage; bool selected; bool folder; bool filler; };
struct RmlDataModelBindGridRowsQuery { uint64_t dataModelHandle; const char* name; };
struct RmlDataModelSetGridRowsQuery { uint64_t rowsHandle; const RmlDataGridRow* rows; uint64_t count; };
struct RmlDataModelGridRowsResult { const Error* error; uint64_t rowsHandle; bool success; };
struct RmlDataModelVariableBoolQuery { uint64_t variableHandle; bool value; };
struct RmlDataModelVariableIntQuery { uint64_t variableHandle; int32_t value; };
struct RmlDataModelVariableFloatQuery { uint64_t variableHandle; float value; };
struct RmlDataModelVariableStringQuery { uint64_t variableHandle; const char* value; };
// A native colour is carried through the ABI as channels. RmlUi receives its
// own Colourb value inside the engine; callers never construct CSS strings.
struct RmlDataModelBindColorQuery { uint64_t dataModelHandle; const char* name; uint8_t red; uint8_t green; uint8_t blue; uint8_t alpha; };
struct RmlDataModelVariableColorQuery { uint64_t variableHandle; uint8_t red; uint8_t green; uint8_t blue; uint8_t alpha; };
// A pixel length remains a number across the ABI. The engine supplies the
// required RmlUi unit when it is used in a style binding.
struct RmlDataModelBindPixelsQuery { uint64_t dataModelHandle; const char* name; float initialValue; };
struct RmlDataModelVariablePixelsQuery { uint64_t variableHandle; float value; };
// A percentage remains a scalar across the ABI. The engine attaches `%` only
// when RmlUi consumes it as a style, so native callers never compose CSS.
struct RmlDataModelBindPercentQuery { uint64_t dataModelHandle; const char* name; float initialValue; };
struct RmlDataModelVariablePercentQuery { uint64_t variableHandle; float value; };
struct RmlDataModelVariableHandleQuery { uint64_t variableHandle; };
struct RmlDataModelGetBoolResult { const Error* error; bool value; bool success; };
struct RmlDataModelGetIntResult { const Error* error; int32_t value; bool success; };
struct RmlDataModelGetFloatResult { const Error* error; float value; bool success; };
struct RmlDataModelGetStringResult { const Error* error; const char* value; bool success; };
struct RmlDataModelGetColorResult { const Error* error; uint8_t red; uint8_t green; uint8_t blue; uint8_t alpha; bool success; };
struct RmlDataModelGetPixelsResult { const Error* error; float value; bool success; };
struct RmlDataModelGetPercentResult { const Error* error; float value; bool success; };

struct RmlUiApi {
	void (*CreateContext)(const RmlCreateContextQuery* query, RmlCreateContextResult* result);
	void (*GetContext)(const RmlGetContextQuery* query, RmlGetContextResult* result);
	void (*RemoveContext)(const RmlRemoveContextQuery* query, RmlRemoveContextResult* result);
	void (*RemoveContextByName)(const RmlRemoveContextByNameQuery* query, RmlRemoveContextByNameResult* result);
	void (*SetDebugContext)(const RmlSetDebugContextQuery* query, RmlSetDebugContextResult* result);
	void (*SetDebugContextByName)(const RmlSetDebugContextByNameQuery* query, RmlSetDebugContextByNameResult* result);
	void (*LoadFontFace)(const RmlLoadFontFaceQuery* query, RmlLoadFontFaceResult* result);
	void (*RegisterEventType)(const RmlRegisterEventTypeQuery* query, RmlRegisterEventTypeResult* result);
	void (*RegiserEventType)(const RmlRegisterEventTypeQuery* query, RmlRegisterEventTypeResult* result);
	void (*AddTranslationString)(const RmlAddTranslationStringQuery* query, RmlAddTranslationStringResult* result);
	void (*ClearTranslations)(const RmlClearTranslationsQuery* query, RmlClearTranslationsResult* result);
	void (*SetMouseCursorAlias)(const RmlSetMouseCursorAliasQuery* query, RmlSetMouseCursorAliasResult* result);
	void (*GetVersion)(const RmlGetVersionQuery* query, RmlGetVersionResult* result);
	void (*IsReady)(const RmlIsReadyQuery* query, RmlIsReadyResult* result);
	void (*ContextCreateDocument)(const RmlContextCreateDocumentQuery* query, RmlContextCreateDocumentResult* result);
	void (*ContextLoadDocument)(const RmlContextLoadDocumentQuery* query, RmlContextLoadDocumentResult* result);
	void (*ContextGetDocument)(const RmlContextGetDocumentQuery* query, RmlContextGetDocumentResult* result);
	void (*ContextAddEventListener)(const RmlContextEventListenerCallbackQuery* query, RmlEventListenerCallbackResult* result);
	void (*ContextRender)(const RmlContextHandleQuery* query, RmlContextBoolResult* result);
	void (*ContextUnloadAllDocuments)(const RmlContextHandleQuery* query, RmlContextBoolResult* result);
	void (*ContextUnloadDocument)(const RmlContextDocumentQuery* query, RmlContextBoolResult* result);
	void (*ContextUpdate)(const RmlContextHandleQuery* query, RmlContextBoolResult* result);
	void (*ContextOpenDataModel)(const RmlContextOpenDataModelQuery* query, RmlContextOpenDataModelResult* result);
	void (*ContextCreateDataModel)(const RmlContextCreateDataModelQuery* query, RmlContextOpenDataModelResult* result);
	void (*ContextRemoveDataModel)(const RmlContextStringQuery* query, RmlContextBoolResult* result);
	void (*ContextProcessMouseMove)(const RmlContextMouseMoveQuery* query, RmlContextBoolResult* result);
	void (*ContextProcessMouseButtonDown)(const RmlContextMouseButtonQuery* query, RmlContextBoolResult* result);
	void (*ContextProcessMouseButtonUp)(const RmlContextMouseButtonQuery* query, RmlContextBoolResult* result);
	void (*ContextProcessMouseWheel)(const RmlContextMouseWheelQuery* query, RmlContextBoolResult* result);
	void (*ContextProcessMouseLeave)(const RmlContextHandleQuery* query, RmlContextBoolResult* result);
	void (*ContextIsMouseInteracting)(const RmlContextHandleQuery* query, RmlContextBoolResult* result);
	void (*ContextProcessKeyDown)(const RmlContextKeyQuery* query, RmlContextBoolResult* result);
	void (*ContextProcessKeyUp)(const RmlContextKeyQuery* query, RmlContextBoolResult* result);
	void (*ContextProcessTextInput)(const RmlContextTextInputQuery* query, RmlContextBoolResult* result);
	void (*ContextEnableMouseCursor)(const RmlContextBoolQuery* query, RmlContextBoolResult* result);
	void (*ContextActivateTheme)(const RmlContextStringBoolQuery* query, RmlContextBoolResult* result);
	void (*ContextIsThemeActive)(const RmlContextStringQuery* query, RmlContextBoolResult* result);
	void (*ContextGetElementAtPoint)(const RmlContextGetElementAtPointQuery* query, RmlContextGetElementAtPointResult* result);
	void (*ContextPullDocumentToFront)(const RmlContextDocumentQuery* query, RmlContextBoolResult* result);
	void (*ContextPushDocumentToBack)(const RmlContextDocumentQuery* query, RmlContextBoolResult* result);
	void (*ContextGetName)(const RmlContextHandleQuery* query, RmlContextGetNameResult* result);
	void (*ContextGetDimensions)(const RmlContextHandleQuery* query, RmlContextGetDimensionsResult* result);
	void (*ContextSetDimensions)(const RmlContextSetDimensionsQuery* query, RmlContextBoolResult* result);
	void (*ContextGetDensityIndependentPixelRatio)(const RmlContextHandleQuery* query, RmlContextGetFloatResult* result);
	void (*ContextSetDensityIndependentPixelRatio)(const RmlContextSetFloatQuery* query, RmlContextBoolResult* result);
	void (*ContextGetFocusElement)(const RmlContextHandleQuery* query, RmlContextGetElementResult* result);
	void (*ContextGetHoverElement)(const RmlContextHandleQuery* query, RmlContextGetElementResult* result);
	void (*ContextGetRootElement)(const RmlContextHandleQuery* query, RmlContextGetElementResult* result);
	void (*DocumentPullToFront)(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentPushToBack)(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentShow)(const RmlDocumentShowQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentHide)(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentClose)(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentCreateElement)(const RmlDocumentCreateElementQuery* query, RmlDocumentCreateElementResult* result);
	void (*DocumentCreateTextNode)(const RmlDocumentStringQuery* query, RmlDocumentCreateElementResult* result);
	void (*DocumentReloadStyleSheet)(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentLoadInlineScript)(const RmlDocumentInlineScriptQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentLoadExternalScript)(const RmlDocumentStringQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentUpdateDocument)(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentAppendToStyleSheet)(const RmlDocumentStringQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentGetTitle)(const RmlDocumentHandleQuery* query, RmlDocumentGetStringResult* result);
	void (*DocumentSetTitle)(const RmlDocumentSetTitleQuery* query, RmlDocumentBoolResult* result);
	void (*DocumentGetContext)(const RmlDocumentHandleQuery* query, RmlDocumentGetContextResult* result);
	void (*DocumentGetUrl)(const RmlDocumentHandleQuery* query, RmlDocumentGetStringResult* result);
	void (*DocumentIsModal)(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result);
	void (*ElementAppendChild)(const RmlElementAppendChildQuery* query, RmlElementGetElementResult* result);
	void (*ElementAddEventListener)(const RmlEventListenerCallbackQuery* query, RmlEventListenerCallbackResult* result);
	void (*ElementBlur)(const RmlElementHandleQuery* query, RmlElementBoolResult* result);
	void (*ElementClick)(const RmlElementHandleQuery* query, RmlElementBoolResult* result);
	void (*ElementFocus)(const RmlElementHandleQuery* query, RmlElementBoolResult* result);
	void (*ElementGetAttribute)(const RmlElementGetAttributeQuery* query, RmlElementGetAttributeResult* result);
	void (*ElementGetElementById)(const RmlElementGetByStringQuery* query, RmlElementGetElementResult* result);
	void (*ElementQuerySelector)(const RmlElementGetByStringQuery* query, RmlElementGetElementResult* result);
	void (*ElementHasAttribute)(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result);
	void (*ElementHasChildNodes)(const RmlElementHandleQuery* query, RmlElementBoolResult* result);
	void (*ElementInsertBefore)(const RmlElementInsertBeforeQuery* query, RmlElementGetElementResult* result);
	void (*ElementIsClassSet)(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result);
	void (*ElementRemoveAttribute)(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result);
	void (*ElementRemoveChild)(const RmlElementChildQuery* query, RmlDocumentCreateElementResult* result);
	void (*ElementReplaceChild)(const RmlElementReplaceChildQuery* query, RmlDocumentCreateElementResult* result);
	void (*ElementScrollIntoView)(const RmlElementScrollIntoViewQuery* query, RmlElementBoolResult* result);
	void (*ElementSetAttribute)(const RmlElementSetAttributeQuery* query, RmlElementBoolResult* result);
	void (*ElementSetClass)(const RmlElementStringBoolQuery* query, RmlElementBoolResult* result);
	void (*ElementGetElementsByClassNameCount)(const RmlElementGetByStringQuery* query, RmlElementGetIntResult* result);
	void (*ElementGetElementsByTagNameCount)(const RmlElementGetByStringQuery* query, RmlElementGetIntResult* result);
	void (*ElementQuerySelectorAllCount)(const RmlElementGetByStringQuery* query, RmlElementGetIntResult* result);
	void (*ElementGetElementsByClassName)(const RmlElementGetByStringQuery* query, RmlElementHandleListResult* result);
	void (*ElementGetElementsByTagName)(const RmlElementGetByStringQuery* query, RmlElementHandleListResult* result);
	void (*ElementQuerySelectorAll)(const RmlElementGetByStringQuery* query, RmlElementHandleListResult* result);
	void (*ElementClone)(const RmlElementHandleQuery* query, RmlDocumentCreateElementResult* result);
	void (*ElementClosest)(const RmlElementGetByStringQuery* query, RmlElementGetElementResult* result);
	void (*ElementSetPseudoClass)(const RmlElementStringBoolQuery* query, RmlElementBoolResult* result);
	void (*ElementIsPseudoClassSet)(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result);
	void (*ElementArePseudoClassesSet)(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result);
	void (*ElementGetActivePseudoClasses)(const RmlElementHandleQuery* query, RmlElementStringListResult* result);
	void (*ElementIsPointWithinElement)(const RmlElementPointQuery* query, RmlElementBoolResult* result);
	/// Absolute offset and border-box size of an element, in context coordinates.
	/// Needed to map a cursor position onto an element (colour pickers, sliders).
	void (*ElementGetRect)(const RmlElementHandleQuery* query, RmlElementGetRectResult* result);
	void (*ElementMatches)(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result);
	void (*ElementDispatchEvent)(const RmlElementDispatchEventQuery* query, RmlElementBoolResult* result);
	void (*ElementProcessDefaultAction)(const RmlElementProcessDefaultActionQuery* query, RmlElementBoolResult* result);
	void (*ElementGetValue)(const RmlElementHandleQuery* query, RmlElementGetStringResult* result);
	void (*ElementGetChild)(const RmlElementGetChildQuery* query, RmlElementGetElementResult* result);
	void (*ElementGetClassName)(const RmlElementHandleQuery* query, RmlElementGetStringResult* result);
	void (*ElementSetClassName)(const RmlElementSetStringQuery* query, RmlElementBoolResult* result);
	void (*ElementGetId)(const RmlElementHandleQuery* query, RmlElementGetStringResult* result);
	void (*ElementSetId)(const RmlElementSetStringQuery* query, RmlElementBoolResult* result);
	void (*ElementGetInnerRml)(const RmlElementHandleQuery* query, RmlElementGetStringResult* result);
	void (*ElementSetInnerRml)(const RmlElementSetStringQuery* query, RmlElementBoolResult* result);
	void (*ElementGetScrollLeft)(const RmlElementHandleQuery* query, RmlElementGetIntResult* result);
	void (*ElementSetScrollLeft)(const RmlElementSetIntQuery* query, RmlElementBoolResult* result);
	void (*ElementGetScrollTop)(const RmlElementHandleQuery* query, RmlElementGetIntResult* result);
	void (*ElementSetScrollTop)(const RmlElementSetIntQuery* query, RmlElementBoolResult* result);
	void (*ElementGetTagName)(const RmlElementHandleQuery* query, RmlElementGetStringResult* result);
	void (*ElementIsVisible)(const RmlElementHandleQuery* query, RmlElementBoolResult* result);
	void (*ElementFormSubmit)(const RmlElementFormSubmitQuery* query, RmlElementBoolResult* result);
	void (*ElementFormControlSelectAdd)(const RmlElementFormControlSelectAddQuery* query, RmlElementBoolResult* result);
	void (*ElementFormControlSelectRemove)(const RmlElementFormControlSelectRemoveQuery* query, RmlElementBoolResult* result);
	void (*ElementFormControlSelectRemoveAll)(const RmlElementHandleQuery* query, RmlElementBoolResult* result);
	void (*ElementFormControlInputSelect)(const RmlElementHandleQuery* query, RmlElementBoolResult* result);
	void (*ElementFormControlInputSetSelection)(const RmlElementFormControlSelectionQuery* query, RmlElementBoolResult* result);
	void (*ElementFormControlInputGetSelection)(const RmlElementHandleQuery* query, RmlElementFormControlSelectionResult* result);
	void (*ElementFormControlTextAreaSelect)(const RmlElementHandleQuery* query, RmlElementBoolResult* result);
	void (*ElementFormControlTextAreaSetSelection)(const RmlElementFormControlSelectionQuery* query, RmlElementBoolResult* result);
	void (*ElementFormControlTextAreaGetSelection)(const RmlElementHandleQuery* query, RmlElementFormControlSelectionResult* result);
	void (*ElementTabSetSetPanel)(const RmlElementTabSetIndexStringQuery* query, RmlElementBoolResult* result);
	void (*ElementTabSetSetTab)(const RmlElementTabSetIndexStringQuery* query, RmlElementBoolResult* result);
	void (*ElementTabSetRemoveTab)(const RmlElementTabSetIndexQuery* query, RmlElementBoolResult* result);
	void (*EventListenerOnAttach)(const RmlEventListenerElementQuery* query, RmlElementBoolResult* result);
	void (*EventListenerOnDetach)(const RmlEventListenerElementQuery* query, RmlElementBoolResult* result);
	void (*EventListenerProcessEvent)(const RmlEventListenerEventQuery* query, RmlElementBoolResult* result);
	void (*EventGetCurrent)(const RmlEventCurrentQuery* query, RmlEventCurrentResult* result);
	void (*EventStopPropagation)(const RmlEventHandleQuery* query, RmlElementBoolResult* result);
	void (*EventStopImmediatePropagation)(const RmlEventHandleQuery* query, RmlElementBoolResult* result);
	void (*EventGetCurrentElement)(const RmlEventHandleQuery* query, RmlElementGetElementResult* result);
	void (*EventGetTargetElement)(const RmlEventHandleQuery* query, RmlElementGetElementResult* result);
	void (*EventGetType)(const RmlEventHandleQuery* query, RmlEventGetStringResult* result);
	void (*EventGetPhase)(const RmlEventHandleQuery* query, RmlEventGetIntResult* result);
	void (*EventIsInterruptible)(const RmlEventHandleQuery* query, RmlEventGetBoolResult* result);
	void (*EventIsPropagating)(const RmlEventHandleQuery* query, RmlEventGetBoolResult* result);
	void (*EventIsImmediatePropagating)(const RmlEventHandleQuery* query, RmlEventGetBoolResult* result);
	void (*EventGetParameterType)(const RmlEventParameterQuery* query, RmlEventGetIntResult* result);
	void (*EventGetParameterBool)(const RmlEventParameterQuery* query, RmlEventGetBoolResult* result);
	void (*EventGetParameterInt)(const RmlEventParameterQuery* query, RmlEventGetIntResult* result);
	void (*EventGetParameterFloat)(const RmlEventParameterQuery* query, RmlEventGetFloatResult* result);
	void (*EventGetParameterString)(const RmlEventParameterQuery* query, RmlEventGetStringResult* result);
	void (*SolLuaDataModelSetDirty)(const RmlSolLuaDataModelSetDirtyQuery* query, RmlElementBoolResult* result);
	void (*DataModelBindBool)(const RmlDataModelBindBoolQuery* query, RmlDataModelBindResult* result);
	void (*DataModelBindInt)(const RmlDataModelBindIntQuery* query, RmlDataModelBindResult* result);
	void (*DataModelBindFloat)(const RmlDataModelBindFloatQuery* query, RmlDataModelBindResult* result);
	void (*DataModelBindString)(const RmlDataModelBindStringQuery* query, RmlDataModelBindResult* result);
	void (*DataModelBindTextRows)(const RmlDataModelBindTextRowsQuery* query, RmlDataModelTextRowsResult* result);
	void (*DataModelSetBool)(const RmlDataModelVariableBoolQuery* query, RmlElementBoolResult* result);
	void (*DataModelSetInt)(const RmlDataModelVariableIntQuery* query, RmlElementBoolResult* result);
	void (*DataModelSetFloat)(const RmlDataModelVariableFloatQuery* query, RmlElementBoolResult* result);
	void (*DataModelSetString)(const RmlDataModelVariableStringQuery* query, RmlElementBoolResult* result);
	void (*DataModelSetTextRows)(const RmlDataModelSetTextRowsQuery* query, RmlElementBoolResult* result);
	void (*DataModelGetBool)(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetBoolResult* result);
	void (*DataModelGetInt)(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetIntResult* result);
	void (*DataModelGetFloat)(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetFloatResult* result);
	void (*DataModelGetString)(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetStringResult* result);
	void (*Vector2fNew)(const RmlVector2fNewQuery* query, RmlVector2fNewResult* result);
	void (*Vector2iNew)(const RmlVector2iNewQuery* query, RmlVector2iNewResult* result);
	// Keep additions at the end: native modules compiled against an earlier
	// table retain the offsets of every pre-existing function pointer.
	void (*DataModelBindNotificationRows)(const RmlDataModelBindNotificationRowsQuery* query, RmlDataModelNotificationRowsResult* result);
	void (*DataModelSetNotificationRows)(const RmlDataModelSetNotificationRowsQuery* query, RmlElementBoolResult* result);
	void (*DataModelBindIconRows)(const RmlDataModelBindIconRowsQuery* query, RmlDataModelIconRowsResult* result);
	void (*DataModelSetIconRows)(const RmlDataModelSetIconRowsQuery* query, RmlElementBoolResult* result);
	void (*DataModelBindOptionRows)(const RmlDataModelBindOptionRowsQuery* query, RmlDataModelOptionRowsResult* result);
	void (*DataModelSetOptionRows)(const RmlDataModelSetOptionRowsQuery* query, RmlElementBoolResult* result);
	void (*DataModelBindChoiceRows)(const RmlDataModelBindChoiceRowsQuery* query, RmlDataModelChoiceRowsResult* result);
	void (*DataModelSetChoiceRows)(const RmlDataModelSetChoiceRowsQuery* query, RmlElementBoolResult* result);
	void (*DataModelBindStatusRows)(const RmlDataModelBindStatusRowsQuery* query, RmlDataModelStatusRowsResult* result);
	void (*DataModelSetStatusRows)(const RmlDataModelSetStatusRowsQuery* query, RmlElementBoolResult* result);
	void (*DataModelBindSwatchRows)(const RmlDataModelBindSwatchRowsQuery* query, RmlDataModelSwatchRowsResult* result);
	void (*DataModelSetSwatchRows)(const RmlDataModelSetSwatchRowsQuery* query, RmlElementBoolResult* result);
	void (*DataModelBindGridRows)(const RmlDataModelBindGridRowsQuery* query, RmlDataModelGridRowsResult* result);
	void (*DataModelSetGridRows)(const RmlDataModelSetGridRowsQuery* query, RmlElementBoolResult* result);
	void (*DataModelBindColor)(const RmlDataModelBindColorQuery* query, RmlDataModelBindResult* result);
	void (*DataModelSetColor)(const RmlDataModelVariableColorQuery* query, RmlElementBoolResult* result);
	void (*DataModelGetColor)(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetColorResult* result);
	void (*DataModelBindPixels)(const RmlDataModelBindPixelsQuery* query, RmlDataModelBindResult* result);
	void (*DataModelSetPixels)(const RmlDataModelVariablePixelsQuery* query, RmlElementBoolResult* result);
	void (*DataModelGetPixels)(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetPixelsResult* result);
	void (*DataModelBindPercent)(const RmlDataModelBindPercentQuery* query, RmlDataModelBindResult* result);
	void (*DataModelSetPercent)(const RmlDataModelVariablePercentQuery* query, RmlElementBoolResult* result);
	void (*DataModelGetPercent)(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetPercentResult* result);
	void (*DataModelBindLogRows)(const RmlDataModelBindLogRowsQuery* query, RmlDataModelLogRowsResult* result);
	void (*DataModelSetLogRows)(const RmlDataModelSetLogRowsQuery* query, RmlElementBoolResult* result);
	void (*ContextPullToFront)(const RmlContextHandleQuery* query, RmlContextBoolResult* result);
	void (*ContextSetPointerCapture)(const RmlContextPointerCaptureQuery* query, RmlContextBoolResult* result);
	void (*ContextTakePointerCaptureDelta)(const RmlContextHandleQuery* query, RmlContextPointerDeltaResult* result);
};

extern const RmlUiApi RMLUI_API;

#ifdef __cplusplus
}

// Native modules are hot-reloaded independently of RmlUi. The engine owns
// contexts created through this API and must destroy them before unloading a
// module, otherwise their event listeners retain callbacks into the old .so.
namespace NativeRmlUi {
	using ContextRemover = void (*)(uint64_t contextHandle);
	void ClearAllContexts(ContextRemover removeContext);
}
#endif
