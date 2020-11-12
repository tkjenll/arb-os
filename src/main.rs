/*
 * Copyright 2020, Offchain Labs, Inc. All rights reserved.
 */

#![allow(unused_parens)]

use compile::{compile_from_file, CompileError};
use contracttemplates::generate_contract_template_file_or_die;
use link::{link, postlink_compile};
use mavm::Value;
use run::{profile_gen_from_file, replay_from_testlog_file, run_from_file, RuntimeEnvironment};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::run::ProfilerMode;
use crate::uint256::Uint256;
use clap::Clap;

mod compile;
mod contracttemplates;
mod evm;
mod link;
mod mavm;
#[cfg(test)]
mod minitests;
pub mod pos;
mod run;
mod stringtable;
mod uint256;

#[derive(Clap, Debug)]
struct CompileStruct {
    input: Vec<String>,
    #[clap(short, long)]
    debug_mode: bool,
    #[clap(short, long)]
    typecheck: bool,
    #[clap(short, long)]
    output: Option<String>,
    #[clap(short, long)]
    compile_only: bool,
    #[clap(short, long)]
    format: Option<String>,
    #[clap(short, long)]
    module: bool,
}

#[derive(Clap, Debug)]
struct RunStruct {
    input: String,
    #[clap(short, long)]
    debug: bool,
}

#[derive(Clap, Debug)]
struct EvmDebug {
    #[clap(short, long)]
    debug: bool,
    #[clap(short, long)]
    profiler: bool,
}

#[derive(Clap, Debug)]
struct Replay {
    input: String,
    #[clap(short, long)]
    debug: bool,
    #[clap(short, long)]
    profiler: ProfilerMode,
    #[clap(short, long)]
    trace: Option<String>,
}

#[derive(Clap, Debug)]
struct Profiler {
    input: String,
    #[clap(short, long)]
    mode: ProfilerMode,
}

#[derive(Clap, Debug)]
enum Args {
    Compile(CompileStruct),
    Run(RunStruct),
    EvmDebug(EvmDebug),
    Profiler(Profiler),
    Replay(Replay),
    MakeTestLogs,
    MakeBenchmarks,
    MakeTemplates,
    EvmTests,
}

fn main() -> Result<(), CompileError> {
    let matches = Args::parse();

    match matches {
        Args::Compile(compile) => {
            let debug_mode = compile.debug_mode;
            let typecheck = compile.typecheck;
            let mut output = get_output(compile.output.as_deref()).unwrap();
            let filenames: Vec<_> = compile.input.clone();
            let mut file_name_chart = HashMap::new();
            if compile.compile_only {
                let filename = &filenames[0];
                let path = Path::new(filename);
                match compile_from_file(path, &mut file_name_chart, debug_mode) {
                    Ok(mut compiled_program) => {
                        compiled_program.iter_mut().for_each(|prog| {
                            prog.file_name_chart.extend(file_name_chart.clone());
                            prog.to_output(&mut *output, compile.format.as_deref());
                        });
                    }
                    Err(e) => {
                        println!("Compilation error: {:?}\nIn file: {}", e, filename);
                        return Err(e);
                    }
                }
            } else {
                let mut compiled_progs = Vec::new();
                for filename in &filenames {
                    let path = Path::new(filename);
                    match compile_from_file(path, &mut file_name_chart, debug_mode) {
                        Ok(compiled_program) => {
                            compiled_program.into_iter().for_each(|prog| {
                                file_name_chart.extend(prog.file_name_chart.clone());
                                compiled_progs.push(prog)
                            });
                        }
                        Err(e) => {
                            println!(
                                "Compilation error: {}\nIn file: {}",
                                e,
                                e.location
                                    .map(|loc| file_name_chart
                                        .get(&loc.file_id)
                                        .unwrap_or(&loc.file_id.to_string())
                                        .clone())
                                    .unwrap_or("Unknown".to_string())
                            );
                            return Err(e);
                        }
                    }
                }

                let is_module = compile.module;
                match link(&compiled_progs, is_module, Some(Value::none()), typecheck) {
                    Ok(linked_prog) => {
                        match postlink_compile(
                            linked_prog,
                            is_module,
                            Vec::new(),
                            file_name_chart,
                            debug_mode,
                        ) {
                            Ok(completed_program) => {
                                completed_program
                                    .to_output(&mut *output, compile.format.as_deref());
                            }
                            Err(e) => {
                                println!("Linking error: {}", e);
                                return Err(e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Linking error: {}", e);
                        return Err(e);
                    }
                }
            }
        }

        Args::Run(run) => {
            let filename = run.input;
            let debug = run.debug;
            let path = Path::new(&filename);
            let env = RuntimeEnvironment::new(Uint256::from_usize(1111));
            match run_from_file(path, Vec::new(), env, debug) {
                Ok(logs) => {
                    println!("Logs: {:?}", logs);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        Args::EvmDebug(evm_debug) => {
            let debug = evm_debug.debug;
            let profile = evm_debug.profiler;
            //let _ = evm::evm_xcontract_call_with_constructors(None, debug, profile);
            let _ = evm::evm_xcontract_call_using_batch(None, debug, profile);
        }

        Args::Profiler(path) => {
            let input = path.input;
            profile_gen_from_file(
                input.as_ref(),
                Vec::new(),
                RuntimeEnvironment::new(Uint256::from_usize(1111)),
                path.mode,
            );
        }

        Args::Replay(replay) => {
            let path = replay.input.as_str();
            let debug = replay.debug;
            let profiler = replay.profiler;
            let trace_file = replay.trace.as_deref();

            if let Err(e) = replay_from_testlog_file(path, true, debug, profiler, trace_file) {
                panic!("Error reading from {}: {}", path, e);
            }
        }

        Args::MakeTestLogs => {
            evm::make_logs_for_all_arbos_tests();
        }

        Args::MakeBenchmarks => {
            evm::benchmarks::make_benchmarks();
        }

        Args::MakeTemplates => {
            let path = Path::new("arb_os/contractTemplates.mini");
            generate_contract_template_file_or_die(path);
        }

        Args::EvmTests => {
            let path = Path::new("evm-tests/VMTests/vmArithmeticTest");
            let _ = evm::evmtest::run_evm_tests(path, None).unwrap();
        }
    }

    Ok(())
}

fn get_output(output_filename: Option<&str>) -> Result<Box<dyn io::Write>, io::Error> {
    match output_filename {
        Some(ref path) => File::create(path).map(|f| Box::new(f) as Box<dyn io::Write>),
        None => Ok(Box::new(io::stdout())),
    }
}
