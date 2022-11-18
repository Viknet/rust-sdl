extern crate libc;

use libc::c_int;
use std::ffi::CString;
use std::path::Path;

use sdl::get_error;
use sdl::video::{Color, Surface};

// Setup linking for all targets.
#[cfg(any(not(target_os = "macos"), not(mac_framework)))]
#[link(name = "SDL_ttf")]
extern "C" {}

#[cfg(all(target_os = "macos", mac_framework))]
#[link(name = "SDL_ttf", kind = "framework")]
extern "C" {}

pub mod ll {
    #![allow(non_camel_case_types)]
    use libc::{c_char, c_int};
    use sdl::video::ll::{SDL_Color, SDL_Surface};
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct _TTF_Font {
        _unused: [u8; 0],
    }
    pub type TTF_Font = _TTF_Font;

    pub const SDL_TTF_MAJOR_VERSION: u32 = 2;
    pub const SDL_TTF_MINOR_VERSION: u32 = 0;
    pub const SDL_TTF_PATCHLEVEL: u32 = 11;
    pub const TTF_MAJOR_VERSION: u32 = 2;
    pub const TTF_MINOR_VERSION: u32 = 0;
    pub const TTF_PATCHLEVEL: u32 = 11;
    pub const UNICODE_BOM_NATIVE: u32 = 65279;
    pub const UNICODE_BOM_SWAPPED: u32 = 65534;
    pub const TTF_STYLE_NORMAL: u32 = 0;
    pub const TTF_STYLE_BOLD: u32 = 1;
    pub const TTF_STYLE_ITALIC: u32 = 2;
    pub const TTF_STYLE_UNDERLINE: u32 = 4;
    pub const TTF_STYLE_STRIKETHROUGH: u32 = 8;
    pub const TTF_HINTING_NORMAL: u32 = 0;
    pub const TTF_HINTING_LIGHT: u32 = 1;
    pub const TTF_HINTING_MONO: u32 = 2;
    pub const TTF_HINTING_NONE: u32 = 3;

