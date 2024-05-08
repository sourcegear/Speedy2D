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

//! Hardware-accelerated drawing of shapes, images, and text, with an easy to
//! use API.
//!
//! Speedy2D aims to be:
//!
//!  - The simplest Rust API for creating a window, rendering graphics/text, and
//!    handling input
//!  - Compatible with any device supporting OpenGL 2.0+ or WebGL 2.0. Support
//!    for OpenGL ES 2.0+ is planned.
//!  - Very fast
//!
//! Supports Windows, Mac, Linux, and WebGL. Support for Android and iOS is in
//! development.
//!
//! By default, Speedy2D contains support for setting up a window with an OpenGL
//! context. If you'd like to handle this yourself, and use Speedy2D only for
//! rendering, you can disable the `windowing` feature.
//!
//! # Useful Links
//!
//! * [Source repository](https://github.com/QuantumBadger/Speedy2D)
//! * [Crate](https://crates.io/crates/speedy2d)
//!
//! # Getting Started (Windows/Mac/Linux)
//!
//! ## Create a window
//!
//! After adding Speedy2D to your Cargo.toml dependencies, create a window as
//! follows:
//!
//! ```rust,no_run
//! use speedy2d::Window;
//!
//! let window = Window::new_centered("Title", (640, 480)).unwrap();
//! ```
//!
//! You may also use [Window::new_fullscreen_borderless()],
//! [Window::new_with_options()], or [Window::new_with_user_events()].
//!
//! ## Implement the callbacks
//!
//! Create a struct implementing the `WindowHandler` trait. Override
//! whichever callbacks you're interested in, for example `on_draw()`,
//! `on_mouse_move()`, or `on_key_down()`.
//!
//! ```
//! use speedy2d::window::{WindowHandler, WindowHelper};
//! use speedy2d::Graphics2D;
//!
//! struct MyWindowHandler {}
//!
//! impl WindowHandler for MyWindowHandler
//! {
//!     fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
//!     {
//!         // Draw things here using `graphics`
//!     }
//! }
//! ```
//!
//! The full list of possible callbacks is currently as follows. See
//! [WindowHandler] for full documentation.
//!
//! It's only necessary to implement the callbacks you actually want to use. The
//! default implementation will do nothing and continue the event loop.
//!
//! ```text
//! fn on_start()
//! fn on_user_event()
//! fn on_resize()
//! fn on_scale_factor_changed()
//! fn on_draw()
//! fn on_mouse_move()
//! fn on_mouse_button_down()
//! fn on_mouse_button_up()
//! fn on_key_down()
//! fn on_key_up()
//! fn on_keyboard_char()
//! fn on_keyboard_modifiers_changed()
//! ```
//!
//! Each callback gives you a [window::WindowHelper] instance, which
//! lets you perform window-related actions, like requesting that a new frame is
//! drawn using [window::WindowHelper::request_redraw()].
//!
//! Note: Unless you call [window::WindowHelper::request_redraw()], frames will
//! only be drawn when necessary, for example when resizing the window.
//!
//! ## Render some graphics
//!
//! The [WindowHandler::on_draw()] callback gives you a [Graphics2D]
//! instance, which lets you draw shapes, text, and images.
//!
//! ```
//! # use speedy2d::window::{WindowHandler, WindowHelper};
//! # use speedy2d::Graphics2D;
//! # use speedy2d::color::Color;
//! #
//! # struct MyWindowHandler {}
//! #
//! # impl WindowHandler for MyWindowHandler
//! # {
//!     fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
//!     {
//!         graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
//!         graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
//!
//!         // Request that we draw another frame once this one has finished
//!         helper.request_redraw();
//!     }
//! # }
//! ```
//!
//! ## Start it running!
//!
//! Once you've implemented the callbacks you're interested in, start the event
//! loop running with [Window::run_loop()]:
//!
//! ```rust,no_run
//! # use speedy2d::Window;
//! # struct MyWindowHandler {}
//! # impl speedy2d::window::WindowHandler for MyWindowHandler {}
//! let window = Window::<()>::new_centered("Title", (640, 480)).unwrap();
//!
//! window.run_loop(MyWindowHandler{});
//! ```
//!
//! ## Alternative: Managing the GL context yourself
//!
//! If you'd rather handle the window creation and OpenGL context management
//! yourself, simply disable Speedy2D's `windowing` feature in your `Cargo.toml`
//! file, and create a context as follows. You will need to specify a loader
//! function to allow Speedy2D to obtain the OpenGL function pointers.
//!
//! ```rust,no_run
//! use speedy2d::GLRenderer;
//! # struct WindowContext {}
//! # impl WindowContext {
//! #     fn get_proc_address(&self, fn_name: &str) -> *const std::ffi::c_void
//! #     {
//! #         std::ptr::null()
//! #     }
//! # }
//! # let window_context = WindowContext {};
//!
//! let mut renderer = unsafe {
//!     GLRenderer::new_for_gl_context((640, 480), |fn_name| {
//!         window_context.get_proc_address(fn_name) as *const _
//!     })
//! }.unwrap();
//! ```
//!
//! Then, draw a frame using [GLRenderer::draw_frame()]:
//!
//! ```rust,no_run
//! # use speedy2d::GLRenderer;
//! # use speedy2d::color::Color;
//! # let mut renderer = unsafe {
//! #     GLRenderer::new_for_gl_context((640, 480), |fn_name| {
//! #         std::ptr::null() as *const _
//! #     })
//! # }.unwrap();
//! renderer.draw_frame(|graphics| {
//!     graphics.clear_screen(Color::WHITE);
//!     graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
//! });
//! ```
//!
//! # Laying out text
//!
//! To render text, a font must be created. Call [font::Font::new()] with the
//! bytes from the TTF or OTF font file.
//!
//! (note: OTF support may be limited)
//!
//! ```rust,no_run
//! use speedy2d::font::Font;
//!
//! let bytes = include_bytes!("../assets/fonts/NotoSans-Regular.ttf");
//! let font = Font::new(bytes).unwrap();
//! ```
//!
//! Then, invoke `font.layout_text()` (part of the [font::TextLayout] trait) to
//! calculate the necessary line breaks and spacing. This will give you
//! a [font::FormattedTextBlock].
//!
//! ```rust,no_run
//! # use speedy2d::font::{Font, TextOptions};
//! # let font = Font::new(&[]).unwrap();
//! use speedy2d::font::TextLayout;
//!
//! let block = font.layout_text("Hello World", 32.0, TextOptions::new());
//! ```
//!
//! Finally, call [Graphics2D::draw_text()] to draw the text block!
//!
//! ```rust,no_run
//! # use speedy2d::GLRenderer;
//! # use speedy2d::color::Color;
//! # use speedy2d::font::{Font, TextOptions, TextLayout};
//! # let font = Font::new(&[]).unwrap();
//! # let block = font.layout_text("Hello World", 32.0, TextOptions::new());
//! # let mut renderer = unsafe {
//! #     GLRenderer::new_for_gl_context((640, 480), |fn_name| {
//! #         std::ptr::null() as *const _
//! #     })
//! # }.unwrap();
//! # renderer.draw_frame(|graphics| {
//! graphics.draw_text((100.0, 100.0), Color::BLUE, &block);
//! # });
//! ```
//!
//! ## Word wrap
//!
//! To wrap lines of text to a certain width, use
//! [font::TextOptions::with_wrap_to_width()]:
//!
//! ```rust,no_run
//! # use speedy2d::font::{Font, TextOptions};
//! # let font = Font::new(&[]).unwrap();
//! use speedy2d::font::{TextLayout, TextAlignment};
//!
//! let block = font.layout_text(
//!     "The quick brown fox jumps over the lazy dog.",
//!     32.0,
//!     TextOptions::new().with_wrap_to_width(300.0, TextAlignment::Left));
//! ```
//!
//! # Loading images
//!
//! Image files (in formats such as PNG, JPG, and BMP) can be loaded using the
//! following APIs, available in both `Graphics2D` and `GLRenderer`.
//!
//! * [Graphics2D::create_image_from_file_path()]
//! * [Graphics2D::create_image_from_file_bytes()]
//! * [GLRenderer::create_image_from_file_path()]
//! * [GLRenderer::create_image_from_file_bytes()]
//!
//! Alternatively, you can create an image from raw pixel data, using:
//!
//! * [Graphics2D::create_image_from_raw_pixels()]
//! * [GLRenderer::create_image_from_raw_pixels()]
//!
//! # Getting Started (WebGL)
//!
//! To use Speedy2D with WebGL, your app must be compiled for WebAssembly.
//! Speedy2D can attach itself to a `canvas` on the page using an ID you
//! specify.
//!
//! As with Windows/Mac/Linux targets, it's possible to use Speedy2D either in a
//! full rendering and event handling configuation, or for rendering only.
//!
//! For rendering only, use the following API:
//!
//! * [GLRenderer::new_for_web_canvas_by_id()]
//!
//! For full keyboard/mouse/etc event handling in addition to rendering, use:
//!
//! * [WebCanvas::new_for_id()]
//! * [WebCanvas::new_for_id_with_user_events()]
//!
//! After initialization, the usual [WindowHandler] callbacks and
//! [window::WindowHelper]/[Graphics2D] APIs should operate as on other
//! platforms.
//!
//! For an example, see the `examples/webgl` directory. To build this, first
//! install the prerequisites:
//!
//! ```shell
//! cargo install wasm-bindgen-cli just
//! ```
//!
//! Then use the following command to run the build:
//!
//! ```shell
//! just build-example-webgl
//! ```

