import React, { useEffect, useState } from "react";
import AudioGen from "./AudioGen";
import AudioProcessorRef from "./audio-processor?url";

import {invoke} from "@tauri-apps/api"

try{invoke('generate_sound')
.then((result) => console.log(result))} catch {console.log("Can't call tauri API")}

// https://github.com/microsoft/TypeScript/issues/28308#issuecomment-650802278

 const registerProcessor = (
  name: string,
  processorCtor: (new (
    options?: AudioWorkletNodeOptions
  ) => AudioWorkletProcessor) & {
    parameterDescriptors?: AudioParamDescriptor[];
  }
): void;


class AudioProcessor {
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

registerProcessor("audio-processor", AudioProcessorRef);


async function createWorkletNode(
  context: BaseAudioContext,
  name: string,
  url: string
) {
  try {
    return new AudioWorkletNode(context, name);
  } catch (err) {
    await context.audioWorklet.addModule(url);
    return new AudioWorkletNode(context, name);
  }
}

    const context = new AudioContext();
    const out = context.destination;
    const ap = await createWorkletNode(context, "audio-processor", AudioProcessorRef);
    ap.connect(out);
    context.resume();

const App = () => {

  return (
    <div>
      <button onClick={() => context.resume()}>Start</button>
      <button onClick={() => ap.disconnect()}>Stop</button>
      <AudioGen/>
    </div>
  );
};

export default App;
