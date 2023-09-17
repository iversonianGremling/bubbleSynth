import {useState} from "react"
import SocketIcon from "../assets/socket.png";
import SocketSelectedIcon from "../assets/socket-white.png"
import { useContext, useRef } from "react";

interface SocketProps {
    id: string,
    angle: number,
    onClick: (e:any) => void,
    socketSelected: boolean
}

const Socket = ({ id, angle, onClick, socketSelected }: SocketProps) => {


    let top = 60;
    let left = -30;
    let rotate = "270deg";
    const angleInRads = (angle / 360) * 2 * Math.PI;
    top = 90 * Math.sin(-angleInRads) + 60;
    left = 90 * Math.cos(-angleInRads) + 60;
    rotate = -angle + 90 + "deg";

    return (
    <div onClick={() => {
      onClick
      }}>
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
          }}
        />
      </div>
    );
  };

export default Socket