//#![deny(warnings)]
//#![deny(missing_docs)]

use std::fmt::{Display, Formatter};
#[cfg(any(doc, doctest, all(target_arch = "wasm32", feature = "windowing")))]
use std::marker::PhantomData;
use std::rc::Rc;

#[cfg(any(feature = "image-loading", doc, doctest))]
use {
    crate::image::ImageFileFormat,
    std::io::{BufRead, Seek},
    std::path::Path
};

use crate::color::Color;
use crate::dimen::{UVec2, Vec2};
use crate::error::{BacktraceError, ErrorMessage};
use crate::font::FormattedTextBlock;
use crate::image::{ImageDataType, ImageHandle, ImageSmoothingMode, RawBitmapData};
use crate::shape::{Polygon, Rect, Rectangle, RoundedRectangle};
#[cfg(any(doc, doctest, feature = "windowing"))]
use crate::window::WindowHandler;
//#[cfg(any(doc, doctest, all(feature = "windowing", not(target_arch = "wasm32"))))]
use crate::window::{
    UserEventSender,
    WindowCreationError,
    WindowCreationOptions,
    WindowPosition,
    WindowSize
};
#[cfg(any(doc, doctest))]
use crate::window_internal_doctest::{WebCanvasImpl, WindowGlutin};
use crate::window_internal_quad::WindowQuad;

pub mod math;
mod text;
mod shapes;
mod quad_gl;
mod texture;

/// Types representing colors.
pub mod color;

/// Types representing shapes.
pub mod shape;

/// Components for loading fonts and laying out text.
pub mod font;

/// Types representing sizes and positions.
pub mod dimen;

/// Utilities and traits for numeric values.
pub mod numeric;

/// Error types.
pub mod error;

/// Types relating to images.
pub mod image;

/// Utilities for accessing the system clock on all platforms.
pub mod time;

/// Allows for the creation and management of windows.
#[cfg(any(doc, doctest, feature = "windowing"))]
pub mod window;

mod window_internal_quad;

