use image::Luma;
use qrcode::QrCode;

pub fn create_qr(path: &String, content: &String) {
    let code = QrCode::new(content.as_bytes()).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save(path.replace(".conf", ".png")).unwrap();
}
