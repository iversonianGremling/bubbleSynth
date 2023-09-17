import { forwardRef, useEffect, useRef, useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import Bubble from './modules/Bubble'
import Xarrow, {refType, useXarrow, Xwrapper} from 'react-xarrows';

function App() {
  const [count, setCount] = useState(0)

  const [lineBeingDrawn, setlineBeingDrawn] = useState(false)
  const [lineHasBeenDrawn, setlineHasBeenDrawn] = useState(false)
  const [startArrowID, setstartArrowID] = useState<string>("")
  const [endArrowID, setendArrowID] = useState<string>("")
  const [socketSelected, setsocketSelected] = useState<boolean>()

  const [cursorPos, setCursorPos] = useState<{x:string, y:string}>({x:"0", y:"0"})
  const cursorFollowerID = "cursorFollower"


  useEffect(() => {
    const handleMouseMove = (e) => {
      setCursorPos({ x: e.clientX, y: e.clientY });
    };

    window.addEventListener('mousemove', handleMouseMove);

    return () => {
      window.removeEventListener(
        'mousemove',
        handleMouseMove
      );
    };
  }, []);

  const handleSocketClick = (e) => {
    const socket = e.target.getAttribute("id")
    console.log(socket)

    if(!lineBeingDrawn){
      setstartArrowID(socket)
      setendArrowID("cursorFollower")
      setlineBeingDrawn(true)
      setlineHasBeenDrawn(false)
      setsocketSelected(true)
    }else{
      setendArrowID(socket)
      setlineBeingDrawn(false)
      setlineHasBeenDrawn(true)
      setsocketSelected(false)

    }
  }


  return (<div className='App'>
    <Bubble index={0} position={{x:300, y:-100}} debugSocketPos={0} onClickSocket={handleSocketClick} socketSelected={socketSelected}/>
    <Bubble index={1} position={{x:600, y:-100}} debugSocketPos={180} onClickSocket={handleSocketClick} socketSelected={socketSelected}/>
    {startArrowID && endArrowID ? <Xarrow start={startArrowID} end={endArrowID} showXarrow={lineHasBeenDrawn || lineBeingDrawn}/>:''}
    <div
        id='cursorFollower'
        className="cursorFollow"
        style={{ top: cursorPos.y, left: cursorPos.x }}
      />
    </div>
    )
}

export default App
