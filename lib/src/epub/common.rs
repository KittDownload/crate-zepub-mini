crate::cache_enum! {
    #[derive(PartialEq, Clone)]
    pub enum LinkRel {
        CSS,
        OTHER(String),
    }
}

pub static EPUB: &str = "OEBPS/";
pub static EPUB3: &str = "EPUB/";
pub static TOC: &str = "OEBPS/toc.ncx";
pub static NAV: &str = "OEBPS/nav.xhtml";
pub static COVER: &str = "OEBPS/cover.xhtml";
pub static OPF: &str = "OEBPS/content.opf";

impl std::fmt::Display for LinkRel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CSS => {
                    "stylesheet"
                }
                Self::OTHER(h) => {
                    &h
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    // use super::do_time_format;
    // use super::do_time_format2;

    // fn assert_value(left: &str, right: String) {
    //     if left != right {
    //         panic!("left= [{left}] right=[{right}] ");
    //     }
    // }

    #[test]
    fn test_time_format() {}
}
