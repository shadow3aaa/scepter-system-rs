use std::vec; // Removed io::Cursor

use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

use super::llm_adapters::Message;

pub fn divergence(concept: &Concept) -> Vec<Message> {
    vec![
        Message::system(
            r"
你是一名格式专家，所有输出必须严格遵循以下xml标记格式，不要包含任何额外文字，说明或标记。
严格符合xml格式，不要输出不符合格式的标签。
常见不符合规范的格式，严禁这样输出！：
- <tag/>: 应为</tag>
- </tag/>: 应为</tag>

输出示例：
<concepts>
    <concept>
        <core>
            核心概念1
        </core>
        <clarification>
            概念的明确1(让概念更加限定性)
        </clarification>
    </concept>
    <concept>
        <core>
            核心概念2
        </core>
        <clarification>
            概念的明确2(让概念更加限定性)
        </clarification>
    </concept>
</concepts>
",
        ),
        Message::user(format!(
            r"
根据输入概念信息生成几个相关的子领域内容。

遵循以下要求：

- 回答语言使用用户输入概念的语言的主导为主导语言。
- 内容少而精，避免冗长的描述。
- 生成的内容应该是概念接近的子领域，避免生成与概念无关或者关系太远的内容。
- 生成的内容应该是尽量发散到概念的不同方面，避免生成相对重复的内容。

>>>START OF INPUT
<concept>
    <core>
        {}
    </core>
    <clarification>
        {}
    </clarification>
</concept>
>>>END OF INPUT
",
            concept.core, concept.clarification
        )),
    ]
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Concept {
    pub core: String,
    pub clarification: String,
}

pub struct ConceptStreamParser {
    buffer: String,
}

#[derive(Debug, PartialEq, Eq)]
enum TagState {
    None,
    Unknown(String),
    InConcepts,
    InConcept,
    InCore,
    InClarification,
}

impl ConceptStreamParser {
    pub const fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn push_chunk(&mut self, chunk: &str) -> Vec<Concept> {
        self.buffer.push_str(chunk);
        self.parse()
    }

    fn parse(&self) -> Vec<Concept> {
        let mut reader = Reader::from_str(&self.buffer);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut tag_state = TagState::None;
        let mut output = Vec::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) => match tag_state {
                    TagState::Unknown(_) => (),
                    _ => match e.name().into_inner() {
                        b"concepts" => {
                            tag_state = TagState::InConcepts;
                        }
                        b"concept" => {
                            tag_state = TagState::InConcept;
                            output.push(Concept::default());
                        }
                        b"core" => {
                            tag_state = TagState::InCore;
                        }
                        b"clarification" => {
                            tag_state = TagState::InClarification;
                        }
                        _ => {
                            if tag_state == TagState::None {
                                tag_state = TagState::Unknown(
                                    String::from_utf8_lossy(e.name().into_inner()).to_string(),
                                );
                            }
                        }
                    },
                },
                Event::End(ref e) => {
                    if let TagState::Unknown(ref tag) = tag_state {
                        if *tag == String::from_utf8_lossy(e.name().into_inner()) {
                            tag_state = TagState::None;
                        }
                    } else {
                        match e.name().into_inner() {
                            b"concepts" => {
                                assert_eq!(tag_state, TagState::InConcepts);
                                tag_state = TagState::None;
                            }
                            b"concept" => {
                                assert!(tag_state == TagState::InConcept);
                                tag_state = TagState::InConcepts;
                            }
                            b"core" => {
                                assert!(tag_state == TagState::InCore);
                                tag_state = TagState::InConcept;
                            }
                            b"clarification" => {
                                assert!(tag_state == TagState::InClarification);
                                tag_state = TagState::InConcept;
                            }
                            _ => (),
                        }
                    }
                }
                Event::Text(ref e) => match tag_state {
                    TagState::InCore => {
                        let text = e.unescape().unwrap();
                        output.last_mut().unwrap().core.push_str(&text);
                    }
                    TagState::InClarification => {
                        let text = e.unescape().unwrap();
                        output.last_mut().unwrap().clarification.push_str(&text);
                    }
                    _ => (),
                },
                Event::Eof => break,
                _ => (),
            }

            buf.clear();
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concept_stream_parser_basic() {
        let mut parser = ConceptStreamParser::new();

        // 模拟流式分块输入
        parser.push_chunk("<concepts>");
        parser.push_chunk("<concept><core>逻辑");
        parser.push_chunk("</core><clarification>研究思维规律");
        parser.push_chunk("</clarification></concept>");
        let concepts = parser.push_chunk("</concepts>");

        assert_eq!(concepts.len(), 1);
        assert_eq!(concepts[0].core, "逻辑");
        assert_eq!(concepts[0].clarification, "研究思维规律");
    }

    #[test]
    fn test_concept_stream_parser_multiple_chunks() {
        let mut parser = ConceptStreamParser::new();

        parser.push_chunk("<concepts>");
        parser.push_chunk("<concept><core>人工智能</core>");
        parser.push_chunk("<clarification>研究智能系统的构建</clarification></concept>");
        parser.push_chunk("<concept><core>数学</core><clarification>");
        parser.push_chunk("研究数量和结构</clarification></concept>");
        let concepts = parser.push_chunk("</concepts>");

        assert_eq!(concepts.len(), 2);
        assert_eq!(concepts[0].core, "人工智能");
        assert_eq!(concepts[0].clarification, "研究智能系统的构建");
        assert_eq!(concepts[1].core, "数学");
        assert_eq!(concepts[1].clarification, "研究数量和结构");
    }

    #[test]
    fn test_partial_incomplete_tag_does_not_panic() {
        let mut parser = ConceptStreamParser::new();
        parser.push_chunk("<concepts>");
        let concepts = parser.push_chunk("<concept><core>未闭合");

        assert_eq!(concepts.len(), 1);
        assert_eq!(concepts[0].core, "未闭合");

        parser.push_chunk("</core><clarification>解释</clarification>");
        let concepts = parser.push_chunk("</concepts>");
        assert_eq!(concepts[0].core, "未闭合");
        assert_eq!(concepts[0].clarification, "解释");
    }

    #[test]
    fn test_noise_tags() {
        let mut parser = ConceptStreamParser::new();
        parser.push_chunk("<noise><concept>应该被忽略</concept></noise>");
        parser.push_chunk("<concepts>");
        parser.push_chunk("<concept><core>概念</core><clarification>描述</clarification>");
        parser.push_chunk("<noise>噪声数据</noise></concept>");
        let concepts = parser.push_chunk("</concepts>");

        assert_eq!(concepts.len(), 1);
        assert_eq!(concepts[0].core, "概念");
        assert_eq!(concepts[0].clarification, "描述");
    }
}
