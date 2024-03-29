use crate::model::TestResult;
use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine as _;
use image::io::Reader as ImageReader;
use image::ImageOutputFormat;
use image::{imageops, DynamicImage, ImageBuffer, Luma, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use imageproc::rect::Rect;
use plot_icon::generate_png;
use qrcode::{EcLevel, QrCode};
use rusttype::{Font, Scale};
use std::cmp;
use std::io::Cursor;

pub fn create_certificate(
    test_result: &TestResult,
    url: &str,
    issuer: &str,
) -> Result<DynamicImage> {
    let cert_width = 800;
    let cert_height = 800;
    let border_color = Rgba([0, 123, 255, 255]);
    let border_thickness = 10u32;
    let highlight_color = Rgba([0, 123, 255, 255]);
    let text_color = Rgba([0, 0, 0, 255]);
    let background_color = Rgba([249, 249, 249, 255]);
    let qr_code_size = 250;

    let qr_code = QrCode::with_error_correction_level(url, EcLevel::H).unwrap();
    let qr_code_image = qr_code.render::<Luma<u8>>().quiet_zone(false).build();

    let qr_code_image_rgba =
        ImageBuffer::from_fn(qr_code_image.width(), qr_code_image.height(), |x, y| {
            let pixel = qr_code_image.get_pixel(x, y);
            if pixel[0] == 0 {
                Rgba([0, 0, 0, 255])
            } else {
                Rgba([255, 255, 255, 255])
            }
        });

    let resized_qr_code_image = imageops::resize(
        &qr_code_image_rgba,
        qr_code_size,
        qr_code_size,
        imageops::FilterType::Nearest,
    );

    let mut cert_image = RgbaImage::new(cert_width, cert_height);

    let logo_bytes = include_bytes!("../assets/favicon.png");
    let logo_image = ImageReader::new(Cursor::new(logo_bytes))
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode image");

    let identicon_image = {
        let data: Vec<u8> = generate_png(test_result.to_base64().as_bytes(), 75)
            .expect("Failed to generate identicon");
        let mut image = ImageReader::new(Cursor::new(data));
        image.set_format(image::ImageFormat::Png);
        image.decode().expect("Failed to decode image")
    };

    let scaled_logo_image = imageops::resize(&logo_image, 75, 75, imageops::FilterType::Nearest);

    imageproc::drawing::draw_filled_rect_mut(
        &mut cert_image,
        imageproc::rect::Rect::at(0, 0).of_size(cert_width, cert_height),
        border_color,
    );

    imageproc::drawing::draw_filled_rect_mut(
        &mut cert_image,
        Rect::at(border_thickness as i32, border_thickness as i32).of_size(
            cert_width - 2 * border_thickness,
            cert_height - 2 * border_thickness,
        ),
        background_color,
    );

    let font_data: &[u8] = include_bytes!("../assets/Lora-Regular.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();

    image::imageops::overlay(&mut cert_image, &scaled_logo_image, 80, 80);
    let cert_width = cert_image.width();
    image::imageops::overlay(
        &mut cert_image,
        &identicon_image,
        cert_width as i64 - 80 - 75,
        80,
    );

    let title = "Certificate of Completion";
    let scale_title = Scale::uniform(40.0);
    let title_width = font.width(scale_title, title);
    draw_text_mut(
        &mut cert_image,
        highlight_color,
        cmp::max(0, (cert_width as i32 - title_width as i32) / 2) as i32,
        80,
        scale_title,
        &font,
        title,
    );

    let name = &test_result.player_name;
    let scale_name = Scale::uniform(20.0);
    let name_width = font.width(scale_name, name);
    draw_text_mut(
        &mut cert_image,
        text_color,
        cmp::max(0, (cert_width as i32 - name_width as i32) / 2) as i32,
        150,
        scale_name,
        &font,
        name,
    );

    let message = format!(
        "has completed the {} test with {} correct answers out of {} total questions.",
        test_result.test_type, test_result.correct_answers, test_result.total_questions
    );
    let scale_message = Scale::uniform(20.0);
    let message_width = font.width(scale_message, &message);
    draw_text_mut(
        &mut cert_image,
        text_color,
        cmp::max(0, (cert_width as i32 - message_width as i32) / 2) as i32,
        200,
        scale_message,
        &font,
        &message,
    );

    let performance_message = format!("Performance: {:.2}%", test_result.performance_percentage);
    let scale_performance = Scale::uniform(20.0);
    let performance_width = font.width(scale_performance, &performance_message);
    draw_text_mut(
        &mut cert_image,
        Rgba([0u8, 0u8, 0u8, 255u8]),
        cmp::max(0, (cert_width as i32 - performance_width as i32) / 2) as i32,
        300,
        scale_performance,
        &font,
        &performance_message,
    );

    // Drawing the date
    let date_str = format!("{}", test_result.date.format("%d %B %Y"));
    let scale_date = Scale::uniform(20.0);
    let date_width = font.width(scale_date, &date_str);
    draw_text_mut(
        &mut cert_image,
        text_color,
        cmp::max(0, (cert_width as i32 - date_width as i32) / 2) as i32,
        350,
        scale_date,
        &font,
        &date_str,
    );

    let issued_by_message = format!("Issued by {}", issuer);
    let scale_issued_by = Scale::uniform(20.0);
    let issued_by_width = font.width(scale_issued_by, &issued_by_message);
    draw_text_mut(
        &mut cert_image,
        highlight_color,
        cmp::max(0, (cert_width as i32 - issued_by_width as i32) / 2) as i32,
        400,
        scale_issued_by,
        &font,
        &issued_by_message,
    );

    let qr_code_size = resized_qr_code_image.width() as u32;
    image::imageops::overlay(
        &mut cert_image,
        &resized_qr_code_image,
        cmp::max(0, (cert_width as i32 - qr_code_size as i32) / 2) as i64,
        (cert_height - qr_code_size - 50) as i64,
    );
    Ok(DynamicImage::ImageRgba8(cert_image))
}

pub trait FontExt {
    fn width(&self, scale: Scale, text: &str) -> u32;
}

impl FontExt for Font<'_> {
    fn width(&self, scale: Scale, text: &str) -> u32 {
        let v_metrics = self.v_metrics(scale);
        let glyphs: Vec<_> = self
            .layout(text, scale, rusttype::point(0.0, v_metrics.ascent))
            .collect();
        glyphs
            .iter()
            .map(|g| g.pixel_bounding_box().map_or(0, |b| b.width()))
            .sum::<i32>() as u32
    }
}

pub fn create_certificate_data_url(
    test_result: &TestResult,
    url: &str,
    issuer: &str,
) -> Result<String> {
    let image = create_certificate(test_result, url, issuer)?;

    let mut image_data: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = general_purpose::STANDARD.encode(image_data);
    Ok(format!("data:image/png;base64,{}", res_base64))
}
