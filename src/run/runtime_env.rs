/*
 * Copyright 2020, Offchain Labs, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::mavm::{Value};


pub struct RuntimeEnvironment {
    pub l1_inbox: Value,
    pub logs: Vec<Value>,
}

impl RuntimeEnvironment {
    pub fn new() -> Self {
        RuntimeEnvironment{ 
            l1_inbox: Value::none(),
            logs: Vec::new(),
        }
    }

    pub fn insert_message(&mut self, msg: Value) {
        self.l1_inbox = Value::Tuple(vec![self.l1_inbox.clone(), msg]);
    }

    pub fn get_inbox(&mut self) -> Value {
        let ret = self.l1_inbox.clone();
        self.l1_inbox = Value::none();
        ret
    }

    pub fn push_log(&mut self, log_item: Value) {
        self.logs.push(log_item);
    }

    pub fn get_all_logs(&self) -> &Vec<Value> {
        &self.logs
    }
}