#[cfg(any(doc, doctest))]
mod window_internal_doctest;

//mod font_cache;
//mod texture_packer;
mod utils;

use quad_gl::QuadGl;

/// An error encountered during the creation of a [GLRenderer].
#[derive(Clone, Debug)]
pub struct GLRendererCreationError
{
    description: String
}

impl GLRendererCreationError
{
    fn msg_with_cause<S, Cause>(description: S, cause: Cause) -> BacktraceError<Self>
    where
        S: AsRef<str>,
        Cause: std::error::Error + 'static
    {
        BacktraceError::new_with_cause(
            Self {
                description: description.as_ref().to_string()
            },
            cause
        )
    }

    #[allow(dead_code)]
    fn msg<S>(description: S) -> BacktraceError<Self>
    where
        S: AsRef<str>
    {
        BacktraceError::new(Self {
            description: description.as_ref().to_string()
        })
    }
}

impl Display for GLRendererCreationError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        Display::fmt("GL renderer creation error: ", f)?;
        Display::fmt(&self.description, f)
    }
}

/// A graphics renderer using an OpenGL backend.
///
/// Note: There is no need to use this struct if you are letting Speedy2D create
/// a window for you.
pub struct GLRenderer
{
    renderer: Graphics2D
}

impl GLRenderer
{
    /// Creates a `GLRenderer` with the specified OpenGL loader function. The
    /// loader function takes the name of an OpenGL function, and returns the
    /// associated function pointer. `viewport_size_pixels` should be set to
    /// the initial viewport size, however this can be changed later using
    /// [GLRenderer:: set_viewport_size_pixels()].
    ///
    /// Note: This function must not be called if you are letting Speedy2D
    /// create a window for you.
    ///
    /// # Safety
    ///
    /// While a `GLRenderer` object is active, you must not make any changes to
    /// the active GL context. Doing so may lead to undefined behavior,
    /// which is why this function is marked `unsafe`. It is strongly
    /// advised not to use any other OpenGL libraries in the same thread
    /// as `GLRenderer`.
    pub fn new_for_quad(
        ) -> Self
    {
        let mut ctx: Box<dyn miniquad::RenderingBackend> =
            miniquad::window::new_rendering_backend();

        let gl = QuadGl::new(&mut *ctx);
        let texture_batcher = crate::texture::Batcher::new(&mut *ctx);
        let renderer = Graphics2D {
            renderer: ctx,
            gl:  gl,
            textures: crate::texture::TexturesContext::new(),
            texture_batcher: texture_batcher,
        };

        GLRenderer { renderer }
    }

    pub fn create_font_from_bytes(&mut self, bytes: &[u8]) -> Result<crate::font::Font, i32>
    {
        let f = text::load_ttf_font_from_bytes(&mut *self.renderer.renderer, bytes).unwrap();
        Ok(crate::font::Font::new_from_other_kind_of_font(f))
    }

    /// Sets the renderer viewport to the specified pixel size, in response to a
    /// change in the window size.
    pub fn set_viewport_size_pixels(&mut self, viewport_size_pixels: UVec2)
    {
        //panic!();
    }

    /// Creates a new [ImageHandle] from the specified raw pixel data.
    ///
    /// The data provided in the `data` parameter must be in the format
    /// specified by `data_type`.
    ///
    /// The returned [ImageHandle] is valid only for the current graphics
    /// context.
    pub fn create_image_from_raw_pixels(
        &mut self,
        data_type: ImageDataType,
        smoothing_mode: ImageSmoothingMode,
        size: UVec2,
        data: &[u8]
    ) -> Result<ImageHandle, BacktraceError<ErrorMessage>>
    {
        self.renderer
            .create_image_from_raw_pixels(data_type, smoothing_mode, size, data)
    }

    /// Loads an image from the specified file path.
    ///
    /// If no `data_type` is provided, an attempt will be made to guess the file
    /// format.
    ///
    /// For a list of supported image types, see [image::ImageFileFormat].
    ///
    /// The returned [ImageHandle] is valid only for the current graphics
    /// context.
    #[cfg(any(feature = "image-loading", doc, doctest))]
    pub fn create_image_from_file_path<S: AsRef<Path>>(
        &mut self,
        data_type: Option<ImageFileFormat>,
        smoothing_mode: ImageSmoothingMode,
        path: S
    ) -> Result<ImageHandle, BacktraceError<ErrorMessage>>
    {
        self.renderer
            .create_image_from_file_path(data_type, smoothing_mode, path)
    }

    /// Loads an image from the provided encoded image file data.
    ///
    /// If no `data_type` is provided, an attempt will be made to guess the file
    /// format.
    ///
    /// The data source must implement `std::io::BufRead` and `std::io::Seek`.
    /// For example, if you have a `&[u8]`, you may wrap it in a
    /// `std::io::Cursor` as follows:
    ///
    /// ```rust,no_run
    /// # use speedy2d::GLRenderer;
    /// # use speedy2d::color::Color;
    /// # use speedy2d::image::ImageSmoothingMode;
    /// use std::io::Cursor;
    /// # let mut renderer = unsafe {
    /// #     GLRenderer::new_for_gl_context((640, 480), |fn_name| {
    /// #         std::ptr::null() as *const _
    /// #     })
    /// # }.unwrap();
    ///
    /// let image_bytes : &[u8] = include_bytes!("../assets/screenshots/hello_world.png");
    ///
    /// let image_result = renderer.create_image_from_file_bytes(
    ///     None,
    ///     ImageSmoothingMode::Linear,
    ///     Cursor::new(image_bytes));
    /// ```
    ///
    /// For a list of supported image types, see [image::ImageFileFormat].
    ///
    /// The returned [ImageHandle] is valid only for the current graphics
    /// context.
    #[cfg(any(feature = "image-loading", doc, doctest))]
    pub fn create_image_from_file_bytes<R: Seek + BufRead>(
        &mut self,
        data_type: Option<ImageFileFormat>,
        smoothing_mode: ImageSmoothingMode,
        file_bytes: R
    ) -> Result<ImageHandle, BacktraceError<ErrorMessage>>
    {
        self.renderer
            .create_image_from_file_bytes(data_type, smoothing_mode, file_bytes)
    }

