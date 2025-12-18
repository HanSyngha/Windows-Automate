// Screen capture implementation (Windows)

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use image::{ImageBuffer, ImageEncoder, Rgba};
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/// Capture the entire screen as base64 encoded PNG
pub fn capture_screen_base64() -> Result<String> {
    unsafe {
        // Get screen dimensions
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);

        // Get device context
        let hdc_screen = GetDC(None);
        if hdc_screen.is_invalid() {
            return Err(anyhow::anyhow!("Failed to get screen DC"));
        }

        // Create compatible DC
        let hdc_mem = CreateCompatibleDC(Some(hdc_screen));
        if hdc_mem.is_invalid() {
            ReleaseDC(None, hdc_screen);
            return Err(anyhow::anyhow!("Failed to create compatible DC"));
        }

        // Create compatible bitmap
        let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);
        if hbitmap.is_invalid() {
            DeleteDC(hdc_mem);
            ReleaseDC(None, hdc_screen);
            return Err(anyhow::anyhow!("Failed to create bitmap"));
        }

        // Select bitmap into DC
        let old_bitmap = SelectObject(hdc_mem, hbitmap.into());

        // Copy screen to bitmap
        let _ = BitBlt(hdc_mem, 0, 0, width, height, Some(hdc_screen), 0, 0, SRCCOPY);

        // Get bitmap bits
        let mut bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height, // Top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut pixels: Vec<u8> = vec![0; (width * height * 4) as usize];

        GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bitmap_info,
            DIB_RGB_COLORS,
        );

        // Cleanup
        SelectObject(hdc_mem, old_bitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(None, hdc_screen);

        // Convert BGRA to RGBA
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2); // Swap B and R
        }

        // Create image buffer
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width as u32, height as u32, pixels)
                .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;

        // Encode to PNG
        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.write_image(
            img.as_raw(),
            width as u32,
            height as u32,
            image::ExtendedColorType::Rgba8,
        )?;

        // Base64 encode
        let base64_data = STANDARD.encode(&png_data);

        Ok(format!("data:image/png;base64,{}", base64_data))
    }
}
