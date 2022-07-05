//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore, Feeder};
use crate::elem_function::Function;
use crate::elem_pipeline::Pipeline;
use crate::elem_setvars::SetVariables;
use crate::ListElem;
use crate::scanner::scanner_blank_part;

pub struct Script {
    pub list: Vec<Box<dyn ListElem>>,
    pub text: String,
    pub procnum: usize,
}

impl Script {
    pub fn exec(&mut self, conf: &mut ShellCore) {
        self.list.iter_mut()
            .for_each(|p| p.exec(conf));
    }

    pub fn new() -> Script{
        Script {
            list: vec!(),
            text: "".to_string(),
            procnum: 0,
        }
    }

    pub fn parse(text: &mut Feeder, conf: &mut ShellCore, next: bool) -> Option<Script> {
        if text.len() == 0 {
            return None;
        };
    
        if text.nth(0) == ')' {
            eprintln!("Unexpected symbol: {}", text.consume(text.len()).trim_end());
            conf.vars.insert("?".to_string(), "2".to_string());
            return None;
        }

        let mut ans = Script::new();
        let mut is_function = false;
    
        let mut procnum = 0;
        loop {
            let pos = scanner_blank_part(text, 0);
            ans.text += &text.consume(pos);

            if let Some(f) = Function::parse(text, conf)            {
                ans.text += &f.text;
                let body = f.body.get_text();
                conf.functions.insert(f.name, body);
                is_function = true;
            }else if let Some(result) = SetVariables::parse(text, conf) {
                ans.text += &result.text;
                ans.list.push(Box::new(result));
            }else if let Some(result) = Pipeline::parse(text, conf) {
                ans.text += &result.text;
                ans.list.push(Box::new(result));
                procnum += 1;
            }
            else {break}

            if text.len() == 0 || text.nth(0) == ')' || !next {
                break;
            }
        }
    
        if ans.text.len() > 0 || is_function {
            ans.procnum = procnum;
            Some(ans)
        }else{
            eprintln!("Unknown phrase");
            conf.vars.insert("?".to_string(), "1".to_string());
            text.consume(text.len());
            None
        }
    }
}
