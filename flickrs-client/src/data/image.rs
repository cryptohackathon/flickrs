use std::rc::Rc;

use druid::{Data, ImageBuf, Lens};

#[derive(Clone, Data, Lens)]
pub struct Image {
    pub link: String,
    pub buffer: Rc<ImageBuf>,
}