    /// Starts the process of drawing a frame. A `Graphics2D` object will be
    /// provided to the callback. When the callback returns, the internal
    /// render queue will be flushed.
    ///
    /// Note: if calling this method, you are responsible for swapping the
    /// window context buffers if necessary.
    #[inline]
    pub fn draw_frame<F: FnOnce(&mut Graphics2D) -> R, R>(&mut self, callback: F) -> R
    {
        //self.renderer.set_clip(None);
        self.renderer.begin_frame();
        let result = callback(&mut self.renderer);
        self.renderer.end_frame();
        result
    }
}

impl Drop for GLRenderer
{
    fn drop(&mut self)
    {
        //self.context.mark_invalid();
    }
}

/// A `Graphics2D` object allows you to draw shapes, images, and text to the
/// screen.
///
/// An instance is provided in the [window::WindowHandler::on_draw] callback.
///
/// If you are managing the GL context yourself, you must invoke
/// [GLRenderer::draw_frame] to obtain an instance.
pub struct Graphics2D
{
    renderer: Box<dyn miniquad::RenderingBackend>,
    gl: QuadGl,
    textures: crate::texture::TexturesContext,
    texture_batcher: crate::texture::Batcher,
}

impl Graphics2D
{
    /// Creates a new [ImageHandle] from the specified raw pixel data.
    ///
    /// The data provided in the `data` parameter must be in the format
    /// specified by `data_type`.
    ///
    /// The returned [ImageHandle] is valid only for the current graphics
    /// context.
    pub fn create_image_from_raw_pixels<S: Into<UVec2>>(
        &mut self,
        data_type: ImageDataType,
        smoothing_mode: ImageSmoothingMode,
        size: S,
        data: &[u8]
    ) -> Result<ImageHandle, BacktraceError<ErrorMessage>>
    {
        panic!();
    }

    /// Loads an image from the specified file path.
    ///
    /// If no `data_type` is provided, an attempt will be made to guess the file
    /// format.
    ///
    /// For a list of supported image types, see [image::ImageFileFormat].
    ///
    /// The returned [ImageHandle] is valid only for the current graphics
    /// context.
    #[cfg(any(feature = "image-loading", doc, doctest))]
    pub fn create_image_from_file_path<S: AsRef<Path>>(
        &mut self,
        data_type: Option<ImageFileFormat>,
        smoothing_mode: ImageSmoothingMode,
        path: S
    ) -> Result<ImageHandle, BacktraceError<ErrorMessage>>
    {
        panic!();
    }

    /// Loads an image from the provided encoded image file data.
    ///
    /// If no `data_type` is provided, an attempt will be made to guess the file
    /// format.
    ///
    /// The data source must implement `std::io::BufRead` and `std::io::Seek`.
    /// For example, if you have a `&[u8]`, you may wrap it in a
    /// `std::io::Cursor` as follows:
    ///
    /// ```rust,no_run
    /// # use speedy2d::GLRenderer;
    /// # use speedy2d::color::Color;
    /// # use speedy2d::image::ImageSmoothingMode;
    /// use std::io::Cursor;
    /// # let mut renderer = unsafe {
    /// #     GLRenderer::new_for_gl_context((640, 480), |fn_name| {
    /// #         std::ptr::null() as *const _
    /// #     })
    /// # }.unwrap();
    /// # renderer.draw_frame(|graphics| {
    ///
    /// let image_bytes : &[u8] = include_bytes!("../assets/screenshots/hello_world.png");
    ///
    /// let image_result = graphics.create_image_from_file_bytes(
    ///     None,
    ///     ImageSmoothingMode::Linear,
    ///     Cursor::new(image_bytes));
    /// # });
    /// ```
    ///
    /// For a list of supported image types, see [image::ImageFileFormat].
    ///
    /// The returned [ImageHandle] is valid only for the current graphics
    /// context.
    #[cfg(any(feature = "image-loading", doc, doctest))]
    pub fn create_image_from_file_bytes<R: Seek + BufRead>(
        &mut self,
        data_type: Option<ImageFileFormat>,
        smoothing_mode: ImageSmoothingMode,
        file_bytes: R
    ) -> Result<ImageHandle, BacktraceError<ErrorMessage>>
    {
        panic!();
    }

    /// Fills the screen with the specified color.
    pub fn clear_screen(&mut self, color: Color)
    {
        self.renderer.clear(Some((color.r(), color.g(), color.b(), color.a())), None, None);
    }

    /// Draws the provided block of text at the specified position.
    ///
    /// Lines of text can be prepared by loading a font (using
    /// [crate::font::Font::new]), and calling `layout_text_line()` on that
    /// font with your desired text.
    ///
    /// To fall back to another font if a glyph isn't found, see
    /// [crate::font::FontFamily].
    ///
    /// To achieve good performance, it's possible to layout a line of text
    /// once, and then re-use the same [crate::font::FormattedTextLine]
    /// object whenever you need to draw that text to the screen.
    ///
    /// Note: Text will be rendered with subpixel precision. If the subpixel
    /// position changes between frames, performance may be degraded, as the
    /// text will need to be re-rendered and re-uploaded. To avoid this,
    /// call `round()` on the position coordinates, to ensure that
    /// the text is always located at an integer pixel position.
    pub fn draw_text<V: Into<Vec2>>(
        &mut self,
        position: V,
        color: Color,
        text: &FormattedTextBlock
    )
    {
        let position = position.into();
        let parms = 
            crate::text::TextParams
            {
                color: color,
                font: &text.f,
                font_size: text.scale as u16,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
            };
        crate::text::draw_text_ex(
            &mut self.gl,
            &mut *self.renderer,
            &self.textures,
            &mut self.texture_batcher,
            &text.text,
            position.x,
            position.y + text.dim.offset_y,
            parms
            );
    }

