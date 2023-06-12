package com.scille.parsec;

import android.util.Log;

import androidx.annotation.NonNull;

import com.getcapacitor.JSObject;
import com.getcapacitor.Plugin;
import com.getcapacitor.PluginCall;
import com.getcapacitor.PluginMethod;
import com.getcapacitor.annotation.CapacitorPlugin;

import com.scille.libparsec.Runtime;
import com.scille.libparsec.IJobCallback;

class JobCallback implements IJobCallback {
  PluginCall callback;

  public JobCallback(PluginCall callback) {
    this.callback = callback;
  }

  public void resolve(String res) {
    // Capacitor only support building JS objects, so we must wrap res
    JSObject ret = new JSObject();
    ret.put("value", res);
    this.callback.resolve(ret);
  }

  public void reject(String res) {
    this.callback.reject(res);
  }
}

@CapacitorPlugin(name = "LibParsec")
public class LibParsecPlugin extends Plugin {
  Runtime runtime;

  @Override
  public void load() {
    this.runtime = new Runtime();
  }

  @PluginMethod()
  public void submitJob(@NonNull PluginCall call) {
      Log.w("!!!!!!!!!!!!!!!!!!TAG", "submitJob: " + call.getData().toString());
    String cmd = call.getString("cmd");
    if(cmd == null) {
        cmd = "foo";
//      call.reject("Input option 'cmd' must be provided.");
//      return;
    }
    String payload = call.getString("payload");
    if(payload == null) {
        payload = "bar";
//      call.reject("Input option 'payload' must be provided.");
//      return;
    }

    this.runtime.submitJob(new JobCallback(call), cmd, payload);
  }
}
