use notan::prelude::Color;
use crate::game::raycast::Perspective;
use crate::game::texture::sampler::TextureSampler;

pub trait DrawColumn {
    fn draw_column(
        &self,
        column: &mut [[u8; 4]],
        column_height: f32,
        tex_x: f32,
        perspective: Perspective,
        draw_callback: impl FnMut(&mut [u8; 4], [u8; 4]),
    );
}

fn calculate_perspective(column: &mut [[u8; 4]], column_height: f32, perspective: Perspective) -> (&mut [[u8; 4]], f32, f32) {
    puffin::profile_function!();
    let screen_height = column.len();
    let screen_middle = screen_height as f32 / 2.0;
    let horizon_height = perspective.horizon_height;
    let y_offset = perspective.y_offset;

    let column_start = screen_middle - column_height * (1.0 - horizon_height) + y_offset;
    let column_end = screen_middle + column_height * horizon_height + y_offset;

    let inverse_lerp = |a, b, v| -> _ { (v - a) / (b - a) };
    let tex_v_start = inverse_lerp(column_start, column_end, 0.0).max(0.0);
    let tex_v_end = inverse_lerp(column_start, column_end, screen_height as f32).min(1.0);

    let column_start = (column_start.round() as usize).min(screen_height);
    let column_end = (column_end.round() as usize).min(screen_height);

    let column_part = &mut column[column_start..column_end];

    (column_part, tex_v_start, tex_v_end)
}

impl DrawColumn for TextureSampler {
    fn draw_column(
        &self,
        column: &mut [[u8; 4]],
        tex_x: f32,
        column_height: f32,
        perspective: Perspective,
        mut draw_callback: impl FnMut(&mut [u8; 4], [u8; 4]),
    ) {
        puffin::profile_function!();

        let (column_part, tex_v_start, tex_v_end) =
            calculate_perspective(column, column_height, perspective);

        let sampled_colors = self.sample_column(tex_x, tex_v_start..tex_v_end, column_part.len());

        puffin::profile_scope!("write pixels");
        for (new, current) in sampled_colors.zip(column_part) {
            draw_callback(current, new);
        }
    }
}

impl DrawColumn for Color {
    fn draw_column(
        &self,
        column: &mut [[u8; 4]],
        tex_x: f32,
        column_height: f32,
        perspective: Perspective,
        draw_callback: impl FnMut(&mut [u8; 4], [u8; 4]),
    ) {
        self.rgba_u8().draw_column(column, tex_x, column_height, perspective, draw_callback)
    }
}

impl DrawColumn for [u8; 4] {
    fn draw_column(
        &self,
        column: &mut [[u8; 4]],
        _tex_x: f32,
        column_height: f32,
        perspective: Perspective,
        mut draw_callback: impl FnMut(&mut [u8; 4], [u8; 4]),
    ) {
        puffin::profile_function!();

        let (column_part, _, _) =
            calculate_perspective(column, column_height, perspective);

        puffin::profile_scope!("write pixels");
        for px in column_part {
            draw_callback(px, *self);
        }
    }
}