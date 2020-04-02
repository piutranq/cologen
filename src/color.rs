#[derive(Debug)]
pub struct Color (u8, u8, u8, u8);

impl Color {
    pub fn from_vec(vec: &Vec<u8>) -> Color {
        Color (
            match vec.get(0) { Some(&byte) => byte, None => 0x00 },
            match vec.get(1) { Some(&byte) => byte, None => 0x00 },
            match vec.get(2) { Some(&byte) => byte, None => 0x00 },
            match vec.get(3) { Some(&byte) => byte, None => 0xFF },
        )
    }

    pub fn value
        (&self, format: char, element: char)
        -> String
    {
        let raw: u8 = match element {
            'R' => self.0,
            'G' => self.1,
            'B' => self.2,
            'A' => self.3,
            _  => unreachable!()
        };
        match format {
            'd' => format!("{}", raw),
            'p' => format!("{}", (raw as usize * 100) / 255) ,
            '.' => format!("{:.2}", (raw as f32 / 255.0)),
            'x' => if raw < 16 {
                format!("0{:x}", raw)
            } else {
                format!("{:x}", raw)
            },
            _ => unreachable!()
        }
    }
}