    /// Draws the provided block of text at the specified position, cropped to
    /// the specified window. Characters outside this window will not be
    /// rendered. Characters partially inside the window will be cropped.
    ///
    /// Both `position` and `crop_window` are relative to the overall render
    /// window.
    ///
    /// See the documentation for [Graphics2D::draw_text] for more details.
    pub fn draw_text_cropped<V: Into<Vec2>>(
        &mut self,
        position: V,
        crop_window: Rect,
        color: Color,
        text: &FormattedTextBlock
    )
    {
        // TODO need to actually crop
        let position = position.into();
        let parms = 
            crate::text::TextParams
            {
                color: color,
                font: &text.f,
                font_size: (text.scale * 1.0) as u16,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
            };
        crate::text::draw_text_ex(
            &mut self.gl,
            &mut *self.renderer,
            &self.textures,
            &mut self.texture_batcher,
            &text.text,
            position.x,
            position.y + text.dim.offset_y,
            parms
            );
    }

    /// Draws a polygon with a single color, with the specified offset in
    /// pixels.
    pub fn draw_polygon<V: Into<Vec2>>(
        &mut self,
        polygon: &Polygon,
        offset: V,
        color: Color
    )
    {
        panic!();
    }

    /// Draws a triangle with the specified colors (one color for each corner).
    ///
    /// The vertex positions (and associated colors) must be provided in
    /// clockwise order.
    pub fn draw_triangle_three_color(
        &mut self,
        vertex_positions_clockwise: [Vec2; 3],
        vertex_colors_clockwise: [Color; 3]
    )
    {
        //panic!();
    }

    /// Draws part of an image, tinted with the provided colors, at the
    /// specified location. The sub-image will be scaled to fill the
    /// triangle described by the vertices in `vertex_positions_clockwise`.
    ///
    /// The coordinates in `image_coords_normalized` should be in the range
    /// `0.0` to `1.0`, and define the portion of the source image which
    /// should be drawn.
    ///
    /// The tinting is performed by for each pixel by multiplying each color
    /// component in the image pixel by the corresponding color component in
    /// the `color` parameter.
    ///
    /// The vertex positions (and associated colors and image coordinates) must
    /// be provided in clockwise order.
    pub fn draw_triangle_image_tinted_three_color(
        &mut self,
        vertex_positions_clockwise: [Vec2; 3],
        vertex_colors: [Color; 3],
        image_coords_normalized: [Vec2; 3],
        image: &ImageHandle
    )
    {
        panic!();
    }

    /// Draws a triangle with the specified color.
    ///
    /// The vertex positions must be provided in clockwise order.
    #[inline]
    pub fn draw_triangle(&mut self, vertex_positions_clockwise: [Vec2; 3], color: Color)
    {
        //self.draw_triangle_three_color(vertex_positions_clockwise, [color, color, color]);
        shapes::draw_triangle(
            &mut self.gl, 
            glam::Vec2::new(vertex_positions_clockwise[0].x, vertex_positions_clockwise[0].y),
            glam::Vec2::new(vertex_positions_clockwise[1].x, vertex_positions_clockwise[1].y),
            glam::Vec2::new(vertex_positions_clockwise[2].x, vertex_positions_clockwise[2].y),
            color);
    }

    /// Draws a quadrilateral with the specified colors (one color for each
    /// corner).
    ///
    /// The vertex positions (and associated colors) must be provided in
    /// clockwise order.
    #[inline]
    pub fn draw_quad_four_color(
        &mut self,
        vertex_positions_clockwise: [Vec2; 4],
        vertex_colors: [Color; 4]
    )
    {
        let vp = vertex_positions_clockwise;
        let vc = vertex_colors;

        self.draw_triangle_three_color([vp[0], vp[1], vp[2]], [vc[0], vc[1], vc[2]]);

        self.draw_triangle_three_color([vp[2], vp[3], vp[0]], [vc[2], vc[3], vc[0]]);
    }

    /// Draws a quadrilateral with the specified color.
    ///
    /// The vertex positions must be provided in clockwise order.
    #[inline]
    pub fn draw_quad(&mut self, vertex_positions_clockwise: [Vec2; 4], color: Color)
    {
        self.draw_quad_four_color(
            vertex_positions_clockwise,
            [color, color, color, color]
        );
    }

    /// Draws part of an image, tinted with the provided colors, at the
    /// specified location. The sub-image will be scaled to fill the
    /// quadrilateral described by the vertices in
    /// `vertex_positions_clockwise`.
    ///
    /// The coordinates in `image_coords_normalized` should be in the range
    /// `0.0` to `1.0`, and define the portion of the source image which
    /// should be drawn.
    ///
    /// The tinting is performed by for each pixel by multiplying each color
    /// component in the image pixel by the corresponding color component in
    /// the `color` parameter.
    ///
    /// The vertex positions (and associated colors and image coordinates) must
    /// be provided in clockwise order.
    #[inline]
    pub fn draw_quad_image_tinted_four_color(
        &mut self,
        vertex_positions_clockwise: [Vec2; 4],
        vertex_colors: [Color; 4],
        image_coords_normalized: [Vec2; 4],
        image: &ImageHandle
    )
    {
        let vp = vertex_positions_clockwise;
        let vc = vertex_colors;
        let ic = image_coords_normalized;

        self.draw_triangle_image_tinted_three_color(
            [vp[0], vp[1], vp[2]],
            [vc[0], vc[1], vc[2]],
            [ic[0], ic[1], ic[2]],
            image
        );

        self.draw_triangle_image_tinted_three_color(
            [vp[2], vp[3], vp[0]],
            [vc[2], vc[3], vc[0]],
            [ic[2], ic[3], ic[0]],
            image
        );
    }

