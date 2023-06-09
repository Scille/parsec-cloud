// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use android_logger::Config;
use jni::objects::{JClass, JObject, JString};
use jni::JNIEnv;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Mutex<tokio::runtime::Runtime> =
        Mutex::new(tokio::runtime::Runtime::new().expect("Cannot start tokio runtime for libparsec"));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_scille_libparsec_Runtime_startRuntime(
    _env: JNIEnv,
    _class: JClass,
) -> bool {
    // TODO: initialize the runtime from here ?
    android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Trace));
    log::info!("LibParsec runtime initialized !");
    true
}

/// Interface between Web, Rust and Android
/// Cordova bridge API for Android requests the inputs and output to be passed as a JS Object
/// hence we have to comply with this ourself to provide the same API across platforms
///
/// Input:
///   - cmd: String
///   - payload: Base64 String separated by ':'
///
/// Output:
///   - value: Base64 String
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_scille_libparsec_Runtime_submitJob(
    _env: JNIEnv,
    _class: JClass,
    _callback: JObject,
    _cmd: JString,
    _payload: JString,
) -> bool {
    unimplemented!();
    // log::info!("LibParsec .submitJob ????");
    // let cmd: String = env
    //     .get_string(cmd)
    //     .expect("Couldn't get java string!")
    //     .into();
    // let payload: String = env
    //     .get_string(payload)
    //     .expect("Couldn't get java string!")
    //     .into();

    // log::info!(
    //     "LibParsec .submitJob lazy static stuff, cmd={}, payload={}",
    //     &cmd,
    //     &payload
    // );

    // log::info!("LibParsec .submitJob java stuff");
    // // Java stuff cannot be passed as-is between threads
    // let callback_global_ref = env.new_global_ref(callback).unwrap();
    // let jvm = env.get_java_vm().unwrap();

    // log::info!("LibParsec about to submit_job...");
    // let submitted = LIBPARSEC_CTX.lock().unwrap().submit_job(Box::new(move || {
    //     log::info!("LibParsec within submit_job !");
    //     // The callback is being called from the libparsec thread, so
    //     // we send closure as a task to be executed by the JavaScript event
    //     // loop. This _will_ block the event loop while executing.
    //     let res = Cmd::decode(&cmd, &payload).map(Cmd::execute);
    //     log::info!("LibParsec call ok !");

    //     let env2 = jvm.attach_current_thread().unwrap();
    //     let (method, payload) = match res {
    //         Ok(Ok(data)) => ("resolve", data),
    //         Ok(Err(err)) | Err(err) => ("reject", err),
    //     };

    //     let payload = env2
    //         .new_string(payload)
    //         .expect("Couldn't create java string!");

    //     log::info!("LibParsec about to call callback...");
    //     let call_res = env2.call_method(
    //         callback_global_ref.as_obj(),
    //         method,
    //         "(Ljava/lang/String;)V",
    //         &[JValue::from(payload)],
    //     );
    //     log::info!("LibParsec about callback called !");
    //     if let Err(err) = call_res {
    //         log::error!("calling job callback has failed: {}", err);
    //     }
    // }));
    // log::info!("LibParsec submit_job done ! {:?}", &submitted);

    // // TODO: raise exception ?
    // match submitted {
    //     Ok(_) => true,
    //     Err(err) => {
    //         log::error!("submit job has failed: {}", err);
    //         false
    //     }
    // }
}
