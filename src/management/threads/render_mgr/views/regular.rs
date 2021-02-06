use crate::management::data::RenderFrame;
use crate::management::rendering::structures::{Frame, Line, Alignment, Cell};

pub async fn process(i: RenderFrame) -> Frame {



    let mut f = Frame {
        rows: vec![
            vec![
                Cell {
                    width: 0,
                    height: 0,
                    lines: vec![
                        Line {
                            align: Alignment::Middle,
                            length: 0,
                            content: "".to_string(),
                            title: None,
                            value: None,
                        }
                    ],
                }
            ]
        ],
    };



    f
}