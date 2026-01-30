extern crate zepub_mini_derive;

mod common;
mod cover;
mod epub;

pub mod parser;
pub mod path;

pub use crate::common::ContentItem;
pub use crate::common::ContentType;
pub use crate::common::DateTimeFormater;

pub mod prelude {
    pub use crate::common::IError;
    pub use crate::common::IResult;

    pub use crate::common::escape_xml;

    pub use crate::epub::builder::EpubBuilder;
    pub use crate::epub::common::LinkRel;
    pub use crate::epub::core::Direction;
    pub use crate::epub::core::EpubAssets;
    pub use crate::epub::core::EpubBook;
    pub use crate::epub::core::EpubHtml;
    pub use crate::epub::core::EpubLink;
    pub use crate::epub::core::EpubMetaData;
    pub use crate::epub::core::EpubNav;
    pub use crate::epub::writer::EpubWriter;

    pub mod appender {
        pub use crate::epub::appender::write_metadata;
    }
}

pub mod internal {
    pub use crate::common::get_css_content_url;
}