    /// Draws part of an image, tinted with the provided color, at the specified
    /// location. The sub-image will be scaled to fill the pixel coordinates
    /// in the provided rectangle.
    ///
    /// The coordinates in `image_coords_normalized` should be in the range
    /// `0.0` to `1.0`, and define the portion of the source image which
    /// should be drawn.
    ///
    /// The tinting is performed by for each pixel by multiplying each color
    /// component in the image pixel by the corresponding color component in
    /// the `color` parameter.
    #[inline]
    pub fn draw_rectangle_image_subset_tinted(
        &mut self,
        rect: impl AsRef<Rectangle>,
        color: Color,
        image_coords_normalized: impl AsRef<Rectangle>,
        image: &ImageHandle
    )
    {
        let rect = rect.as_ref();
        let image_coords_normalized = image_coords_normalized.as_ref();

        self.draw_quad_image_tinted_four_color(
            [
                *rect.top_left(),
                rect.top_right(),
                *rect.bottom_right(),
                rect.bottom_left()
            ],
            [color, color, color, color],
            [
                *image_coords_normalized.top_left(),
                image_coords_normalized.top_right(),
                *image_coords_normalized.bottom_right(),
                image_coords_normalized.bottom_left()
            ],
            image
        );
    }

    /// Draws an image, tinted with the provided color, at the specified
    /// location. The image will be scaled to fill the pixel coordinates in
    /// the provided rectangle.
    ///
    /// The tinting is performed by for each pixel by multiplying each color
    /// component in the image pixel by the corresponding color component in
    /// the `color` parameter.
    #[inline]
    pub fn draw_rectangle_image_tinted(
        &mut self,
        rect: impl AsRef<Rectangle>,
        color: Color,
        image: &ImageHandle
    )
    {
        self.draw_rectangle_image_subset_tinted(
            rect,
            color,
            Rectangle::new(Vec2::ZERO, Vec2::new(1.0, 1.0)),
            image
        );
    }

    /// Draws an image at the specified location. The image will be
    /// scaled to fill the pixel coordinates in the provided rectangle.
    #[inline]
    pub fn draw_rectangle_image(
        &mut self,
        rect: impl AsRef<Rectangle>,
        image: &ImageHandle
    )
    {
        self.draw_rectangle_image_tinted(rect, Color::WHITE, image);
    }

    /// Draws an image at the specified pixel location. The image will be
    /// drawn at its original size with no scaling.
    #[inline]
    pub fn draw_image<P: Into<Vec2>>(&mut self, position: P, image: &ImageHandle)
    {
        let position = position.into();

        self.draw_rectangle_image(
            Rectangle::new(position, position + image.size().into_f32()),
            image
        );
    }

    /// Draws a single-color rectangle at the specified location. The
    /// coordinates of the rectangle are specified in pixels.
    #[inline]
    pub fn draw_rectangle(&mut self, rect: impl AsRef<Rectangle>, color: Color)
    {
        let rect = rect.as_ref();

        /*
        self.draw_quad(
            [
                *rect.top_left(),
                rect.top_right(),
                *rect.bottom_right(),
                rect.bottom_left()
            ],
            color
        );
        */
        //log::info!("rect: {:?} {:?}", rect, color);
        shapes::draw_rectangle(&mut self.gl, rect.left(), rect.top(), rect.width(), rect.height(), color);
    }

