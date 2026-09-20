;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

;; Core-Wasm RmlUi menu smoke fixture.
;; Compile with: wat2wasm rml_menu.wat -o rml_menu.wasm
(module
	;; Core dynamic-input calls use an 8-byte descriptor (pointer, byte count).
	;; The pointed-to value is itself a little-endian u32 byte length followed by
	;; the bytes. Fixed results occupy 16 bytes: u64 handle/value and a bool.
	(import "spring:rml-ui" "create-context"
		(func $rml_create_context (param i32 i32) (result i32)))
	(import "spring:rml-ui" "context-load-document"
		(func $rml_load_document (param i64 i32 i32) (result i32)))
	(import "spring:rml-ui" "document-show"
		(func $rml_show_document (param i64 i32) (result i64)))
	(import "spring:rml-ui" "element-get-element-by-id"
		(func $rml_get_element_by_id (param i64 i32 i32) (result i32)))
	(import "spring:rml-ui" "element-set-inner-rml"
		(func $rml_set_inner_rml (param i64 i32) (result i64)))
	(import "spring:rml-ui" "element-add-event-listener"
		(func $rml_add_event_listener
			(param i64 i32 i32 i32 i32 i32 i32 i32) (result i32)))
	(import "spring:rml-ui" "is-ready"
		(func $rml_is_ready (param i32) (result i64)))

	;; A menu module must opt into the menu environment (1 << 5).
	(func (export "SPRING_ENV_MASK") (result i32)
		i32.const 32)
	;; Generated variable callins negotiate a scratch range even when this
	;; fixture only uses fixed-size menu callins. Offset 256, capacity 256.
	(func (export "spring:callin/scratch-info") (result i64)
		i64.const 1099511628032)

	(memory (export "memory") 1)
	(global $context (mut i64) (i64.const 0))
	(global $document (mut i64) (i64.const 0))
	(global $button (mut i64) (i64.const 0))
	(global $status (mut i64) (i64.const 0))

	;; Length-prefixed dynamic strings, plus raw variable-input setter/event bytes.
	(data (i32.const 0) "\08\00\00\00rml-menu")
	(data (i32.const 16) "\11\00\00\00WasmMenu/menu.rml")
	(data (i32.const 40) "\06\00\00\00launch")
	(data (i32.const 52) "\06\00\00\00status")
	(data (i32.const 64) "Clicked!")
	(data (i32.const 76) "click")

	;; Descriptors: clicked text, context name, document path, button id, status id.
	(data (i32.const 88) "\40\00\00\00\08\00\00\00")
	(data (i32.const 96) "\00\00\00\00\0c\00\00\00")
	(data (i32.const 104) "\10\00\00\00\15\00\00\00")
	(data (i32.const 112) "\28\00\00\00\0a\00\00\00")
	(data (i32.const 120) "\34\00\00\00\0a\00\00\00")

	(func $packed_success (param $value i64) (result i32)
		local.get $value
		i64.const 1
		i64.and
		i64.const 1
		i64.eq
		local.get $value
		i64.const 32
		i64.shr_u
		i64.eqz
		i32.and)

	(func $set_status (result i32)
		global.get $status
		i32.const 88
		call $rml_set_inner_rml
		call $packed_success)

	(func $load_menu
		(local $status_code i32)
		(local $packed i64)

		;; Create the context and stop immediately on a Core error or failed result.
		i32.const 96
		i32.const 128
		call $rml_create_context
		local.set $status_code
		local.get $status_code
		if
			unreachable
		end
		i32.const 128
		i64.load
		global.set $context
		i32.const 136
		i32.load8_u
		if
		else
			i64.const 0
			global.set $context
			unreachable
		end

		;; Load the document from the menu archive, using its packaged path.
		global.get $context
		i32.const 104
		i32.const 144
		call $rml_load_document
		local.set $status_code
		local.get $status_code
		if
			i64.const 0
			global.set $context
			unreachable
		end
		i32.const 144
		i64.load
		global.set $document
		i32.const 152
		i32.load8_u
		if
		else
			i64.const 0
			global.set $context
			unreachable
		end

		;; Show the document with modal/focus options absent.
		global.get $document
		i32.const 208
		call $rml_show_document
		local.set $packed
		local.get $packed
		call $packed_success
		if
		else
			i64.const 0
			global.set $context
			unreachable
		end

		;; Resolve the button and status elements from the loaded document.
		global.get $document
		i32.const 112
		i32.const 160
		call $rml_get_element_by_id
		local.set $status_code
		local.get $status_code
		if
			i64.const 0
			global.set $context
			unreachable
		end
		i32.const 160
		i64.load
		global.set $button
		i32.const 168
		i32.load8_u
		if
		else
			i64.const 0
			global.set $context
			unreachable
		end

		global.get $document
		i32.const 120
		i32.const 176
		call $rml_get_element_by_id
		local.set $status_code
		local.get $status_code
		if
			i64.const 0
			global.set $context
			unreachable
		end
		i32.const 176
		i64.load
		global.set $status
		i32.const 184
		i32.load8_u
		if
		else
			i64.const 0
			global.set $context
			unreachable
		end

		;; Register a retained Core callback on the button's click event.
		global.get $button
		i32.const 76
		i32.const 5
		i32.const 0
		i32.const 1
		i32.const 0
		i32.const 0
		i32.const 192
		call $rml_add_event_listener
		local.set $status_code
		local.get $status_code
		if
			i64.const 0
			global.set $context
			unreachable
		end
		i32.const 200
		i32.load8_u
		if
		else
			i64.const 0
			global.set $context
			unreachable
		end
	)

	;; The host resolves this export when the retained event listener is added.
	(func (export "spring:callback/dispatch")
		(param $callback_id i32) (param $user_data i32)
		local.get $callback_id
		i32.const 1
		i32.eq
		if
			call $set_status
			i32.eqz
			if
				unreachable
			end
		end)

	(func (export "spring:callin/activate-menu") (param i32)
		global.get $context
		i64.eqz
		if
			call $load_menu
		end)

	(func (export "spring:callin/update") (param f32)
		(local $packed i64)
		;; Exercise the direct fixed-output call as part of every update.
		i32.const 0
		call $rml_is_ready
		local.set $packed
		local.get $packed
		i64.const 32
		i64.shr_u
		i64.eqz
		if
		else
			unreachable
		end)
)
