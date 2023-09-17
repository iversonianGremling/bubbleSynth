import React, { useState, useRef } from 'react'
import './App.css'
import Bubble from './modules/Bubble'
import Xarrow, {useXarrow, Xwrapper} from 'react-xarrows';
import CursorGhost from './modules/CursorGhost';

function App() {

  const bubbleRefs = useRef<any[]>([])
  const socketRefs = useRef<any[]>([])

  const [startArrowPos, setstartArrowPos] = useState()
  const [endArrowPos, setendArrowPos] = useState()
  const [lineIsBeingDrawn, setlineIsBeingDrawn] = useState(false)

  const handleRemoveBubble = (e:React.SyntheticEvent) => {
    const bubbleRef = e.target
  }

  const updateXarrow = useXarrow()

  const handleOnClickSocket = (e:React.SyntheticEvent) => {
    const socketStyle = e.target.getAttribute("style")
    const top = socketStyle.match(/top: (.+?)px;/)
    const left = socketStyle.match(/left: (.+?)px;/)
    console.log(`The socket is ${e.target.getAttribute("id")}`)
    console.log(`The top position is ${top[1]}`)
    console.log(`The left position is ${left[1]}`)

    const socket = e.target.getAttribute("id")
    if (!lineIsBeingDrawn){
      setstartArrowPos(socket)
      setlineIsBeingDrawn(true)
    } else {
      setstartArrowPos(socket)
      setlineIsBeingDrawn(false)
    }

  }

  return (
  <>
  <Xwrapper>
    <Bubble
      index={'0'}
      handleRemoveCircle={handleRemoveBubble}
      handleOnClickSocket={handleOnClickSocket}
      position={{x: 630, y: 20}}
      socketPos={180}
      />
      <Bubble
      index={'1'}
      handleRemoveCircle={handleRemoveBubble}
      handleOnClickSocket={handleOnClickSocket}
      position={{x: 260, y: 20}}
      socketPos={0}
      />
      <Xarrow start={'1'} end={'0'}/>
      {/*nota, para hacerle animacion a la flecha hacer que un elemento siga al mouse y que sea invisible*/}
    </Xwrapper>
    <CursorGhost/>
  </>
  )
}

export default App
