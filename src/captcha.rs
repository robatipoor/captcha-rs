use captcha::Captcha;

pub fn generate_captcha(){
    Captcha::new()
    .add_chars(5)
    .apply_filter(filters::Noise::new(0.1))
    .view(220, 120)
    .save(Path::new("/tmp/captcha.png"))
    .expect("save failed");
}