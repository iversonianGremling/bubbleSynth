import {useState} from "react"
import SocketIcon from "../assets/socket.png";
import SocketSelectedIcon from "../assets/socket-white.png"
import { useContext, useRef } from "react";

interface SocketProps {
    id: string,
    angle: number,
    onClick: (e:any) => void,
    socketSelected: boolean
    position: any
}

const Socket = ({ id, angle, onClick, socketSelected, position}: SocketProps) => {


    let top = 60;
    let left = -30;
    let rotate = "270deg";
    const angleInRads = (angle / 360) * 2 * Math.PI;
    const radius = 70
    top = radius * Math.sin(-angleInRads) + 60 + position.y;
    left = radius * Math.cos(-angleInRads) + 60 - position.x;
    rotate = -angle + 90 + "deg";

    return (
    <div onClick={() => {
      onClick
      }}
    >
      <img
          id={id}
          draggable="false"
          src={socketSelected? SocketSelectedIcon : SocketIcon}
          onClick={onClick}
          style={{
            position: "absolute",
            rotate: rotate,
            top: { top }.top,
            left: { left }.left,
            width: 30,
            opacity: 1
          }}
        />
      </div>
    );
  };

export default Socket
