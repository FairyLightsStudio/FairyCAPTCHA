mod r#gen {
    include!(concat!(env!("OUT_DIR"), "/pow_captcha_volo_gen.rs"));
}

pub use r#gen::pow_captcha_volo_gen::*;
