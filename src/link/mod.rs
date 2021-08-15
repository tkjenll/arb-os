/*
 * Copyright 2020, Offchain Labs, Inc. All rights reserved.
 */

//! Provides types and utilities for linking together compiled mini programs

use crate::compile::{
    comma_list, CompileError, CompiledProgram, DebugInfo, ErrorSystem, FileInfo, GlobalVar,
    SourceFileMap, Type, TypeTree,
};
use crate::console::Color;
use crate::mavm::{AVMOpcode, Instruction, LabelId, Opcode, Value};
use crate::pos::{try_display_location, Location};
use crate::stringtable::StringId;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::io;
use xformcode::make_uninitialized_tuple;

use crate::compile::miniconstants::init_constant_table;
use std::path::Path;
pub use xformcode::{value_from_field_list, TupleTree, TUPLE_SIZE};

mod optimize;
mod striplabels;
mod xformcode;

#[derive(Clone, Serialize, Deserialize)]
pub struct SerializableTypeTree {
    inner: BTreeMap<String, (Type, String)>,
}

impl SerializableTypeTree {
    pub fn from_type_tree(tree: TypeTree) -> Self {
        let mut inner = BTreeMap::new();
        for ((path, id), tipe) in tree.into_iter() {
            inner.insert(format!("{}, {}", comma_list(&path), id), tipe);
        }
        Self { inner }
    }
    pub fn into_type_tree(self) -> TypeTree {
        let mut type_tree = HashMap::new();
        for (path, tipe) in self.inner.into_iter() {
            let mut x: Vec<_> = path.split(", ").map(|val| val.to_string()).collect();
            let id = x
                .pop()
                .map(|id| id.parse::<usize>())
                .expect("empty list")
                .expect("failed to parse");
            type_tree.insert((x, id), tipe);
        }
        type_tree
    }
}

/// Represents a mini program that has gone through the post-link compilation step.
///
/// This is typically constructed via the `postlink_compile` function.
#[derive(Serialize, Deserialize)]
pub struct LinkedProgram {
    #[serde(default)]
    pub arbos_version: u64,
    pub code: Vec<Instruction<AVMOpcode>>,
    pub static_val: Value,
    pub globals: Vec<GlobalVar>,
    // #[serde(default)]
    pub file_info_chart: BTreeMap<u64, FileInfo>,
    pub type_tree: SerializableTypeTree,
}

impl LinkedProgram {
    /// Serializes self to the format specified by the format argument, with a default of json for
    /// None. The output is written to a dynamically dispatched implementor of `std::io::Write`,
    /// specified by the output argument.
    pub fn to_output(&self, output: &mut dyn io::Write, format: Option<&str>) {
        match format {
            Some("pretty") => {
                writeln!(output, "static: {}", self.static_val).unwrap();
                for (idx, insn) in self.code.iter().enumerate() {
                    writeln!(
                        output,
                        "{:05}:  {} \t\t {}",
                        idx,
                        insn,
                        try_display_location(
                            insn.debug_info.location,
                            &self.file_info_chart,
                            false
                        )
                    )
                    .unwrap();
                }
            }
            None | Some("json") => match serde_json::to_string(self) {
                Ok(prog_str) => {
                    writeln!(output, "{}", prog_str).unwrap();
                }
                Err(e) => {
                    eprintln!("failure");
                    writeln!(output, "json serialization error: {:?}", e).unwrap();
                }
            },
            Some("bincode") => match bincode::serialize(self) {
                Ok(encoded) => {
                    if let Err(e) = output.write_all(&encoded) {
                        writeln!(output, "bincode write error: {:?}", e).unwrap();
                    }
                }
                Err(e) => {
                    writeln!(output, "bincode serialization error: {:?}", e).unwrap();
                }
            },
            Some(weird_value) => {
                writeln!(output, "invalid format: {}", weird_value).unwrap();
            }
        }
    }
}

/// Represents an import generated by a `use` statement.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Import {
    /// Module path, relative to logical program root.
    pub path: Vec<String>,
    /// Name of `Type` or function to be imported.
    pub name: String,
    /// Unique global id this import refers to
    pub unique_id: LabelId,
    /// `StringId` of the use-statement from parsing according to the containing module's `StringTable`
    pub id: Option<StringId>,
    /// Location of the use-statement in code
    pub location: Option<Location>,
}