    /// Draws a single-color rounded rectangle at the specified location. The
    /// coordinates of the rounded rectangle are specified in pixels.
    #[inline]
    pub fn draw_rounded_rectangle(
        &mut self,
        round_rect: impl AsRef<RoundedRectangle>,
        color: Color
    )
    {
        let round_rect = round_rect.as_ref();
        shapes::draw_rectangle_ex2(
            &mut self.gl,
            round_rect.left(),
            round_rect.top(),
            round_rect.width(),
            round_rect.height(),
            &shapes::DrawRectangleParams2
            {
                color: color,
                border_radius: 8.0, //round_rect.radius,
                 border_radius_segments: 20,
                ..Default::default()
            }
            );

        /*
        let round_rect = round_rect.as_ref();

        //create 3 rectangles (the middle one is taller)
        //draw middle quad (the taller one)
        self.draw_quad(
            [
                round_rect.top_left() + Vec2::new(round_rect.radius(), 0.0),
                round_rect.top_right() + Vec2::new(-round_rect.radius(), 0.0),
                round_rect.bottom_right() + Vec2::new(-round_rect.radius(), 0.0),
                round_rect.bottom_left() + Vec2::new(round_rect.radius(), 0.0)
            ],
            color
        );

        //draw left quad
        self.draw_quad(
            [
                round_rect.top_left() + Vec2::new(0.0, round_rect.radius()),
                round_rect.top_left()
                    + Vec2::new(round_rect.radius(), round_rect.radius()),
                round_rect.bottom_left()
                    + Vec2::new(round_rect.radius(), -round_rect.radius()),
                round_rect.bottom_left() + Vec2::new(0.0, -round_rect.radius())
            ],
            color
        );

        //draw right quad
        self.draw_quad(
            [
                round_rect.top_right() + Vec2::new(0.0, round_rect.radius()),
                round_rect.top_right()
                    + Vec2::new(-round_rect.radius(), round_rect.radius()),
                round_rect.bottom_right()
                    + Vec2::new(-round_rect.radius(), -round_rect.radius()),
                round_rect.bottom_right() + Vec2::new(0.0, -round_rect.radius())
            ],
            color
        );

        //draw triangles
        self.draw_triangle(
            [
                round_rect.top_left() + Vec2::new(round_rect.radius(), 0.0),
                round_rect.top_left()
                    + Vec2::new(round_rect.radius(), round_rect.radius()),
                round_rect.top_left() + Vec2::new(0.0, round_rect.radius())
            ],
            color
        );
        self.draw_triangle(
            [
                round_rect.top_right() + Vec2::new(-round_rect.radius(), 0.0),
                round_rect.top_right()
                    + Vec2::new(-round_rect.radius(), round_rect.radius()),
                round_rect.top_right() + Vec2::new(0.0, round_rect.radius())
            ],
            color
        );
        self.draw_triangle(
            [
                round_rect.bottom_left() + Vec2::new(round_rect.radius(), 0.0),
                round_rect.bottom_left() + Vec2::new(0.0, -round_rect.radius()),
                round_rect.bottom_left()
                    + Vec2::new(round_rect.radius(), -round_rect.radius())
            ],
            color
        );
        self.draw_triangle(
            [
                round_rect.bottom_right() + Vec2::new(-round_rect.radius(), 0.0),
                round_rect.bottom_right()
                    + Vec2::new(-round_rect.radius(), -round_rect.radius()),
                round_rect.bottom_right() + Vec2::new(0.0, -round_rect.radius())
            ],
            color
        );

        //draw top right circle
        self.draw_circle_section_triangular_three_color(
            [
                round_rect.top_right() + Vec2::new(-round_rect.radius(), 0.0),
                round_rect.top_right(),
                round_rect.top_right() + Vec2::new(0.0, round_rect.radius())
            ],
            [color; 3],
            [
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(1.0, 0.0)
            ]
        );

        //draw top left circle
        self.draw_circle_section_triangular_three_color(
            [
                round_rect.top_left() + Vec2::new(0.0, round_rect.radius()),
                *round_rect.top_left(),
                round_rect.top_left() + Vec2::new(round_rect.radius(), 0.0)
            ],
            [color; 3],
            [
                Vec2::new(-1.0, 0.0),
                Vec2::new(-1.0, 1.0),
                Vec2::new(0.0, 1.0)
            ]
        );

        //draw bottom left circle
        self.draw_circle_section_triangular_three_color(
            [
                round_rect.bottom_left() + Vec2::new(round_rect.radius(), 0.0),
                round_rect.bottom_left(),
                round_rect.bottom_left() + Vec2::new(0.0, -round_rect.radius())
            ],
            [color; 3],
            [
                Vec2::new(0.0, -1.0),
                Vec2::new(-1.0, -1.0),
                Vec2::new(-1.0, 0.0)
            ]
        );

        // draw bottom right circle
        self.draw_circle_section_triangular_three_color(
            [
                round_rect.bottom_right() + Vec2::new(0.0, -round_rect.radius()),
                *round_rect.bottom_right(),
                round_rect.bottom_right() + Vec2::new(-round_rect.radius(), 0.0)
            ],
            [color; 3],
            [
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, -1.0),
                Vec2::new(0.0, -1.0)
            ]
        );
        */
    }

    /// Draws a single-color line between the given points, specified in pixels.
    ///
    /// # Pixel alignment
    ///
    /// On a display with square pixels, an integer-valued coordinate is located
    /// at the boundary between two pixels, rather than the center of the
    /// pixel. For example:
    ///
    ///  * `(0.0, 0.0)` = Top left of pixel
    ///  * `(0.5, 0.5)` = Center of pixel
    ///  * `(1.0, 1.0)` = Bottom right of pixel
    ///
    /// If drawing a line of odd-numbered thickness, it is advisable to locate
    /// the start and end of the line at the centers of pixels, rather than
    /// the edges.
    ///
    /// For example, a one-pixel-thick line between `(0.0, 10.0)` and `(100.0,
    /// 10.0)` will be drawn as a rectangle with corners `(0.0, 9.5)` and
    /// `(100.0, 10.5)`, meaning that the line's thickness will actually
    /// span two half-pixels. Drawing the same line between `(0.0, 10.5)`
    /// and `(100.0, 10.5)` will result in a pixel-aligned rectangle between
    /// `(0.0, 10.0)` and `(100.0, 11.0)`.
    pub fn draw_line<VStart: Into<Vec2>, VEnd: Into<Vec2>>(
        &mut self,
        start_position: VStart,
        end_position: VEnd,
        thickness: f32,
        color: Color
    )
    {
        let start_position = start_position.into();
        let end_position = end_position.into();

        let gradient_normalized = match (end_position - start_position).normalize() {
            None => return,
            Some(gradient) => gradient
        };

        let gradient_thickness = gradient_normalized * (thickness / 2.0);

        let offset_anticlockwise = gradient_thickness.rotate_90_degrees_anticlockwise();
        let offset_clockwise = gradient_thickness.rotate_90_degrees_clockwise();

        let start_anticlockwise = start_position + offset_anticlockwise;
        let start_clockwise = start_position + offset_clockwise;

        let end_anticlockwise = end_position + offset_anticlockwise;
        let end_clockwise = end_position + offset_clockwise;

        self.draw_quad(
            [
                start_anticlockwise,
                end_anticlockwise,
                end_clockwise,
                start_clockwise
            ],
            color
        );
    }

    /// Draws a circle, filled with a single color, at the specified pixel
    /// location.
    pub fn draw_circle<V: Into<Vec2>>(
        &mut self,
        center_position: V,
        radius: f32,
        color: Color
    )
    {
    }

