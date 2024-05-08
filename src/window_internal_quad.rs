/*
 *  Copyright 2021 QuantumBadger
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use crate::dimen::{IVec2, UVec2, Vec2, Vector2};
use crate::error::{BacktraceError, ErrorMessage};
use crate::window::{
    DrawingWindowHandler,
    EventLoopSendError,
    ModifiersState,
    MouseButton,
    MouseScrollDistance,
    UserEventSender,
    VirtualKeyCode,
    WindowCreationError,
    WindowCreationMode,
    WindowCreationOptions,
    WindowEventLoopAction,
    WindowFullscreenMode,
    WindowHandler,
    WindowHelper,
    WindowPosition,
    WindowSize,
    WindowStartupInfo
};
use crate::GLRenderer;
use crate::Color;

pub(crate) struct WindowHelperQuad<UserEventType: 'static>
{
    renderer: Rc<RefCell<GLRenderer>>,
    event_proxy: Sender<UserEventType>,
    redraw_requested: Cell<bool>,
    terminate_requested: bool,
    physical_size: UVec2,
    is_mouse_grabbed: Cell<bool>,
    tmp: std::marker::PhantomData<UserEventType>
}

impl<UserEventType> WindowHelperQuad<UserEventType>
{
    #[inline]
    pub fn new(
        initial_physical_size: UVec2,
        ep: Sender<UserEventType>,
        renderer: Rc<RefCell<GLRenderer>>,
    ) -> Self
    {
        WindowHelperQuad {
            renderer: renderer,
            event_proxy: ep,
            redraw_requested: Cell::new(false),
            terminate_requested: false,
            physical_size: initial_physical_size,
            is_mouse_grabbed: Cell::new(false),
            tmp: std::marker::PhantomData {},
        }
    }

    #[must_use]
    pub fn create_font_from_bytes(&self, bytes: &[u8]) -> Result<crate::font::Font,i32>
    {
        self.renderer.borrow_mut().create_font_from_bytes(bytes)
    }

    #[inline]
    #[must_use]
    pub fn is_redraw_requested(&self) -> bool
    {
        self.redraw_requested.get()
    }

    #[inline]
    pub fn set_redraw_requested(&mut self, redraw_requested: bool)
    {
        self.redraw_requested.set(redraw_requested);
    }

    #[inline]
    pub fn get_event_loop_action(&self) -> WindowEventLoopAction
    {
        match self.terminate_requested {
            true => WindowEventLoopAction::Exit,
            false => WindowEventLoopAction::Continue
        }
    }

    pub fn terminate_loop(&mut self)
    {
        self.terminate_requested = true;
    }

    pub fn set_icon_from_rgba_pixels(
        &self,
        data: Vec<u8>,
        size: UVec2
    ) -> Result<(), BacktraceError<ErrorMessage>>
    {

        Ok(())
    }

    pub fn set_cursor_visible(&self, visible: bool)
    {
    }

    pub fn set_cursor_grab(
        &self,
        grabbed: bool
    ) -> Result<(), BacktraceError<ErrorMessage>>
    {
        panic!();
    }

    pub fn set_resizable(&self, resizable: bool)
    {
    }

    #[inline]
    pub fn request_redraw(&self)
    {
        self.redraw_requested.set(true);
    }

    pub fn set_title(&self, title: &str)
    {
    }

    pub fn set_fullscreen_mode(&self, mode: WindowFullscreenMode)
    {
    }

    pub fn set_size_pixels<S: Into<UVec2>>(&self, size: S)
    {
    }

    pub fn get_size_pixels(&self) -> UVec2
    {
        let (w, h) = miniquad::window::screen_size();
        let dpi = miniquad::window::dpi_scale();
        return UVec2::new((w / dpi) as u32, (h / dpi) as u32);
    }

    pub fn set_size_scaled_pixels<S: Into<Vec2>>(&self, size: S)
    {
    }

    pub fn set_position_pixels<P: Into<IVec2>>(&self, position: P)
    {
    }

    pub fn set_position_scaled_pixels<P: Into<Vec2>>(&self, position: P)
    {
    }

    #[inline]
    #[must_use]
    pub fn get_scale_factor(&self) -> f64
    {
        miniquad::window::dpi_scale().into()
    }

    pub fn create_user_event_sender(&self) -> UserEventSender<UserEventType>
    {
        UserEventSender::new(UserEventSenderQuad::new(self.event_proxy.clone()))
    }
}

pub(crate) struct WindowQuad<UserEventType: 'static>
{
    title: String,
    options: WindowCreationOptions,
    tmp: std::marker::PhantomData<UserEventType>,
}

impl<UserEventType: 'static> WindowQuad<UserEventType>
{
    pub fn new(
        title: &str,
        options: WindowCreationOptions
    ) -> Result<WindowQuad<UserEventType>, BacktraceError<WindowCreationError>>
    {
        return Ok(WindowQuad
        {
            title: title.to_string(),
            options: options,
            tmp: std::marker::PhantomData {},
        });
    }

    pub fn create_user_event_sender(&self) -> UserEventSender<UserEventType>
    {
        todo!();
    }

    pub fn get_inner_size_pixels(&self) -> UVec2
    {
        let (w, h) = miniquad::window::screen_size();
        return UVec2::new(w as u32, h as u32);
    }

    pub fn run_loop<Handler>(self, handler: Handler) -> !
    where
        Handler: WindowHandler<UserEventType> + 'static
    {
        // TODO get initial width and height
        let config = 
            miniquad::conf::Conf {
                window_width: 1200,
                window_height: 1200,
                window_title: self.title.to_string(),
                high_dpi: true,
                ..Default::default()
            };

        miniquad::start(miniquad::conf::Conf { ..config }, move || {
            let (tx, rx): (Sender<UserEventType>, Receiver<UserEventType>) = mpsc::channel();
            let (w, h) = miniquad::window::screen_size();
            let initial_viewport_size_pixels = UVec2::new(w as u32, h as u32);

            let renderer = GLRenderer::new_for_quad();
            let renderer = RefCell::new(renderer);
            let renderer = Rc::new(renderer);

            let mut helper = WindowHelper::new(WindowHelperQuad::new(
                initial_viewport_size_pixels,
                tx.clone(),
                renderer.clone(),
            ));

            let mut handler = DrawingWindowHandler::new(handler, renderer);
            handler.on_start(
                &mut helper,
                WindowStartupInfo::new(
                    initial_viewport_size_pixels,
                    miniquad::window::dpi_scale().into(),
                )
            );

            Box::new(Stage::new(handler, helper, rx))
        });

        panic!(); // TODO should not get here
    }

}

impl From<miniquad::MouseButton> for MouseButton
{
    fn from(button: miniquad::MouseButton) -> Self
    {
        match button {
            miniquad::MouseButton::Left => MouseButton::Left,
            miniquad::MouseButton::Right => MouseButton::Right,
            miniquad::MouseButton::Middle => MouseButton::Middle,
            miniquad::MouseButton::Unknown => MouseButton::Other(0) // TODO
        }
    }
}

impl From<miniquad::KeyCode> for VirtualKeyCode
{
    fn from(virtual_key_code: miniquad::KeyCode) -> Self
    {
        match virtual_key_code {
            miniquad::KeyCode::Key1 => VirtualKeyCode::Key1,
            miniquad::KeyCode::Key2 => VirtualKeyCode::Key2,
            miniquad::KeyCode::Key3 => VirtualKeyCode::Key3,
            miniquad::KeyCode::Key4 => VirtualKeyCode::Key4,
            miniquad::KeyCode::Key5 => VirtualKeyCode::Key5,
            miniquad::KeyCode::Key6 => VirtualKeyCode::Key6,
            miniquad::KeyCode::Key7 => VirtualKeyCode::Key7,
            miniquad::KeyCode::Key8 => VirtualKeyCode::Key8,
            miniquad::KeyCode::Key9 => VirtualKeyCode::Key9,
            miniquad::KeyCode::Key0 => VirtualKeyCode::Key0,
            miniquad::KeyCode::A => VirtualKeyCode::A,
            miniquad::KeyCode::B => VirtualKeyCode::B,
            miniquad::KeyCode::C => VirtualKeyCode::C,
            miniquad::KeyCode::D => VirtualKeyCode::D,
            miniquad::KeyCode::E => VirtualKeyCode::E,
            miniquad::KeyCode::F => VirtualKeyCode::F,
            miniquad::KeyCode::G => VirtualKeyCode::G,
            miniquad::KeyCode::H => VirtualKeyCode::H,
            miniquad::KeyCode::I => VirtualKeyCode::I,
            miniquad::KeyCode::J => VirtualKeyCode::J,
            miniquad::KeyCode::K => VirtualKeyCode::K,
            miniquad::KeyCode::L => VirtualKeyCode::L,
            miniquad::KeyCode::M => VirtualKeyCode::M,
            miniquad::KeyCode::N => VirtualKeyCode::N,
            miniquad::KeyCode::O => VirtualKeyCode::O,
            miniquad::KeyCode::P => VirtualKeyCode::P,
            miniquad::KeyCode::Q => VirtualKeyCode::Q,
            miniquad::KeyCode::R => VirtualKeyCode::R,
            miniquad::KeyCode::S => VirtualKeyCode::S,
            miniquad::KeyCode::T => VirtualKeyCode::T,
            miniquad::KeyCode::U => VirtualKeyCode::U,
            miniquad::KeyCode::V => VirtualKeyCode::V,
            miniquad::KeyCode::W => VirtualKeyCode::W,
            miniquad::KeyCode::X => VirtualKeyCode::X,
            miniquad::KeyCode::Y => VirtualKeyCode::Y,
            miniquad::KeyCode::Z => VirtualKeyCode::Z,
            miniquad::KeyCode::Escape => VirtualKeyCode::Escape,
            miniquad::KeyCode::F1 => VirtualKeyCode::F1,
            miniquad::KeyCode::F2 => VirtualKeyCode::F2,
            miniquad::KeyCode::F3 => VirtualKeyCode::F3,
            miniquad::KeyCode::F4 => VirtualKeyCode::F4,
            miniquad::KeyCode::F5 => VirtualKeyCode::F5,
            miniquad::KeyCode::F6 => VirtualKeyCode::F6,
            miniquad::KeyCode::F7 => VirtualKeyCode::F7,
            miniquad::KeyCode::F8 => VirtualKeyCode::F8,
            miniquad::KeyCode::F9 => VirtualKeyCode::F9,
            miniquad::KeyCode::F10 => VirtualKeyCode::F10,
            miniquad::KeyCode::F11 => VirtualKeyCode::F11,
            miniquad::KeyCode::F12 => VirtualKeyCode::F12,
            miniquad::KeyCode::F13 => VirtualKeyCode::F13,
            miniquad::KeyCode::F14 => VirtualKeyCode::F14,
            miniquad::KeyCode::F15 => VirtualKeyCode::F15,
            miniquad::KeyCode::F16 => VirtualKeyCode::F16,
            miniquad::KeyCode::F17 => VirtualKeyCode::F17,
            miniquad::KeyCode::F18 => VirtualKeyCode::F18,
            miniquad::KeyCode::F19 => VirtualKeyCode::F19,
            miniquad::KeyCode::F20 => VirtualKeyCode::F20,
            miniquad::KeyCode::F21 => VirtualKeyCode::F21,
            miniquad::KeyCode::F22 => VirtualKeyCode::F22,
            miniquad::KeyCode::F23 => VirtualKeyCode::F23,
            miniquad::KeyCode::F24 => VirtualKeyCode::F24,
            //miniquad::KeyCode::Snapshot => VirtualKeyCode::PrintScreen,
            //miniquad::KeyCode::Scroll => VirtualKeyCode::ScrollLock,
            miniquad::KeyCode::Pause => VirtualKeyCode::PauseBreak,
            miniquad::KeyCode::Insert => VirtualKeyCode::Insert,
            miniquad::KeyCode::Home => VirtualKeyCode::Home,
            miniquad::KeyCode::Delete => VirtualKeyCode::Delete,
            miniquad::KeyCode::End => VirtualKeyCode::End,
            miniquad::KeyCode::PageDown => VirtualKeyCode::PageDown,
            miniquad::KeyCode::PageUp => VirtualKeyCode::PageUp,
            miniquad::KeyCode::Left => VirtualKeyCode::Left,
            miniquad::KeyCode::Up => VirtualKeyCode::Up,
            miniquad::KeyCode::Right => VirtualKeyCode::Right,
            miniquad::KeyCode::Down => VirtualKeyCode::Down,
            miniquad::KeyCode::Backspace => VirtualKeyCode::Backspace,
            miniquad::KeyCode::Enter => VirtualKeyCode::Return,
            miniquad::KeyCode::Space => VirtualKeyCode::Space,
            //miniquad::KeyCode::Compose => VirtualKeyCode::Compose,
            //miniquad::KeyCode::Caret => VirtualKeyCode::Caret,
            //miniquad::KeyCode::Numlock => VirtualKeyCode::Numlock,
            miniquad::KeyCode::Kp0 => VirtualKeyCode::Numpad0,
            miniquad::KeyCode::Kp1 => VirtualKeyCode::Numpad1,
            miniquad::KeyCode::Kp2 => VirtualKeyCode::Numpad2,
            miniquad::KeyCode::Kp3 => VirtualKeyCode::Numpad3,
            miniquad::KeyCode::Kp4 => VirtualKeyCode::Numpad4,
            miniquad::KeyCode::Kp5 => VirtualKeyCode::Numpad5,
            miniquad::KeyCode::Kp6 => VirtualKeyCode::Numpad6,
            miniquad::KeyCode::Kp7 => VirtualKeyCode::Numpad7,
            miniquad::KeyCode::Kp8 => VirtualKeyCode::Numpad8,
            miniquad::KeyCode::Kp9 => VirtualKeyCode::Numpad9,
            miniquad::KeyCode::KpAdd => VirtualKeyCode::NumpadAdd,
            miniquad::KeyCode::KpDivide => VirtualKeyCode::NumpadDivide,
            miniquad::KeyCode::KpDecimal => VirtualKeyCode::NumpadDecimal,
            //miniquad::KeyCode::KpComma => VirtualKeyCode::NumpadComma,
            miniquad::KeyCode::KpEnter => VirtualKeyCode::NumpadEnter,
            miniquad::KeyCode::KpEqual => VirtualKeyCode::NumpadEquals,
            miniquad::KeyCode::KpMultiply => VirtualKeyCode::NumpadMultiply,
            miniquad::KeyCode::KpSubtract => VirtualKeyCode::NumpadSubtract,
            //miniquad::KeyCode::AbntC1 => VirtualKeyCode::AbntC1,
            //miniquad::KeyCode::AbntC2 => VirtualKeyCode::AbntC2,
            miniquad::KeyCode::Apostrophe => VirtualKeyCode::Apostrophe,
            //miniquad::KeyCode::Apps => VirtualKeyCode::Apps,
            //miniquad::KeyCode::Asterisk => VirtualKeyCode::Asterisk,
            //miniquad::KeyCode::At => VirtualKeyCode::At,
            //miniquad::KeyCode::Ax => VirtualKeyCode::Ax,
            miniquad::KeyCode::Backslash => VirtualKeyCode::Backslash,
            //miniquad::KeyCode::Calculator => VirtualKeyCode::Calculator,
            //miniquad::KeyCode::Capital => VirtualKeyCode::Capital,
            //miniquad::KeyCode::Colon => VirtualKeyCode::Colon,
            miniquad::KeyCode::Comma => VirtualKeyCode::Comma,
            //miniquad::KeyCode::Convert => VirtualKeyCode::Convert,
            miniquad::KeyCode::Equal => VirtualKeyCode::Equals,
            miniquad::KeyCode::GraveAccent => VirtualKeyCode::Grave,
            //miniquad::KeyCode::Kana => VirtualKeyCode::Kana,
            //miniquad::KeyCode::Kanji => VirtualKeyCode::Kanji,
            miniquad::KeyCode::LeftAlt => VirtualKeyCode::LAlt,
            miniquad::KeyCode::LeftBracket => VirtualKeyCode::LBracket,
            miniquad::KeyCode::LeftControl => VirtualKeyCode::LControl,
            miniquad::KeyCode::LeftShift => VirtualKeyCode::LShift,
            //miniquad::KeyCode::LWin => VirtualKeyCode::LWin,
            //miniquad::KeyCode::Mail => VirtualKeyCode::Mail,
            //miniquad::KeyCode::MediaSelect => VirtualKeyCode::MediaSelect,
            //miniquad::KeyCode::MediaStop => VirtualKeyCode::MediaStop,
            miniquad::KeyCode::Minus => VirtualKeyCode::Minus,
            //miniquad::KeyCode::Mute => VirtualKeyCode::Mute,
            //miniquad::KeyCode::MyComputer => VirtualKeyCode::MyComputer,
            //miniquad::KeyCode::NavigateForward => VirtualKeyCode::NavigateForward,
            //miniquad::KeyCode::NavigateBackward => VirtualKeyCode::NavigateBackward,
            //miniquad::KeyCode::NextTrack => VirtualKeyCode::NextTrack,
            //miniquad::KeyCode::NoConvert => VirtualKeyCode::NoConvert,
            //miniquad::KeyCode::OEM102 => VirtualKeyCode::OEM102,
            miniquad::KeyCode::Period => VirtualKeyCode::Period,
            //miniquad::KeyCode::PlayPause => VirtualKeyCode::PlayPause,
            //miniquad::KeyCode::Plus => VirtualKeyCode::Plus,
            //miniquad::KeyCode::Power => VirtualKeyCode::Power,
            //miniquad::KeyCode::PrevTrack => VirtualKeyCode::PrevTrack,
            miniquad::KeyCode::RightAlt => VirtualKeyCode::RAlt,
            miniquad::KeyCode::RightBracket => VirtualKeyCode::RBracket,
            miniquad::KeyCode::RightControl => VirtualKeyCode::RControl,
            miniquad::KeyCode::RightShift => VirtualKeyCode::RShift,
            //miniquad::KeyCode::RWin => VirtualKeyCode::RWin,
            miniquad::KeyCode::Semicolon => VirtualKeyCode::Semicolon,
            miniquad::KeyCode::Slash => VirtualKeyCode::Slash,
            //miniquad::KeyCode::Sleep => VirtualKeyCode::Sleep,
            //miniquad::KeyCode::Stop => VirtualKeyCode::Stop,
            //miniquad::KeyCode::Sysrq => VirtualKeyCode::Sysrq,
            miniquad::KeyCode::Tab => VirtualKeyCode::Tab,
            //miniquad::KeyCode::Underline => VirtualKeyCode::Underline,
            //miniquad::KeyCode::Unlabeled => VirtualKeyCode::Unlabeled,
            //miniquad::KeyCode::VolumeDown => VirtualKeyCode::VolumeDown,
            //miniquad::KeyCode::VolumeUp => VirtualKeyCode::VolumeUp,
            //miniquad::KeyCode::Wake => VirtualKeyCode::Wake,
            //miniquad::KeyCode::WebBack => VirtualKeyCode::WebBack,
            //miniquad::KeyCode::WebFavorites => VirtualKeyCode::WebFavorites,
            //miniquad::KeyCode::WebForward => VirtualKeyCode::WebForward,
            //miniquad::KeyCode::WebHome => VirtualKeyCode::WebHome,
            //miniquad::KeyCode::WebRefresh => VirtualKeyCode::WebRefresh,
            //miniquad::KeyCode::WebSearch => VirtualKeyCode::WebSearch,
            //miniquad::KeyCode::WebStop => VirtualKeyCode::WebStop,
            //miniquad::KeyCode::Yen => VirtualKeyCode::Yen,
            //miniquad::KeyCode::Copy => VirtualKeyCode::Copy,
            //miniquad::KeyCode::Paste => VirtualKeyCode::Paste,
            //miniquad::KeyCode::Cut => VirtualKeyCode::Cut,
            miniquad::KeyCode::CapsLock => VirtualKeyCode::Wake,
            miniquad::KeyCode::World1 => VirtualKeyCode::Wake,
            miniquad::KeyCode::World2 => VirtualKeyCode::Wake,
            miniquad::KeyCode::ScrollLock => VirtualKeyCode::Wake,
            miniquad::KeyCode::NumLock => VirtualKeyCode::Wake,
            miniquad::KeyCode::PrintScreen => VirtualKeyCode::Wake,
            miniquad::KeyCode::F25 => VirtualKeyCode::Wake,
            miniquad::KeyCode::LeftSuper => VirtualKeyCode::Wake,
            miniquad::KeyCode::RightSuper => VirtualKeyCode::Wake,
            miniquad::KeyCode::Menu => VirtualKeyCode::Wake,
            miniquad::KeyCode::Unknown => VirtualKeyCode::Wake,
        }
    }
}

impl From<miniquad::KeyMods> for ModifiersState
{
    fn from(state: miniquad::KeyMods) -> Self
    {
        ModifiersState {
            ctrl: state.ctrl,
            alt: state.alt,
            shift: state.shift,
            logo: state.logo
        }
    }
}

/*
impl From<PhysicalSize<u32>> for UVec2
{
    fn from(value: PhysicalSize<u32>) -> Self
    {
        Self::new(value.width, value.height)
    }
}
*/

