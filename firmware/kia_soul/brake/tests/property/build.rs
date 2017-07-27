extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    gcc::Config::new()
        .flag("-w")
        .define("KIA_SOUL", Some("ON"))
        .include("include")
        .include("../../include")
        .include("../../../../common/include")
        .include("../../../../common/testing/mocks/")
        .include("../../../../common/libs/can")
        .include("../../../../common/libs/dac")
        .include("../../../../common/libs/pid")
        .include("../../../../../api/include")
        .file("../../../../common/testing/mocks/Arduino_mock.cpp")
        .file("../../../../common/testing/mocks/DAC_MCP49xx_mock.cpp")
        .file("../../../../common/testing/mocks/mcp_can_mock.cpp")
        .file("../../../../common/libs/can/oscc_can.cpp")
        .file("../../../../common/libs/dac/oscc_dac.cpp")
        .file("../../src/communications.cpp")
        .file("../../src/brake_control.cpp")
        .file("../../src/globals.cpp")
        .file("../../src/master_cylinder.cpp")
        .file("../../src/helper.cpp")
        .file("../../src/accumulator.cpp")
        .compile("libbrake_test.a");

    let out_dir = env::var("OUT_DIR").unwrap();

    let _ = bindgen::builder()
        .header("include/wrapper.hpp")
        .generate_comments(false)
        .layout_tests(false)
        .clang_arg("-I../../include")
        .clang_arg("-I../../../../common/include")
        .clang_arg("-I../../../../common/libs/can")
        .clang_arg("-I../../../../common/libs/pid")
        .clang_arg("-I../../../../common/testing/mocks")
        .clang_arg("-I../../../../../api/include")
        .whitelisted_function("publish_brake_report")
        .whitelisted_function("check_for_incoming_message")
        .whitelisted_function("check_for_operator_override")
        .whitelisted_var("OSCC_MAGIC_BYTE_0")
        .whitelisted_var("OSCC_MAGIC_BYTE_1")
        .whitelisted_var("OSCC_BRAKE_REPORT_CAN_ID")
        .whitelisted_var("OSCC_BRAKE_REPORT_CAN_DLC")
        .whitelisted_var("OSCC_BRAKE_COMMAND_CAN_ID")
        .whitelisted_var("OSCC_BRAKE_COMMAND_CAN_DLC")
        .whitelisted_var("OSCC_FAULT_REPORT_CAN_ID")
        .whitelisted_var("MINIMUM_BRAKE_COMMAND")
        .whitelisted_var("MAXIMUM_BRAKE_COMMAND")
        .whitelisted_var("BRAKE_OVERRIDE_PEDAL_THRESHOLD_IN_DECIBARS")
        .whitelisted_var("OSCC_BRAKE_REPORT_PUBLISH_INTERVAL_IN_MSEC")
        .whitelisted_var("CAN_STANDARD")
        .whitelisted_var("CAN_MSGAVAIL")
        .whitelisted_type("oscc_brake_report_s")
        .whitelisted_type("oscc_brake_command_s")
        .whitelisted_type("can_frame_s")
        .whitelisted_type("kia_soul_brake_control_state_s")
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("brake_test.rs"))
        .expect("Unable to generate bindings");
}
