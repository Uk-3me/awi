// Aldaron's Window Interface
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/window_ops/mod.rs

pub trait WindowOps {
	// Create the window.
	fn new(title: &str, icon: (u32, u32, &[u32])) -> Self;
	// Show the window.
	fn show(&self) -> ();
	// Re-draw the window.
	fn update(&self) -> ();
	// Poll for events, returns true if there's more.  Adds 1+ to input.
	fn poll_event(&mut self, input: &mut ::input::InputQueue,
		wh: &mut(u32,u32), keyboard: &mut ::Keyboard) -> bool;
	// Toggle fullscreen.
	fn fullscreen(&mut self) -> ();
	// Get connection details
	fn get_connection(&self) -> ::WindowConnection;
}