    /// Draws a triangular subset of a circle.
    ///
    /// Put simply, this function will draw a triangle on the screen, textured
    /// with a region of a circle.
    ///
    /// The circle region is specified using `vertex_circle_coords_normalized`,
    /// which denotes UV coordinates relative to an infinitely-detailed
    /// circle of radius `1.0`, and center `(0.0, 0.0)`.
    ///
    /// For example, to draw the top-right half of a circle with radius 100px:
    ///
    /// ```rust,no_run
    /// # use speedy2d::GLRenderer;
    /// # use speedy2d::dimen::Vec2;
    /// # use speedy2d::color::Color;
    /// # let mut renderer = unsafe {
    /// #     GLRenderer::new_for_gl_context((640, 480), |fn_name| {
    /// #         std::ptr::null() as *const _
    /// #     })
    /// # }.unwrap();
    /// # renderer.draw_frame(|graphics| {
    /// graphics.draw_circle_section_triangular_three_color(
    ///         [
    ///                 Vec2::new(200.0, 200.0),
    ///                 Vec2::new(300.0, 200.0),
    ///                 Vec2::new(300.0, 300.0)],
    ///         [Color::MAGENTA; 3],
    ///         [
    ///                 Vec2::new(-1.0, -1.0),
    ///                 Vec2::new(1.0, -1.0),
    ///                 Vec2::new(1.0, 1.0)]);
    /// # });
    /// ```
    #[inline]
    pub fn draw_circle_section_triangular_three_color(
        &mut self,
        vertex_positions_clockwise: [Vec2; 3],
        vertex_colors: [Color; 3],
        vertex_circle_coords_normalized: [Vec2; 3]
        )
    {
        /*
        shapes::draw_triangle(
            &mut self.gl, 
            glam::Vec2::new(vertex_positions_clockwise[0].x, vertex_positions_clockwise[0].y),
            glam::Vec2::new(vertex_positions_clockwise[1].x, vertex_positions_clockwise[1].y),
            glam::Vec2::new(vertex_positions_clockwise[2].x, vertex_positions_clockwise[2].y),
            vertex_colors[0]);
            */
        //self.renderer.draw_circle_section(
            //vertex_positions_clockwise,
            //vertex_colors,
            //vertex_circle_coords_normalized
        //);
    }

    /// Sets the current clip to the rectangle specified by the given
    /// coordinates. Rendering operations have no effect outside of the
    /// clipping area.
    pub fn set_clip(&mut self, rect: Option<Rectangle<i32>>)
    {
    }

    /// Captures a screenshot of the render window. The returned data contains
    /// the color of each pixel. Pixels are represented using a `u8` for each
    /// component (red, green, blue, and alpha). Use the `format` parameter to
    /// specify the byte layout (and size) of each pixel.
    pub fn capture(&mut self, format: ImageDataType) -> RawBitmapData
    {
        panic!();
    }

    fn begin_frame(&mut self) {
        self.gl.reset();
    }

    pub(crate) fn pixel_perfect_projection_matrix(&self) -> glam::Mat4 {
        let (width, height) = miniquad::window::screen_size();
        let dpi = miniquad::window::dpi_scale();

        glam::Mat4::orthographic_rh_gl(0., width / dpi, height / dpi, 0., -1., 1.)
    }

    fn end_frame(&mut self) {
        let screen_mat = self.pixel_perfect_projection_matrix();
        self.gl.draw(&mut *self.renderer, screen_mat);

        self.renderer.commit_frame();
    }

}

/// Struct representing a window.
pub struct Window<UserEventType = ()>
where
    UserEventType: 'static
{
    window_impl: WindowQuad<UserEventType>,
}

impl Window<()>
{
    /// Create a new window, centered in the middle of the primary monitor.
    pub fn new_centered<Str, Size>(
        title: Str,
        size: Size
    ) -> Result<Window<()>, BacktraceError<WindowCreationError>>
    where
        Str: AsRef<str>,
        Size: Into<UVec2>
    {
        let size = size.into();

        Self::new_with_options(
            title.as_ref(),
            WindowCreationOptions::new_windowed(
                WindowSize::PhysicalPixels(size),
                Some(WindowPosition::Center)
            )
        )
    }

    /// Create a new window, in fullscreen borderless mode on the primary
    /// monitor.
    pub fn new_fullscreen_borderless<Str>(
        title: Str
    ) -> Result<Window<()>, BacktraceError<WindowCreationError>>
    where
        Str: AsRef<str>
    {
        Self::new_with_options(
            title.as_ref(),
            WindowCreationOptions::new_fullscreen_borderless()
        )
    }

    /// Create a new window with the specified options.
    pub fn new_with_options(
        title: &str,
        options: WindowCreationOptions
    ) -> Result<Window<()>, BacktraceError<WindowCreationError>>
    {
        Self::new_with_user_events(title, options)
    }
}

//#[cfg(any(doc, doctest, all(feature = "windowing", not(target_arch = "wasm32"))))]
impl<UserEventType: 'static> Window<UserEventType>
{
    /// Create a new window with the specified options, with support for user
    /// events. See [window::UserEventSender].
    pub fn new_with_user_events(
        title: &str,
        options: WindowCreationOptions
    ) -> Result<Self, BacktraceError<WindowCreationError>>
    {
        let window_impl = WindowQuad::<UserEventType>::new(title, options)?;

        Ok(Window {
            window_impl
        })
    }

    /// Creates a [window::UserEventSender], which can be used to post custom
    /// events to this event loop from another thread.
    ///
    /// If calling this, specify the type of the event data using
    /// `Window::<YourTypeHere>::new_with_user_events()`.
    ///
    /// See [UserEventSender::send_event], [WindowHandler::on_user_event].
    pub fn create_user_event_sender(&self) -> UserEventSender<UserEventType>
    {
        self.window_impl.create_user_event_sender()
    }

    /// Run the window event loop, with the specified callback handler.
    ///
    /// Once the event loop finishes running, the entire app will terminate,
    /// even if other threads are still running. See
    /// [window::WindowHelper::terminate_loop()].
    pub fn run_loop<H>(self, handler: H) -> !
    where
        H: WindowHandler<UserEventType> + 'static
    {
        self.window_impl.run_loop(handler);
    }
}

