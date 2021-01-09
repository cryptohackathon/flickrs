use std::rc::Rc;

use druid::{im::Vector, ImageBuf};
use druid::{Data, Lens};

pub mod image;

use image::Image;

#[derive(Clone, Data, Lens, Default)]
pub struct State {
    pub images: Vector<Rc<Image>>,
}

impl State {
    pub fn load_images(&mut self) {
        let img = Image {
            link: "Default image".into(),
            buffer: ImageBuf::from_data(include_bytes!("../flick-rs.png"))
                .unwrap()
                .into(),
        };

        self.images.push_back(img.into());
    }
}