impl Import {
    pub fn new(
        path: Vec<String>,
        name: String,
        id: Option<StringId>,
        location: Option<Location>,
    ) -> Self {
        let unique_id = Import::unique_id(&path, &name);
        Import {
            path,
            name,
            unique_id,
            id,
            location,
        }
    }

    pub fn loc(&self) -> Vec<Location> {
        self.location.into_iter().collect()
    }

    pub fn new_builtin(virtual_file: &str, name: &str) -> Self {
        let path = vec!["core".to_string(), virtual_file.to_string()];
        let name = name.to_string();
        let unique_id = Import::unique_id(&path, &name);
        Import {
            path,
            name,
            unique_id,
            id: None,
            location: None,
        }
    }

    pub fn unique_id(path: &Vec<String>, name: &String) -> LabelId {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        name.hash(&mut hasher);
        hasher.finish()
    }
}

/// Converts a linked `CompiledProgram` into a `LinkedProgram` by fixing non-forward jumps,
/// converting wide tuples to nested tuples, performing code optimizations, converting the jump
/// table to a static value, and combining the file info chart with the associated argument.
pub fn postlink_compile(
    program: CompiledProgram,
    mut file_info_chart: BTreeMap<u64, FileInfo>,
    _error_system: &mut ErrorSystem,
    test_mode: bool,
    debug: bool,
) -> Result<LinkedProgram, CompileError> {
    let consider_debug_printing = |code: &Vec<Instruction>, did_print: bool, phase: &str| {
        if debug {
            println!("========== {} ==========", phase);
            for (idx, insn) in code.iter().enumerate() {
                println!(
                    "{}  {}",
                    Color::grey(format!("{:04}", idx)),
                    insn.pretty_print(Color::PINK)
                );
            }
        } else if did_print {
            println!("========== {} ==========", phase);
            for (idx, insn) in code.iter().enumerate() {
                if insn.debug_info.attributes.codegen_print {
                    println!(
                        "{}  {}",
                        Color::grey(format!("{:04}", idx)),
                        insn.pretty_print(Color::PINK)
                    );
                }
            }
        }
    };

    let mut did_print = false;

    if debug {
        println!("========== after initial linking ===========");
        for (idx, insn) in program.code.iter().enumerate() {
            println!(
                "{}  {}",
                Color::grey(format!("{:04}", idx)),
                insn.pretty_print(Color::PINK)
            );
        }
    } else {
        for (idx, insn) in program.code.iter().enumerate() {
            if insn.debug_info.attributes.codegen_print {
                println!(
                    "{}  {}",
                    Color::grey(format!("{:04}", idx)),
                    insn.pretty_print(Color::PINK)
                );
                did_print = true;
            }
        }
    }
    let (code_2, jump_table) =
        striplabels::fix_nonforward_labels(&program.code, program.globals.len() - 1);
    //consider_debug_printing(&code_2, did_print, "after fix_backward_labels");

    let code_3 = xformcode::fix_tuple_size(&code_2, program.globals.len())?;
    //consider_debug_printing(&code_3, did_print, "after fix_tuple_size");

    let code_4 = optimize::peephole(&code_3);
    //consider_debug_printing(&code_4, did_print, "after peephole optimization");

    let (mut code_5, jump_table_final) = striplabels::strip_labels(code_4, &jump_table)?;
    let jump_table_value = xformcode::jump_table_to_value(jump_table_final);

    hardcode_jump_table_into_register(&mut code_5, &jump_table_value, test_mode);
    let code_final: Vec<_> = code_5
        .into_iter()
        .map(|insn| {
            if let Opcode::AVMOpcode(inner) = insn.opcode {
                Ok(Instruction::new(inner, insn.immediate, insn.debug_info))
            } else {
                Err(CompileError::new(
                    String::from("Postlink error"),
                    format!("In final output encountered virtual opcode {}", insn.opcode),
                    insn.debug_info.location.into_iter().collect(),
                ))
            }
        })
        .collect::<Result<Vec<_>, CompileError>>()?;

    if debug {
        println!("============ after strip_labels =============");
        println!("static: {}", jump_table_value);
        for (idx, insn) in code_final.iter().enumerate() {
            println!("{:04}  {}", idx, insn);
        }
        println!("============ after full compile/link =============");
    }

    file_info_chart.extend(program.file_info_chart.clone());

    Ok(LinkedProgram {
        arbos_version: init_constant_table(Some(Path::new("arb_os/constants.json")))
            .unwrap()
            .get("ArbosVersionNumber")
            .unwrap()
            .clone()
            .trim_to_u64(),
        code: code_final,
        static_val: Value::none(),
        globals: program.globals.clone(),
        file_info_chart,
        type_tree: SerializableTypeTree::from_type_tree(program.type_tree),
    })
}

