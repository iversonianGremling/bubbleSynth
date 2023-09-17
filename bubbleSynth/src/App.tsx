import React, { useEffect, useState } from "react";
import { animated, useTransition } from "react-spring";
import logos from "./Utilites/logos";
import Draggable from "react-draggable";
import Socket from "./modules/Socket";
import Xarrow, { useXarrow } from "react-xarrows";
import { FaTimes } from "react-icons/fa";
import parametersList from "./Utilites/bubbleTypeParamsList";
import { invoke } from '@tauri-apps/api'

const App = () => {
  const [cursorPos, setCursorPos] = useState({x: 0, y:0})

  const [menuPosition, setMenuPosition] = useState({ x: 0, y: 0 });
  const [menuItems, setMenuItems] = useState<object[]>([]);

  const [bubbleArray, setBubbleArray] = useState<object[]>([]);
  const [bubblePositions, setBubblePositions] = useState<object[]>([]);
  const [bubblePosition, setbubblePosition] = useState({x: 0, y: 0})

  const [paramItems,  setParamItems] = useState<object[]>([])
  const [paramPositions, setparamPositions] = useState<object[]>([])

  const [startArrowIDs, setstartArrowIDs] = useState<string[]>([])
  const [endArrowIDs, setendArrowIDs] = useState<string[]>([])
  const [lineBeingDrawn, setlineBeingDrawn] = useState(false)
  const [lineHasBeenDrawn, setLineHasBeenDrawn] = useState(false)
  const [socketSelected, setsocketSelected] = useState(false)


  invoke('generate_sound')
  .then((result) => console.log(result));

  let DraggableCore = Draggable.DraggableCore;
  DraggableCore = Draggable.DraggableCore;
  const updateXarrow = useXarrow();

  useEffect(() => {
    const handleClick = (e) => {
      const { clientX, clientY } = e;
      setMenuPosition({ x: clientX, y: clientY });
    };
    document.addEventListener("mousemove", handleClick);
    return () => {
      document.removeEventListener("mousemove", handleClick);
    };
  }, []);

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
  }, [])

  useEffect(() => {
    const handleMouseMove = (e) => {
    const {clientX, clientY } = e;
    setCursorPos({x: clientX, y: clientY})
    if(!lineBeingDrawn && socketSelected) setendArrowIDs("cursorFollower")}

    window.addEventListener('mousemove', handleMouseMove);

    return () => {
      window.removeEventListener(
        'mousemove',
        handleMouseMove
      );
    };
   }, [])



  const menuTransition = useTransition(menuItems, {
    from: { x: menuPosition.x, y: menuPosition.y, opacity: 0 },
    enter: (item) => (next) =>
      next({ x: item.x, y: item.y, opacity: 1, delay: item.delay }),
    leave: { opacity: 0 },
  });

  const bubbleTransition = useTransition(bubbleArray, {
    from: { opacity: 0 },
    enter: { opacity: 1 },
    leave: { opacity: 0 },
  });

  const paramTransition = useTransition(paramItems, {
    from: {x: bubblePosition.x, y: bubblePosition.y, opacity: 0},
    enter: item => (next) => (
      next({x:item.x, y:item.y, opacity: 1, delay: item.delay})
    ),
    leave: {x: bubblePosition.x, y: bubblePosition.y, opacity: 0}
  });

  const getCirclePosition = (position: number, numberOfElements: number) => {
    const circleRadius = 100;
    const centerX = menuPosition.x;
    const centerY = menuPosition.y;
    setBubblePositions([...bubblePositions, menuPosition]);
    const xPos =
      circleRadius * Math.cos(((2 * Math.PI) / numberOfElements) * position) +
      centerX;
    const yPos =
      circleRadius * Math.sin(((2 * Math.PI) / numberOfElements) * position) +
      centerY;
    return { x: xPos, y: yPos };
  };

  const getParamCirclePosition = (position: number, numberOfElements: number, center: object) => {
    const circleRadius = 100
    const centerX = center.x + 37
    const centerY = center.y + 37
    setbubblePosition({x: centerX, y: centerY})
    setparamPositions([...paramPositions, {x: centerX, y: centerY}])
    const xPos = circleRadius * Math.cos(2 * Math.PI / numberOfElements * position) + centerX
    const yPos = circleRadius * Math.sin(2 * Math.PI / numberOfElements * position) + centerY
    return {x:xPos, y:yPos}
  };

  const handleScreenClick = (e) => {
  const items = { //TODO
    ADSR: {
      name: "ADSR",
      logo: logos.ADSRIcon,
      nameParams: ["Atack","Decay","Sustain","Release", "Trigger", "Output"]
    }
  }
    setMenuItems((v) =>
      v.length
        ? []
        : [
            {
              imgSrc: logos.ADSRIcon,
              x: getCirclePosition(1, 6).x,
              y: getCirclePosition(1, 6).y,
              type: "ADSR"
            },
            {
              imgSrc: logos.OscillatorIcon,
              x: getCirclePosition(2, 6).x,
              y: getCirclePosition(2, 6).y,
              type: "Oscillator"
            },
            {
              imgSrc: logos.SamplerIcon,
              x: getCirclePosition(3, 6).x,
              y: getCirclePosition(3, 6).y,
              type: "Sampler"
            },
            {
              imgSrc: logos.SequencerIcon,
              x: getCirclePosition(4, 6).x,
              y: getCirclePosition(4, 6).y,
              type: "Sequencer"
            },
            {
              imgSrc: logos.LFOIcon,
              x: getCirclePosition(5, 6).x,
              y: getCirclePosition(5, 6).y,
              type: "LFO"
            },
            {
              imgSrc: logos.EffectIcon,
              x: getCirclePosition(6, 6).x,
              y: getCirclePosition(6, 6).y,
              type: "Sequencer"
            },
          ]
    );
  };

  const createBubble = (e) => {
    const bubbleType: string = e.target.getAttribute("type")
    const newBubble = {
      id: bubbleArray.length,
      img: e.target.getAttribute("src"),
      params: parametersList[bubbleType]
    };
    setBubbleArray((prevBubbles) => [...prevBubbles, newBubble]);
  };

  const handleRemoveBubble = (e, index: number) => {
    e.stopPropagation()
    setBubbleArray((a) =>
      a.filter((_, i) =>
        i !== index
      )
    );
  };

  const handleOnClickBubble = (e) => {
    e.stopPropagation()
    const center = e.target.getBoundingClientRect()
    const bubbleType = e.target
    const paramNameArray = ["Pan", "Volume", "Detune"]
    const namesLength = paramNameArray.length
    let paramItemsDummy: object[] = []
    for (let i  = 0; i  < paramNameArray.length; i ++) {
      paramItemsDummy[i] = {
        paramName: paramNameArray[i],
        x: getParamCirclePosition(i, namesLength, center).x,
        y: getParamCirclePosition(i, namesLength, center).y,
      }
    }
    setParamItems(v => v.length ? [] : paramItemsDummy)
  }

  const getParamName = (bubbleType: object, index: number) =>
  {
    //parametersList[bubbleType][index]
    return ["Detune", "Volume", "Pan"][index]
  }

  const handleSocketClick = (e) => {
    e.stopPropagation()
    const socket = e.target.getAttribute("id")

    if(!lineBeingDrawn){
      setstartArrowIDs([...startArrowIDs, socket])
      setendArrowIDs([...endArrowIDs, "cursorFollower"])
      setlineBeingDrawn(true)
      setLineHasBeenDrawn(false)
      setsocketSelected(true)
    }else{
      setendArrowIDs([...endArrowIDs.slice(0,-1), socket])
      setlineBeingDrawn(false)
      setLineHasBeenDrawn(true)
      setsocketSelected(false)
    }
  }

  return (
    <div className="fixed inset-0 bg-gray-500" onClick={handleScreenClick}>
      <div className="container">
        {menuTransition((style, item) =>
          item ? (
            <animated.div
              style={style}
              className="circle absolute w-9 h-9 bg-white rounded-full"
              onClick={createBubble}
            >
              <img className="logo object-fill absolute" src={item.imgSrc} type={item.type}  />
            </animated.div>
          ) : (
            ""
          )
        )}
        {bubbleTransition((style, item, _, i) =>
          item && bubbleArray[i]? (
            <Draggable>
              <animated.div
                style={{
                  ...style,
                  x: bubblePositions[i].x - 42,
                  y: bubblePositions[i].y - 42,
                }}
              >
                <div
                  className="absolute left-28 remove-circle rounded-full bg-slate-700 w-4 h-4"
                  onClick={(e) => {
                    handleRemoveBubble(e, i);
                  }}
                >
                  <FaTimes  />
                </div>

                <div
                  className="absolute bg-white rounded-full w-28"
                  onClick={handleOnClickBubble}
                  onDrag={updateXarrow}
                >
                  <img src={bubbleArray[i].img} draggable={false} />
                </div>
                {bubbleArray[i].params.map((param, i_param) => {
                  return bubbleArray[i].params.length % 2 === 0 ?
                  <Socket id = {i * 10 + i_param} angle={360/bubbleArray[i].params.length * i_param} onClick={handleSocketClick} socketSelected={socketSelected} position={{x:20, y:-20}}/>
                  :
                  <Socket id = {i * 10 + i_param} angle={360/bubbleArray[i].params.length * i_param + 50} onClick={handleSocketClick} socketSelected={socketSelected} position={{x:20, y:-20}}/>
                })}
              </animated.div>
            </Draggable>
          ) : (
            ""
          )
        )}
        {paramTransition((style, item, _, i) =>
          item ? (
            <animated.div
              style={style}
              className="flex items-center justify-center absolute w-9 h-9 bg-white rounded-full">
              <div>
                {getParamName(item, i)}
                              </div>
              <Socket id = {item.toString() + i} angle={90} onClick={handleSocketClick} socketSelected={socketSelected} position={{x:58, y:0}}
                />

              </animated.div>
          ):(""))}

        <div>
        {startArrowIDs.map((ID, i) => {
          const start = startArrowIDs[i]
          const end = endArrowIDs[i]
          let validConnection = true
          if(start === end) validConnection = false
          return validConnection ? <Xarrow start={startArrowIDs[i]} end={endArrowIDs[i]}
          showXarrow={lineHasBeenDrawn || lineBeingDrawn}/>: ('')
        })}
        </div>
         <div
        id='cursorFollower'
        className="absolute bg-transparent rounded-full w-5 h-5"
        style={{ top: cursorPos.y, left: cursorPos.x }}
      > </div>
    </div>
      </div>
  );
};

export default App;
