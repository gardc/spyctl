// pub mod image_buffer_serde {
//     use image::{ImageBuffer, RgbaImage};
//     use serde::{Deserialize, Deserializer, Serialize, Serializer};

//     pub fn serialize<S>(
//         img: &RgbaImage,
//         serializer: S,
//     ) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let (width, height) = img.dimensions();
//         let raw_data = img.as_raw();
//         (width, height, raw_data).serialize(serializer)
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<RgbaImage, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let (width, height, raw_data): (u32, u32, Vec<u8>) =
//             Deserialize::deserialize(deserializer)?;
//         ImageBuffer::from_raw(width, height, raw_data)
//             .ok_or_else(|| serde::de::Error::custom("Failed to create ImageBuffer"))
//     }
// }
