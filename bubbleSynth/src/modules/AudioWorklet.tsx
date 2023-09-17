// When using the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true

// Invoke the command

class WhiteNoiseProcessor extends AudioWorkletProcessor {
  process(inputs, outputs, parameters) {
    const output = outputs[0];
    let promise_response = [];
    invoke("generate_sound").then((result) => {
       promise_response = result;
    });
    output.forEach((channel) => {
      for (let i = 0; i < channel.length; i++) {
        channel[i] = promise_response[i];
      }
    });
    return true;
  }
}

registerProcessor("white-noise-processor", WhiteNoiseProcessor);
