package com.scille.libparsec

interface IJobCallback {
    fun success(string: String)
    fun error(string: String)
}

class Runtime {
    init {
        System.loadLibrary("libparsec_bindings_jni")
        startRuntime()
    }

    external fun startRuntime(): Boolean;
    external fun submitJob(callback: IJobCallback, cmd: String, payload: String): Boolean;
}
