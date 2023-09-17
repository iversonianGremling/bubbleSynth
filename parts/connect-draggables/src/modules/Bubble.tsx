import Draggable from "react-draggable"
import oscillatorLogo from "../assets/oscillator.png"
import { FaTimes} from "react-icons/fa"
import '../App.css'
import Socket from "./Socket"
import { useXarrow } from "react-xarrows"

interface bubbleProps {
    index: number,
    position: {x: number, y: number}
    handleRemoveCircle: (e:any) => any
    debugSocketPos: number
    onClickSocket: (e:any) => any
    socketSelected: boolean
}

{/*hacer que cada una gestione sus sockets*/}

let DraggableCore = Draggable.DraggableCore;

function Bubble({index, position, handleRemoveCircle, debugSocketPos, onClickSocket, socketSelected}: bubbleProps) {

  const bubbleHeight = "150"
  const bubbleWidth = "150"

  const bubbleStyle = {
    position: "relative",
    top: position.y,
    left: position.x ,
  }
  const debugSocketNumber = "1"
  const updateXarrow = useXarrow()

return (
  <Draggable
  onDrag={updateXarrow}>
  <div>
    <div
      style={bubbleStyle}
      className="handler circleContainer">
        <div className="circle">
          <img
            width={bubbleWidth}
            height={bubbleHeight}
            src={oscillatorLogo}
            draggable={false}
            />
        </div>
        <span
        className="remove-circle"
        onClick={() => handleRemoveCircle(index)}
        >
        <FaTimes />
        </span>
        <Socket angle={debugSocketPos} onClick={onClickSocket} id={index + debugSocketNumber} socketSelected={socketSelected}/>
      </div>
          </div>
  </Draggable>)
}

export default Bubble
