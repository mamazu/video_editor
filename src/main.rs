#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use boa_engine::JsError;
use boa_engine::JsNativeError;
use boa_engine::JsObject;
use boa_engine::JsString;
use boa_engine::NativeFunction;
use boa_engine::object::Object;
use evideo_editor::VideoEditor;
use std::env;
use std::fs;

use boa_engine::{Context, Source};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main_gui(_args: Arguments) -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    return eframe::run_native(
        "evideo_editor",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(VideoEditor::new(cc))),
    )
}
use boa_engine::{JsArgs, JsResult, JsValue};

fn log(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    println!("{:?}",args.get_or_undefined(0).to_string(context)?);
    return Ok(JsValue::undefined())
}

fn testing(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    println!("{:?}",args.get_or_undefined(0).to_string(context)?);
    return Ok(JsValue::undefined())
}

fn load_clip(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    let video_path = args.get_or_undefined(0);
    if !video_path.is_string() {
        let error = JsError::from_native(
            JsNativeError::typ().with_message("First argument of load_clip must be a string")
        );
        return Err(error);
    }

    println!("video path: {:?}", video_path);

    let result = JsObject::with_object_proto(context.intrinsics());
    let _ = result.create_data_property("path", "hello", context);
    let _ = result.create_data_property("hallo", NativeFunction::from_fn_ptr(testing), context);

    return Ok(JsValue::Object(result))
}

fn create_variable(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    return Ok(JsValue::undefined())
}

#[cfg(not(target_arch = "wasm32"))]
fn main_console(args: Arguments) -> eframe::Result<()>
{
    use boa_engine::NativeFunction;

    let contents = fs::read_to_string(args.file_path.unwrap())
        .expect("Should have been able to read the file");

    let mut context = Context::default();
    let _ = context.register_global_builtin_callable("log", 1, NativeFunction::from_fn_ptr(log));
    let _ = context.register_global_builtin_callable("load_clip", 1, NativeFunction::from_fn_ptr(load_clip));
    let _ = context.register_global_builtin_callable("create_variable", 1, NativeFunction::from_fn_ptr(createVariable));

    match context.eval(Source::from_bytes(&contents)) {
        Ok(res) => {
            println!("Return value of the program: {:#?}", res.to_string(&mut context).unwrap());
        }
        Err(e) => {
            // Pretty print the error
            eprintln!("Uncaught {:#?}", e);
        }
    };
    return Ok(())
}

fn print_help() -> () {
    println!("Usage: video_editor [file_path] [options]");
    println!("\t file_path \t Path of the script that produces the video");
    println!("");
    println!("Options:");
    println!("\t--gui\t\t Open the application in gui mode");
}

#[derive(Debug, Clone)]
struct Arguments {
    file_path: Option<String>,
    gui_mode: bool,
}

fn parse_arguments() -> Option<Arguments> {
    let mut args: Vec<String> = env::args().rev().collect();

    if args.len() == 1 {
        return None;
    }

    args.pop();


    let mut arguments = Arguments{file_path: None, gui_mode: false};
    let arg = args.pop().unwrap();

    if !arg.starts_with("--") {
        arguments.file_path = Some(arg);
    } else {
        args.push(arg)
    }

    arguments.gui_mode = args.contains(&"--gui".to_string());

    return Some(arguments);
}

fn main() -> Result<(), std::io::Error> {
    match parse_arguments() {
        Some(args) => {
            if args.gui_mode {
                main_gui(args).unwrap();
            } else {
                main_console(args).unwrap();
            }
        },
        _ => {
            print_help();
        }
    }

    return Ok(());
}
