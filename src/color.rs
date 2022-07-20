use crate::utils::clamp;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn hex(hex: &str) -> Result<Self, ()> {
        if !hex.starts_with('#') {
            return Err(());
        }
        if let Ok(color_value) = u32::from_str_radix(hex.trim_start_matches('#'), 16) {
            eprintln!("cl: {}", color_value);
            return match hex.len() {
                4 => {
                    let r = ((color_value >> 8) & 0xF) as u8;
                    let g = ((color_value >> 4) & 0xF) as u8;
                    let b = (color_value & 0xF) as u8;
                    Ok(Color::rgb(r | r << 4, g | g << 4, b | b << 4))
                }
                7 => Ok(Color::rgb(
                    ((color_value >> 16) & 0xFF) as u8,
                    ((color_value >> 8) & 0xFF) as u8,
                    (color_value & 0xFF) as u8,
                )),
                _ => Err(()),
            };
        }
        Err(())
    }

    pub fn lerp_to(&self, to: &Color, amount: f32) -> Color {
        let lerp_component = |start: u8, end: u8, amount: f32| -> u8 {
            let dx = end as f32 - start as f32;
            let r = start as f32 + dx * amount;
            clamp(r, 0.0, 254.0) as u8
        };

        Color::rgb(
            lerp_component(self.red, to.red, amount),
            lerp_component(self.green, to.green, amount),
            lerp_component(self.blue, to.blue, amount),
        )
    }
}
