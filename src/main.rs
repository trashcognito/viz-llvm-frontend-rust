#[macro_use]
extern crate clap;
use clap::{App, Arg};
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::targets::{Target, InitializationConfig, TargetMachine, TargetTriple, RelocMode, CodeModel};
use inkwell::{OptimizationLevel, AddressSpace};
enum VmstructIndices {
    PC=0,//   uint16_t pc;
    A,//   uint16_t regA;
    B,//   uint16_t regB;
    C,//   uint16_t regC;
    X,//   uint16_t regX;
    Y,//   uint16_t regY;
    Z,//   uint16_t regZ;
    F,//   uint16_t regF;
    H,//   uint16_t regH;
    SP,//   uint8_t sp;
    FIXED0,//   uint16_t fixed0;
    FIXED1,//   uint16_t fixed1;
    ADDRSPACE//   uint16_t *addrspace;
}
fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("Input")
            .help("Viz file to compile")
            .index(1)
            .required(true)
        )
        .arg(Arg::with_name("Output")
            .help("Output file name")
            .short("o")
            .long("output")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("Target")
            .help("Target triplet to build for")
            .long("target")
            .takes_value(true)
            .required(false)
        )
        .arg(Arg::with_name("Optimization Level")
            .help("Optimization level for application")
            .short("O")
            .possible_values(&["0","1","2","3"])
            .default_value("2"))
        .arg(Arg::with_name("Features")
            .help("CPU Features for application")
            .long("features")
            .require_equals(true)
            .takes_value(true)
            .required(false))
        .get_matches();
    //boilerplate modified from https://benkonz.github.io/building-a-brainfuck-compiler-with-rust-and-llvm/
    let context = Context::create();
    let module = context.create_module("viz_rust");
    let builder = context.create_builder();
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_type, Some(Linkage::External));
    let basic_block = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(basic_block);
    //compile goes here
    Target::initialize_all(&InitializationConfig::default());
    let target_triple = match matches.value_of("Target") {
        None => {TargetMachine::get_default_triple()}
        Some(t) => {TargetTriple::create(t)}
    };
    //TODO: check if LLVM CPU string and target triplet are interchangable
    let cpu = match matches.value_of("Target") {
        None => {TargetMachine::get_host_cpu_name().to_string()}
        Some(t) => {String::from(t)}
    };
    let features = match matches.value_of("Features") {
        None => {TargetMachine::get_host_cpu_features().to_string()}
        Some(f) => {String::from(f)}
    };
    let optimization_level = match matches.value_of("Optimization Level") {
        None => {OptimizationLevel::Default}
        Some(o) => {match o {
            "0" => OptimizationLevel::None,
            "1" => OptimizationLevel::Less,
            "2" => OptimizationLevel::Default,
            "3" | _ => OptimizationLevel::Aggressive
        }}
    };
    let target = Target::from_triple(&target_triple)
        .expect("Could not get LLVM Target from triple!");
    let target_machine = target
        .create_target_machine(
            &target_triple,
            cpu.as_str(),
            features.as_str(),
            optimization_level,
            RelocMode::Default,
            CodeModel::Default
        ).expect("Could not create target machine!");
    let i16_type = context.i16_type();
    let i16_ptr_type = i16_type.ptr_type(AddressSpace::Generic);
    //typedef struct virtualmachine {
    //   uint16_t pc;
    //   uint16_t regA;
    //   uint16_t regB;
    //   uint16_t regC;
    //   uint16_t regX;
    //   uint16_t regY;
    //   uint16_t regZ;
    //   uint16_t regF;
    //   uint16_t regH;
    //   uint8_t sp;
    //   uint16_t fixed0;
    //   uint16_t fixed1;
    //   uint16_t *addrspace;
    // } virtualmachine_t;
    let i8_type = context.i8_type();
    let vmstruct_type = context.struct_type(&[
        i16_type.into(),//   uint16_t pc;
        i16_type.into(),//   uint16_t regA;
        i16_type.into(),//   uint16_t regB;
        i16_type.into(),//   uint16_t regC;
        i16_type.into(),//   uint16_t regX;
        i16_type.into(),//   uint16_t regY;
        i16_type.into(),//   uint16_t regZ;
        i16_type.into(),//   uint16_t regF;
        i16_type.into(),//   uint16_t regH;
        i8_type.into(),//   uint8_t sp;
        i16_type.into(),//   uint16_t fixed0;
        i16_type.into(),//   uint16_t fixed1;
        i16_type.into()//   uint16_t *addrspace;
    ], false);
    let vmstruct = builder.build_alloca(vmstruct_type, "vmstruct");
    //TODO: solve vsm text/data separation problem
    builder.build_return(Some(&i32_type.const_zero()));
}
