// white-noise-processor.js
import invoke from "@tauri/api"

class WhiteNoiseProcessor extends AudioWorkletProcessor {
    process(inputs, outputs, parameters) {
      const output = outputs[0];
      output.forEach((channel) => {
        for (let i = 0; i < channel.length; i++) {
          channel[i] = Math.random() * 2 - 1;
        }
      });
      return true;
    }
  }

  registerProcessor("white-noise-processor", WhiteNoiseProcessor);

  export default WhiteNoiseProcessor
