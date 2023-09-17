import {invoke} from "@tauri-apps/api"

try{invoke('generate_sound')
.then((result) => console.log(result))} catch {console.log("Can't call tauri API")}

class AudioProcessor extends AudioWorkletProcessor {
  process(inputs, outputs, parameters) {
    const output = outputs[0];
    output.forEach((channel) => {
      for (let i = 0; i < channel.length; i++) {
        channel[i] = (Math.random() * 2 - 1) * 0.05;
      }
    });
    return true;
  }
}

registerProcessor("audio-processor", AudioProcessor);

export default AudioProcessor

