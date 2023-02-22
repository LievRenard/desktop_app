use tui::{
    backend::{Backend},
    layout::{Alignment},
    style::{Color, Style},
    text::{Span},
    widgets::{
        canvas::{Canvas, Rectangle, Points},
        Block, BorderType, Borders},
    Frame
};
use image::{DynamicImage};
use crate::settings::programs::PROGRAM_LIST;

pub fn draw<B: Backend>(f: &mut Frame<B>, app_select: i32){
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();
    let icon_size : f64 = 48.0;
    let size_x : f64 = (48*6+24*7) as f64;
    let size_y : f64 = (48*3+24*4) as f64;

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Apps")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    // draw app icons
    let canvas = Canvas::default()
        .x_bounds([0.0, size_x])
        .y_bounds([0.0, size_y])
        .paint(|ctx| {
            ctx.layer();
            for order in 0..20 {
                if order < PROGRAM_LIST.len() {
                    //App icon draw
                    let (icon_name, icon_path, _) = PROGRAM_LIST[order];
                    let icon_pixels = get_icon(icon_path.to_string());
                    for i in 0..(48*48) {
                        ctx.draw(&Points {
                            coords: &[(24.0 + (icon_size + 24.0) * (order % 6) as f64 + (i%48) as f64, 24.0 + (icon_size + 24.0) * (2 - order / 6) as f64 + (i/48) as f64)],
                            color: Color::Rgb(icon_pixels[i].0[0], icon_pixels[i].0[1], icon_pixels[i].0[2])
                        });
                    }
                    //App Selection
                    if order == (app_select as usize) {
                        ctx.draw(&Rectangle {
                            x: 24.0 + (icon_size + 24.0) * (order % 6) as f64,
                            y: 24.0 + (icon_size + 24.0) * (2 - order / 6) as f64,
                            width: icon_size,
                            height: icon_size,
                            color: Color::Yellow
                        });
                        //App name draw
                        ctx.print(
                            24.0 + (icon_size + 24.0) * (order % 6) as f64,
                            12.0 + (icon_size + 24.0) * (2 - order / 6) as f64,
                            Span::styled(icon_name, Style::default().fg(Color::Yellow)),
                        );
                    }
                    else {
                        ctx.print(
                            24.0 + (icon_size + 24.0) * (order % 6) as f64,
                            12.0 + (icon_size + 24.0) * (2 - order / 6) as f64,
                            Span::styled(icon_name, Style::default().fg(Color::White)),
                        );
                    }
                }
            }
        });
    f.render_widget(canvas, size);
}

fn get_icon(path: String) -> Vec<image::Rgb<u8>> {
    let mut icon_pixels: Vec<image::Rgb<u8>> = Vec::new();
    let img = image::open(path).unwrap().into_rgb8();
    let resized_img = DynamicImage::from(img).resize(48, 48, image::imageops::Triangle).into_rgb8();
    for x in (0..48).rev() {
        for y in 0..48 {
            icon_pixels.push(*resized_img.get_pixel(y,x));
        }
    }
    icon_pixels
}