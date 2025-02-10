// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![feature(let_chains)]
use std::ffi::OsString;
use crate::session::KaniSession;

pub mod args;
pub mod args_toml;
mod assess;
mod call_cargo;
mod call_cbmc;
mod call_goto_cc;
mod call_goto_instrument;
mod call_goto_synthesizer;
pub mod call_single_file;
mod cbmc_output_parser;
mod cbmc_property_renderer;
mod concrete_playback;
mod coverage;
mod harness_runner;
mod list;
mod metadata;
mod project;
pub mod session;
mod util;
mod version;


#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum InvocationType {
    CargoKani(Vec<OsString>),
    Standalone,
}