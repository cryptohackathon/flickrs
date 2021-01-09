use druid::{
    widget::{CrossAxisAlignment, Flex, Label, List, ListIter},
    ImageBuf, Widget, WidgetExt,
};

use crate::data::{image::Image, State};

pub trait ImageIter {
    fn link(&self) -> String;
    fn buffer(&self) -> &ImageBuf;
}

impl ListIter<Image> for State {
    fn for_each(&self, mut cb: impl FnMut(&Image, usize)) {
        ListIter::for_each(&self.images, |img, index| {
            let img_row = Image {
                link: img.link.to_owned(),
                buffer: img.buffer.to_owned(),
            };
            cb(&img_row, index);
        })
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Image, usize)) {
        ListIter::for_each_mut(&mut self.images, |img, index| {
            let mut img_row = Image {
                link: img.link.to_owned(),
                buffer: img.buffer.to_owned(),
            };
            cb(&mut img_row, index);
        })
    }

    fn data_len(&self) -> usize {
        self.images.len()
    }
}

pub fn make_list() -> impl Widget<State> {
    List::new(|| {
        let mut img_col = Flex::column();
        img_col.add_child(
            druid::widget::Image::new(
                ImageBuf::from_data(include_bytes!("../placeholder.png")).unwrap(),
            )
            .expand_width()
            .lens(Image::buffer),
        );

        let mut info_col = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
        let mut lbl = Label::raw();
        lbl.set_text_size(20.0);

        info_col.add_child(lbl.lens(Image::link));

        druid::widget::Split::columns(img_col, info_col).split_point(0.7)
    })
}
