import { FaTimes} from "react-icons/fa";
import Draggable from 'react-draggable'; // Both at the same time
import Socket from "./Socket";
import OscillatorLogo from "../assets/oscillator.png"
import { useState } from "react";

const DraggableCore = Draggable.DraggableCore;

interface BubbleProps {
  index:string,
  position: {x: number, y: number},
  handleRemoveCircle: (e: any) => any,
  handleOnClickSocket: (e: any) => any,
  socketPos:number
}

const handleDrag = () => {}

const Bubble = ({index, position, socketPos, handleRemoveCircle, handleOnClickSocket}: BubbleProps) => {
const [x, setX] = useState(0)
const [y, setY] = useState(0)

  return (
      <div className="handler">
      <div
        style={{
        position: "absolute",
        top: position.y,
        left: position.x,
        width: 150,
        height: 150,
        }}
      >
          <div className={"handler"}>
            <div className={`circle`}>
              <img src={OscillatorLogo} draggable="false" width={150} height={150}></img>
            </div>
          </div>
          <span
            className="remove-circle"
            onClick={() => handleRemoveCircle(index)}
          >
            <FaTimes />
          </span>
      </div>
        <Socket id={index} angle={socketPos} onClick={handleOnClickSocket}  />
      </div>
  )
}

export default Bubble
