use rotatar_types::Color;

pub trait ToIcedColor {
    fn into_iced(self) -> iced::Color;
}

impl ToIcedColor for Color {
    fn into_iced(self) -> iced::Color {
        iced::Color {
            r: self.r() as f32 / 255.0,
            g: self.g() as f32 / 255.0,
            b: self.b() as f32 / 255.0,
            a: 1.0,
        }
    }
}
