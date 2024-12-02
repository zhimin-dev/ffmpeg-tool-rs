use crate::repeat::com::RepeatCheck;

struct VideoCheck {
    one_url: String,
    two_url: String,
    check_dir: String,
}

impl RepeatCheck for VideoCheck {
    fn check(&self) -> bool {
        return false;
    }
}
