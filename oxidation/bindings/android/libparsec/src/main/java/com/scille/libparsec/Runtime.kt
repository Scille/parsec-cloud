package com.scille.libparsec

interface IJobCallback {
    fun resolve(string: String)
    fun reject(string: String)
}

class Runtime {
    init {
        System.loadLibrary("libparsec_bindings_jni")
        startRuntime()
    }

    external fun startRuntime(): Boolean;
    external fun submitJob(callback: IJobCallback, cmd: String, payload: String): Boolean;
}
