import SocketIcon from "../assets/socket.png";

interface SocketInterface {
    id: string
    angle: number
    onClick: (e:any) => null,
}

const Socket = ({ id, angle, onClick}: SocketInterface) => {

  const angleInRads = (angle / 360) * 2 * Math.PI;
  const top = 90 * Math.sin(-angleInRads) + 60;
  const left = 90 * Math.cos(-angleInRads) + 60;
  const rotate = -angle + 90 + "deg";

    return (
        <img
          id={id}
          draggable={false}
          src={SocketIcon}
          onClick={onClick}
          style={{
            position: "absolute",
            rotate: rotate,
            top: { top }.top,
            left: { left }.left,
            width: 30,
          }}
        />
    );}

export default Socket
