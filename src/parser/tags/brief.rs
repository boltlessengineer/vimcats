use chumsky::{prelude::just, select, Parser};

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Brief {
    pub desc: Vec<String>,
}

impl_parse!(Brief, {
    select! {
        TagType::Comment(x) => x,
    }
    .repeated()
    .delimited_by(just(TagType::BriefStart), just(TagType::BriefEnd))
    .map(|desc| Self { desc })
});
