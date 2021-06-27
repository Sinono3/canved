use crate::canvas::{BufColor, CanvasBuffer, RgbColor};
use image::io::Reader as ImageReader;
use std::io::{self as io, Cursor, Read};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ImageIo {
    Stdio,
    File(PathBuf),
}

impl ImageIo {
    pub fn read(&self) -> Result<CanvasBuffer, Box<dyn std::error::Error>> {
        let img = match self {
            ImageIo::Stdio => {
                let mut buf: Vec<u8> = Vec::new();

                io::stdin().lock().read_to_end(&mut buf)?;

                ImageReader::new(Cursor::new(&buf))
                    .with_guessed_format()?
                    .decode()?
            }
            ImageIo::File(path) => ImageReader::open(path)?.with_guessed_format()?.decode()?,
        };

        Ok(image_to_buffer(img.into_rgb8()))
    }

    pub fn write(
        &self,
        buffer: &CanvasBuffer,
        override_format: Option<ImageFormat>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output_img = image::DynamicImage::ImageRgb8(image::RgbImage::from_fn(
            buffer.width(),
            buffer.height(),
            |x, y| image::Rgb(RgbColor::from(buffer.get_pixel(x, y)).0),
        ));

        Ok(match self {
            ImageIo::Stdio => {
                let output_format = override_format.unwrap_or(ImageFormat::Png);
                let stdout = io::stdout();
                let mut lock = stdout.lock();

                output_img.write_to(&mut lock, output_format)?
            }
            ImageIo::File(path) => {
                if let Some(format) = override_format {
                    output_img.save_with_format(path, format.into())?
                } else {
                    output_img.save(path)?
                }
            }
        })
    }
}

impl FromStr for ImageIo {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(ImageIo::Stdio),
            _ => Ok(ImageIo::File(PathBuf::from(s))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    Ico,
    Bmp,
    Tga,
}

impl FromStr for ImageFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "png" => Ok(ImageFormat::Png),
            "jpeg" => Ok(ImageFormat::Jpeg),
            "gif" => Ok(ImageFormat::Gif),
            "ico" => Ok(ImageFormat::Ico),
            "bmp" => Ok(ImageFormat::Bmp),
            "tga" => Ok(ImageFormat::Tga),
            _ => Err("no match"),
        }
    }
}

impl Into<image::ImageOutputFormat> for ImageFormat {
    fn into(self) -> image::ImageOutputFormat {
        use image::ImageOutputFormat::*;

        match self {
            Self::Png => Png,
            Self::Jpeg => Jpeg(90), // TODO: select quality
            Self::Gif => Gif,
            Self::Ico => Ico,
            Self::Bmp => Bmp,
            Self::Tga => Tga,
        }
    }
}

impl Into<image::ImageFormat> for ImageFormat {
    fn into(self) -> image::ImageFormat {
        use image::ImageFormat::*;

        match self {
            Self::Png => Png,
            Self::Jpeg => Jpeg,
            Self::Gif => Gif,
            Self::Ico => Ico,
            Self::Bmp => Bmp,
            Self::Tga => Tga,
        }
    }
}

fn image_to_buffer(image: image::RgbImage) -> CanvasBuffer {
    let data: Vec<BufColor> = image
        .chunks(3)
        .map(|v| BufColor(((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32))
        .collect();

    CanvasBuffer::new(data, image.width(), image.height())
}
