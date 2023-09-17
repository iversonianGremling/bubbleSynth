import NoiseGen from "./NoiseGen";


let actx = new AudioContext();
let out = actx.destination;

await actx.audioWorklet.addModule("../public/worklets/white-noise-processor.js");

const whiteNoiseNode = new AudioWorkletNode(
  actx,
  "white-noise-processor"
)

whiteNoiseNode.connect(out);
actx.resume();


const App = () => {

  return (
    <div>
      <button onClick={() => actx.resume()}>Start</button>
      <button onClick={() => whiteNoiseNode.disconnect()}>Stop</button>
      <NoiseGen/>
    </div>
  );
};

export default App;