pub(crate) enum UserEventQuad<UserEventType: 'static>
{
    MouseGrabStatusChanged(bool),
    FullscreenStatusChanged(bool),
    UserEvent(UserEventType)
}

pub struct UserEventSenderQuad<UserEventType: 'static>
{
    event_proxy: Sender<UserEventType>,
}

impl<UserEventType> Clone for UserEventSenderQuad<UserEventType>
{
    fn clone(&self) -> Self
    {
        UserEventSenderQuad {
            event_proxy: self.event_proxy.clone()
        }
    }
}

impl<UserEventType> UserEventSenderQuad<UserEventType>
{
    fn new(ep: Sender<UserEventType>) -> Self
    {
        Self { event_proxy: ep }
    }

    pub fn send_event(&self, event: UserEventType) -> Result<(), EventLoopSendError>
    {
        self.event_proxy.send(event);
        Ok(())
    }
}

struct Stage<UserEventType, HandlerType>
    where UserEventType: 'static,
        HandlerType: WindowHandler<UserEventType> + 'static
{
    handler: DrawingWindowHandler<UserEventType, HandlerType>,
    helper: WindowHelper<UserEventType>,
    user_events: Receiver<UserEventType>,
}

impl<UserEventType, HandlerType: WindowHandler<UserEventType>> Stage<UserEventType, HandlerType>
{
    const DEFAULT_BG_COLOR: Color = Color::BLACK;

