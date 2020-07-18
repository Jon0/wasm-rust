extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

cfg_if! {
	// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
	// allocator.
	if #[cfg(feature = "wee_alloc")] {
		extern crate wee_alloc;
		#[global_allocator]
		static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
	}
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

	// Use `web_sys`'s global `window` function to get a handle on the global
	// window object.
	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	// Manufacture the element we're gonna append
	let val = document.create_element("p")?;
	val.set_inner_html("Hello from Rust!");

	body.append_child(&val)?;

	greet();

	test();

	Ok(())
}

#[wasm_bindgen]
extern {
	fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
	alert("Hello, wasm-game-of-life!!!!");
}


#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
	a + b
}


fn test() {
	let ctx = web_sys::AudioContext::new().unwrap();

	// Create our web audio objects.
        let primary = ctx.create_oscillator().unwrap();
        let fm_osc = ctx.create_oscillator().unwrap();
        let gain = ctx.create_gain().unwrap();
        let fm_gain = ctx.create_gain().unwrap();

        // Some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.5); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);

        // Connect the nodes up!

        // The primary oscillator is routed through the gain node, so that
        // it can control the overall output volume.
        primary.connect_with_audio_node(&gain).unwrap();

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&ctx.destination()).unwrap();

        // The FM oscillator is connected to its own gain node, so it can
        // control the amount of modulation.
        fm_osc.connect_with_audio_node(&fm_gain).unwrap();

        // Connect the FM oscillator to the frequency parameter of the main
        // oscillator, so that the FM node can modulate its frequency.
        fm_gain.connect_with_audio_param(&primary.frequency()).unwrap();

        // Start the oscillators!
        primary.start().unwrap();
        fm_osc.start().unwrap();
}

