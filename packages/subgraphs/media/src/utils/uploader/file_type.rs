use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum LibFileType {
    PDF,
    WORD,  // .doc or .docx
    EXCEL, // .xls or .xlsx
    IMAGE, // .jpg, .jpeg, .png, .gif
    TEXT,  // .txt
    CSV,
    PPT, // .ppt or .pptx
    ZIP,
    XML,
    JSON,
    OTHER,
}