    extern "C" {
        // pub fn TTF_Linked_Version() -> *const SDL_version;
        // pub fn TTF_ByteSwappedUNICODE(swapped: c_int);

        pub fn TTF_Init() -> c_int;

        pub fn TTF_OpenFont(file: *const c_char, ptsize: c_int) -> *mut TTF_Font;

        // pub fn TTF_OpenFontIndex(
        //     file: *const c_char,
        //     ptsize: c_int,
        //     index: c_long,
        // ) -> *mut TTF_Font;
        // pub fn TTF_OpenFontRW(
        //     src: *mut SDL_RWops,
        //     freesrc: c_int,
        //     ptsize: c_int,
        // ) -> *mut TTF_Font;
        // pub fn TTF_OpenFontIndexRW(
        //     src: *mut SDL_RWops,
        //     freesrc: c_int,
        //     ptsize: c_int,
        //     index: c_long,
        // ) -> *mut TTF_Font;
        // pub fn TTF_GetFontStyle(font: *const TTF_Font) -> c_int;
        // pub fn TTF_SetFontStyle(font: *mut TTF_Font, style: c_int);
        // pub fn TTF_GetFontOutline(font: *const TTF_Font) -> c_int;
        // pub fn TTF_SetFontOutline(font: *mut TTF_Font, outline: c_int);
        // pub fn TTF_GetFontHinting(font: *const TTF_Font) -> c_int;
        // pub fn TTF_SetFontHinting(font: *mut TTF_Font, hinting: c_int);
        // pub fn TTF_FontHeight(font: *const TTF_Font) -> c_int;
        // pub fn TTF_FontAscent(font: *const TTF_Font) -> c_int;
        // pub fn TTF_FontDescent(font: *const TTF_Font) -> c_int;
        // pub fn TTF_FontLineSkip(font: *const TTF_Font) -> c_int;
        // pub fn TTF_GetFontKerning(font: *const TTF_Font) -> c_int;
        // pub fn TTF_SetFontKerning(font: *mut TTF_Font, allowed: c_int);
        // pub fn TTF_FontFaces(font: *const TTF_Font) -> c_long;
        // pub fn TTF_FontFaceIsFixedWidth(font: *const TTF_Font) -> c_int;
        // pub fn TTF_FontFaceFamilyName(font: *const TTF_Font) -> *mut c_char;
        // pub fn TTF_FontFaceStyleName(font: *const TTF_Font) -> *mut c_char;
        // pub fn TTF_GlyphIsProvided(font: *const TTF_Font, ch: Uint16) -> c_int;
        // pub fn TTF_GlyphMetrics(
        //     font: *mut TTF_Font,
        //     ch: Uint16,
        //     minx: *mut c_int,
        //     maxx: *mut c_int,
        //     miny: *mut c_int,
        //     maxy: *mut c_int,
        //     advance: *mut c_int,
        // ) -> c_int;
        // pub fn TTF_SizeText(
        //     font: *mut TTF_Font,
        //     text: *const c_char,
        //     w: *mut c_int,
        //     h: *mut c_int,
        // ) -> c_int;

        // pub fn TTF_SizeUTF8(
        //     font: *mut TTF_Font,
        //     text: *const c_char,
        //     w: *mut c_int,
        //     h: *mut c_int,
        // ) -> c_int;

        // pub fn TTF_SizeUNICODE(
        //     font: *mut TTF_Font,
        //     text: *const Uint16,
        //     w: *mut c_int,
        //     h: *mut c_int,
        // ) -> c_int;
        // pub fn TTF_RenderText_Solid(
        //     font: *mut TTF_Font,
        //     text: *const c_char,
        //     fg: SDL_Color,
        // ) -> *mut SDL_Surface;

        pub fn TTF_RenderUTF8_Solid(
            font: *mut TTF_Font,
            text: *const c_char,
            fg: SDL_Color,
        ) -> *mut SDL_Surface;

        // pub fn TTF_RenderUNICODE_Solid(
        //     font: *mut TTF_Font,
        //     text: *const Uint16,
        //     fg: SDL_Color,
        // ) -> *mut SDL_Surface;
        // pub fn TTF_RenderGlyph_Solid(
        //     font: *mut TTF_Font,
        //     ch: Uint16,
        //     fg: SDL_Color,
        // ) -> *mut SDL_Surface;
        // pub fn TTF_RenderText_Shaded(
        //     font: *mut TTF_Font,
        //     text: *const c_char,
        //     fg: SDL_Color,
        //     bg: SDL_Color,
        // ) -> *mut SDL_Surface;

        // pub fn TTF_RenderUTF8_Shaded(
        //     font: *mut TTF_Font,
        //     text: *const c_char,
        //     fg: SDL_Color,
        //     bg: SDL_Color,
        // ) -> *mut SDL_Surface;

        // pub fn TTF_RenderUNICODE_Shaded(
        //     font: *mut TTF_Font,
        //     text: *const Uint16,
        //     fg: SDL_Color,
        //     bg: SDL_Color,
        // ) -> *mut SDL_Surface;
        // pub fn TTF_RenderGlyph_Shaded(
        //     font: *mut TTF_Font,
        //     ch: Uint16,
        //     fg: SDL_Color,
        //     bg: SDL_Color,
        // ) -> *mut SDL_Surface;
        // pub fn TTF_RenderText_Blended(
        //     font: *mut TTF_Font,
        //     text: *const c_char,
        //     fg: SDL_Color,
        // ) -> *mut SDL_Surface;

        // pub fn TTF_RenderUTF8_Blended(
        //     font: *mut TTF_Font,
        //     text: *const c_char,
        //     fg: SDL_Color,
        // ) -> *mut SDL_Surface;

        // pub fn TTF_RenderUNICODE_Blended(
        //     font: *mut TTF_Font,
        //     text: *const Uint16,
        //     fg: SDL_Color,
        // ) -> *mut SDL_Surface;
        // pub fn TTF_RenderGlyph_Blended(
        //     font: *mut TTF_Font,
        //     ch: Uint16,
        //     fg: SDL_Color,
        // ) -> *mut SDL_Surface;
        pub fn TTF_CloseFont(font: *mut TTF_Font);
        pub fn TTF_Quit();
        // pub fn TTF_WasInit() -> c_int;
        // pub fn TTF_GetFontKerningSize(
        //     font: *mut TTF_Font,
        //     prev_index: c_int,
        //     index: c_int,
        // ) -> c_int;
    }
}

pub struct Font {
    raw: *mut ll::TTF_Font,
}

pub fn init() -> Result<(), String> {
    let init_result = unsafe { ll::TTF_Init() };

    if init_result == 0 {
        Ok(())
    } else {
        Err("SDL_ttf init error.".to_string())
    }
}

pub fn open_font(file: &Path, pt: isize) -> Result<Font, String> {
    let cfile = CString::new(file.to_str().unwrap()).unwrap();
    unsafe {
        let raw = ll::TTF_OpenFont(cfile.as_ptr(), pt as c_int);

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Font { raw })
        }
    }
}

pub fn render_utf8_solid(font: &Font, text: String, color: &Color) -> Result<Surface, String> {
    let raw_font = font.raw;
    let text = CString::new(text).unwrap();
    let fg = color.to_struct();

    unsafe {
        let raw_surface = ll::TTF_RenderUTF8_Solid(raw_font, text.as_ptr(), fg);

        if raw_surface.is_null() {
            Err(get_error())
        } else {
            Ok(Surface {
                raw: raw_surface,
                owned: true,
            })
        }
    }
}

pub fn close_font(font: Font) {
    let raw = font.raw;

    unsafe {
        ll::TTF_CloseFont(raw);
    }
}

pub fn quit() {
    unsafe {
        ll::TTF_Quit();
    }
}
