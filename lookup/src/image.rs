use crate::icon::ExtIcon;
use crate::icon::Graphics;


const TYPE_INT_RGB: u32 = 1;


struct Image {
    width: u32,
    height: u32,
    type_int_rgb: u32,
}

impl Image {
    fn to_image(&self, icon: Box<&dyn ExtIcon>) -> Image {
        let img = icon.get_lookup().lookup(self);

        if img != None {
            return img;
        }

        let width = icon.get_icon_width();
        let height = icon.get_icon_height();
        let type_int_rgb = TYPE_INT_RGB;

        let buf = BufferedImage { width, height, type_int_rgb };

        icon.paint_icon(None, buf.get_graphics(), 0, 0);
        
        Self {
            width: width,
            height: height,
            type_int_rgb: type_int_rgb,
        }
    }
}


struct BufferedImage {
    width: u32,
    height: u32,
    type_int_rgb: u32,
}

impl BufferedImage {
    fn get_graphics(&self) -> Graphics {
        Graphics
    }
}