    fn new(
        handler: DrawingWindowHandler<UserEventType, HandlerType>,
        helper: WindowHelper<UserEventType>,
        user_events: Receiver<UserEventType>,
        ) -> Self 
    {
        Stage 
        {
            handler: handler,
            helper: helper,
            user_events: user_events,
        }
    }

}

impl<UserEventType, HandlerType: WindowHandler<UserEventType>> miniquad::EventHandler for Stage<UserEventType, HandlerType> {
    fn resize_event(&mut self, width: f32, height: f32) {
        let dpi = miniquad::window::dpi_scale();
        self.handler.on_resize(&mut self.helper, UVec2::new((width / dpi) as u32, (height / dpi) as u32));
    }

    fn raw_mouse_motion(&mut self, x: f32, y: f32) {
        //self.handler.on_mouse_move(&mut self.helper, Vec2::new(x, y));
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let dpi = miniquad::window::dpi_scale();
        self.handler.on_mouse_move(&mut self.helper, Vec2::new(x / dpi, y / dpi));
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
    }

    fn mouse_button_down_event(&mut self, btn: miniquad::MouseButton, x: f32, y: f32) {
        self.handler.on_mouse_button_down(&mut self.helper, btn.into());
    }

    fn mouse_button_up_event(&mut self, btn: miniquad::MouseButton, x: f32, y: f32) {
        self.handler.on_mouse_button_up(&mut self.helper, btn.into());
    }

    fn touch_event(&mut self, phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) {
    }

    fn char_event(&mut self, character: char, modifiers: miniquad::KeyMods, repeat: bool) {
        self.handler.on_keyboard_char(&mut self.helper, character);
    }

    fn key_down_event(&mut self, keycode: miniquad::KeyCode, modifiers: miniquad::KeyMods, repeat: bool) {
        self.handler.on_key_down(&mut self.helper, Some(keycode.into()), 0); // TODO
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, modifiers: miniquad::KeyMods) {
    }

    fn update(&mut self) {
        match self.user_events.try_recv()
        {
            Ok(x) => 
            {
                self.handler.on_user_event(&mut self.helper, x);
            },
            Err(_) =>
            {
            },
        }
    }

    fn draw(&mut self) {
        self.handler.on_draw(&mut self.helper);
    }

    fn window_restored_event(&mut self) {
    }

    fn window_minimized_event(&mut self) {
    }

    fn quit_requested_event(&mut self) {
    }
}

