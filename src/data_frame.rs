#[derive(Debug)]
pub struct DataFrame {
    frame_type: u8,
    id: u32,
    device_type: u8,
    battery: u8,
    data: f32,
}

impl DataFrame {
    pub fn parse(msg: [u8; 13]) -> Result<DataFrame, ()> {
        let header = msg[0];
        let payload = &msg[1..12];
        let checksum = msg[12];

        match header {
            0xE0 if payload.iter().fold(0, |acc, cur| acc ^ cur) == checksum => {
                let frame_type = payload[0];
                let id = u32::from_be_bytes([payload[1], payload[2], payload[3], payload[4]]);
                let device_type = payload[5];
                let battery = payload[6];
                let data = f32::from_bits(u32::from_le_bytes([
                    payload[7],
                    payload[8],
                    payload[9],
                    payload[10],
                ]));

                Ok(DataFrame {
                    frame_type,
                    id,
                    device_type,
                    battery,
                    data,
                })
            }
            _ => Err(()),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn frame_type(&self) -> u8 {
        self.frame_type
    }

    pub fn device_type(&self) -> u8 {
        self.device_type
    }

    pub fn battery(&self) -> u8 {
        self.battery
    }

    pub fn data(&self) -> f32 {
        self.data
    }
}
