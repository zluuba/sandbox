struct BufferedImage;
struct Image;

impl Image {
    fn to_image(&self, icon: <dyn ExtIcon>) -> Option<Image> {
        let img = icon.get_lookup().lookup(&self);

        if img != None {
            return img;
        }

        let buf = BufferedImage {
            icon.get_icon_width(),
            icon.get_icon_height(),
            BufferedImage.TYPE_INT_RGB,
        };

        icon.paint_icon(None, buf.get_graphics(), 0, 0);
        
        buf
    }
}