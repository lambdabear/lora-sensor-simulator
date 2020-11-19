#[derive(Debug)]
pub struct DataFrame {
    frame_type: u8,
    id: [u8; 4],
    device_type: u8,
    battery: u8,
    data: f32,
}

impl DataFrame {
    pub fn new(frame_type: u8, id: [u8; 4], device_type: u8, battery: u8, data: f32) -> Self {
        DataFrame {
            frame_type,
            id,
            device_type,
            battery,
            data,
        }
    }
    pub fn parse(msg: [u8; 13]) -> Result<DataFrame, ()> {
        let header = msg[0];
        let payload = &msg[1..12];
        let checksum = msg[12];

        match header {
            0xE0 if payload.iter().fold(0, |acc, cur| acc ^ cur) == checksum => {
                let frame_type = payload[0];
                let id = [payload[1], payload[2], payload[3], payload[4]];
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

    pub fn encode(&self) -> [u8; 13] {
        let mut buffer = [0_u8; 13];
        let header: u8 = 0xE0;
        let id = self.id;
        let data = self.data.to_bits().to_le_bytes();

        buffer[0] = header;
        buffer[1] = self.frame_type;
        buffer[2] = id[0];
        buffer[3] = id[1];
        buffer[4] = id[2];
        buffer[5] = id[3];
        buffer[6] = self.device_type;
        buffer[7] = self.battery;
        buffer[8] = data[0];
        buffer[9] = data[1];
        buffer[10] = data[2];
        buffer[11] = data[3];

        let checksum = buffer[1..12].iter().fold(0, |acc, cur| acc ^ cur);
        buffer[12] = checksum;

        buffer
    }

    pub fn id(&self) -> [u8; 4] {
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
