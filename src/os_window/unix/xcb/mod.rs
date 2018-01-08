// Aldaron's Window Interface
// Copyright (c) 2017-2018 Jeron Aldaron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/os_window/unix/xcb/mod.rs

mod ffi;
mod event;
mod keyboard;
mod native_connection;
mod property;

use self::native_connection::NativeConnection;
use self::property::Property;
use self::event::Event;

pub struct XcbWindow {
	native: NativeConnection,
	fullscreen: Property,
}

impl ::WindowOps for XcbWindow {
	fn new(title: &str, icon: (u32, u32, &[u32])) -> Self {
		let xcb_dl = unsafe { ffi::load_dl() };
		let native = NativeConnection::new(xcb_dl);

		if native.failed() {
			return XcbWindow {
				fullscreen: Property::dummy(), native: native
			}
		}

		let native = native.title(title);
		let native = native.icon(icon);

		XcbWindow {
			fullscreen: Property::create(native.connection(),
				b"_NET_WM_STATE", b"_NET_WM_STATE_FULLSCREEN"),
			native: native,
		}
	}

	fn show(&self) -> () {
		// Make sure 'X' button works before showing!
		Property::create(self.native.connection(), b"WM_PROTOCOLS",
				b"WM_DELETE_WINDOW")
			.catch(self.native.connection(), self.native.1);
		self.native.show()
	}

	fn update(&self) -> () {
		self.native.update()
	}

	fn poll_event(&mut self, input: &mut ::input::InputQueue,
		wh: &mut(u32,u32), keyboard: &mut ::input::keyboard::Keyboard)
		-> bool
	{
		let connection = self.native.connection();
		let keyboard_state = self.native.keyboard_state();

		Event::create(connection, keyboard_state).poll(input, wh,
			keyboard)
	}

	fn fullscreen(&mut self) -> () {
		self.fullscreen.apply(self.native.connection(), self.native.1)
	}

	fn get_connection(&self) -> ::WindowConnection {
		::WindowConnection::Xcb(self.native.0 .0, self.native.1)
	}
}

impl XcbWindow {
	pub fn failed(&self) -> bool {
		self.native.failed()
	}
}