fn hardcode_jump_table_into_register(
    code: &mut Vec<Instruction>,
    jump_table: &Value,
    test_mode: bool,
) {
    let offset = if test_mode { 1 } else { 2 };
    let old_imm = code[offset].clone().immediate.unwrap();
    code[offset] = Instruction::from_opcode_imm(
        code[offset].opcode,
        old_imm.replace_last_none(jump_table),
        code[offset].debug_info,
    );
}

/// Combines the `CompiledProgram`s in progs_in into a single `CompiledProgram` with offsets adjusted
/// to avoid collisions and auto-linked programs added.
pub fn link(
    progs_in: Vec<CompiledProgram>,
    globals: Vec<GlobalVar>,
    test_mode: bool,
) -> CompiledProgram {
    let progs = progs_in.to_vec();
    let type_tree = progs[0].type_tree.clone();
    let mut insns_so_far: usize = 3; // leave 2 insns of space at beginning for initialization
    let mut int_offsets = Vec::new();
    let mut merged_source_file_map = SourceFileMap::new_empty();
    let mut merged_file_info_chart = HashMap::new();

    for prog in &progs {
        merged_source_file_map.push(
            prog.code.len(),
            match &prog.source_file_map {
                Some(sfm) => sfm.get(0),
                None => "".to_string(),
            },
        );
        int_offsets.push(insns_so_far);
        insns_so_far += prog.code.len();
    }

    let mut relocated_progs = Vec::new();
    let mut func_offset: usize = 0;
    for (i, prog) in progs.into_iter().enumerate() {
        merged_file_info_chart.extend(prog.file_info_chart.clone());

        let source_file_map = prog.source_file_map.clone();
        let (relocated_prog, new_func_offset) =
            prog.relocate(int_offsets[i], func_offset, source_file_map);

        relocated_progs.push(relocated_prog);
        func_offset = new_func_offset + 1;
    }

    /*global_num_limit.push(GlobalVar::new(
        usize::MAX,
        "_jump_table".to_string(),
        Type::Any,
        None,
    ));*/

    // Initialize globals or allow jump table retrieval
    let mut linked_code = if test_mode {
        vec![
            Instruction::from_opcode_imm(
                Opcode::AVMOpcode(AVMOpcode::Noop),
                Value::none(),
                DebugInfo::default(),
            ),
            Instruction::from_opcode_imm(
                Opcode::AVMOpcode(AVMOpcode::Noop),
                make_uninitialized_tuple(globals.len()),
                DebugInfo::default(),
            ),
            Instruction::from_opcode(Opcode::AVMOpcode(AVMOpcode::Rset), DebugInfo::default()),
        ]
    } else {
        vec![
            Instruction::from_opcode(Opcode::AVMOpcode(AVMOpcode::Rpush), DebugInfo::default()),
            Instruction::from_opcode_imm(
                Opcode::AVMOpcode(AVMOpcode::Noop),
                Value::none(),
                DebugInfo::default(),
            ),
            Instruction::from_opcode_imm(
                Opcode::AVMOpcode(AVMOpcode::Rset),
                make_uninitialized_tuple(globals.len()),
                DebugInfo::default(),
            ),
        ]
    };

    if globals
        .iter()
        .any(|x| x.debug_info.attributes.codegen_print)
    {
        for curr in &mut linked_code {
            curr.debug_info.attributes.codegen_print = true;
        }
    }

    for mut rel_prog in relocated_progs {
        linked_code.append(&mut rel_prog.code);
    }

    CompiledProgram::new(
        linked_code,
        globals,
        Some(merged_source_file_map),
        merged_file_info_chart,
        type_tree,
    )
}
