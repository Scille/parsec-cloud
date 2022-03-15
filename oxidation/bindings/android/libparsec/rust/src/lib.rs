#[macro_use] extern crate lazy_static;
extern crate jni;
extern crate android_logger;

use std::sync::Mutex;
use log::Level;
use android_logger::Config;
use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject, JValue};


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_scille_libparsec_Runtime_startRuntime(
    _env: JNIEnv,
    _class: JClass,
) -> bool {
    // TODO: initialize the runtime from here ?
    android_logger::init_once(
        Config::default().with_min_level(Level::Trace)
    );
    log::info!("LibParsec runtime initialized !");
    true
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_scille_libparsec_Runtime_submitJob(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
    cmd: JString,  // TODO: use JByteBuffer instead
    payload: JString,  // TODO: use JByteBuffer instead
) -> bool  {
    log::info!("LibParsec .submitJob ????");
    let cmd: String = env
        .get_string(cmd)
        .expect("Couldn't get java string!")
        .into();
    let payload: String = env
        .get_string(payload)
        .expect("Couldn't get java string!")
        .into();

    log::info!("LibParsec .submitJob lazy static stuff, cmd={}, payload={}", &cmd, &payload);
    lazy_static! {
        static ref LIBPARSEC_CTX: Mutex<libparsec_bindings_common::RuntimeContext> = Mutex::new(libparsec_bindings_common::create_context());
    }

    let mut runtime = LIBPARSEC_CTX.lock().unwrap();

    log::info!("LibParsec .submitJob java stuff");
    // Java stuff cannot be passed as-is between threads
    let callback_global_ref = env.new_global_ref(callback).unwrap();
    let jvm = env.get_java_vm().unwrap();

    log::info!("LibParsec about to submit_job...");
    let submitted = runtime.submit_job(Box::new(move || {
        log::info!("LibParsec within submit_job !");
        // The callback is being called from the libparsec thread, so
        // we send closure as a task to be executed by the JavaScript event
        // loop. This _will_ block the event loop while executing.
        let res = libparsec_bindings_common::decode_and_execute(&cmd, &payload);
        log::info!("LibParsec call ok !");

        let env2 = jvm.attach_current_thread().unwrap();
        let (method, payload) = match  res {
            Ok(data) => {
                ("resolve", data)
            },
            Err(err) => {
                ("reject", err)
            },
        };

        let payload = env2
        .new_string(payload)
        .expect("Couldn't create java string!");

        log::info!("LibParsec about to call callback...");
        let call_res = env2.call_method(
        callback_global_ref.as_obj(),
        method, "(Ljava/lang/String;)V",
        &[JValue::from(payload)],
        );
        log::info!("LibParsec about callback called !");
        if let Err(err) = call_res {
            log::error!("calling job callback has failed: {}", err);
        }
    }));
    log::info!("LibParsec submit_job done ! {:?}", &submitted);

    // TODO: raise exception ?
    match submitted {
        Ok(_) => true,
        Err(err) => {
            log::error!("submit job has failed: {}", err);
            false
        }
    }
}




// #[macro_use]
// extern crate lazy_static;
// extern crate jni;

// use std::sync::Mutex;
// use std::ffi::CString;
// use std::os::raw::c_char;

// use jni::JNIEnv;
// use jni::objects::{JClass, JObject, JValue, JString, JNull};

// pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "C" fn invokeCallbackViaJNA(callback: Callback) {
//     let s = CString::new("Hello from Rust").unwrap();
//     unsafe { callback(s.as_ptr()); }
// }

// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "C" fn Java_com_scille_parsec_LibParsec_SubmitJob(
//     env: JNIEnv,
//     _class: JClass,
//     cmd: JString,
//     payload: JString,
//     callback: JObject,
// ) {
//     lazy_static! {
//         static ref LIBPARSEC_CTX: Mutex<libparsec_bindings_common::RuntimeContext> = Mutex::new(libparsec_bindings_common::create_context());
//     }

//     let submitted = LIBPARSEC_CTX.lock().unwrap().submit_job(Box::new(move || {
//         // The callback is being called from the libparsec thread, so
//         // we send closure as a task to be executed by the JavaScript event
//         // loop. This _will_ block the event loop while executing.
//         let res = libparsec_bindings_common::decode_and_execute(&cmd, &payload);

//         let args = match res {
//             Ok(data) => [
//                 JNull(),
//                 JValue::from(JObject::from(data)),
//             ],
//             Err(err) => [
//                 JValue::from(JObject::from(err)),
//                 JNull(),
//             ],
//         };
//         env.call_method(
//             callback,
//             "callback", "(Ljava/lang/String;Ljava/lang/String;)V",
//             &args,
//         ).unwrap();
//     }));

//     match submitted {
//         Ok(_) => Ok(cx.undefined()),
//         Err(err) => cx.throw_error(err)
//     }

//     // let s = String::from("Hello from Rust");
//     // let response = env.new_string(&s)
//     //     .expect("Couldn't create java string!");
//     // env.call_method(callback, "callback", "(Ljava/lang/String;)V",
//     //                 &[JValue::from(JObject::from(response))]).unwrap();
// }
