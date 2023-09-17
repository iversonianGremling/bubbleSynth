import React, { useEffect, useState } from 'react';
import { animated, useTransition } from 'react-spring'
import logos from "./Utilites/logos"
import { Parser } from "html-to-react";


const App = () => {
  const [bubblePosition, setBubblePosition] = useState({ x: 0, y: 0 });
  const [paramItems, setParamItems] = useState<object[]>([]);
  const [bubbleArray, setBubbleArray] = useState<object[]>([])
  const [bubblePositions, setBubblePositions] = useState<object[]>([])

  const htmlParser = new Parser();

  const paramTransition = useTransition(paramItems, {
    from: {x: bubblePosition.x, y: bubblePosition.y, opacity: 0},
    enter: item => (next) => (
      next({x:item.x, y:item.y, opacity: 1, delay: item.delay})
    ),
    leave: {x: bubblePosition.x, y: bubblePosition.y, opacity: 0}
  });

  const bubbleTransition = useTransition(bubbleArray, {
    from: {opacity: 0},
    enter: {opacity: 1},
    leave: {opacity: 0},
  })

  const getCirclePosition = (position: number, numberOfElements: number, center: object) => {
    const circleRadius = 100
    setBubblePosition({x: center.x -60, y: center.y -430})
    const centerX = center.x - 60
    const centerY = center.y - 430
    setBubblePositions([...bubblePositions, bubblePosition])
    const xPos = circleRadius * Math.cos(2 * Math.PI / numberOfElements * position) + centerX
    const yPos = circleRadius * Math.sin(2 * Math.PI / numberOfElements * position) + centerY
    return {x:xPos, y:yPos}
  }

  const handleBubbleClick = (e) => {
    const center = e.target.getBoundingClientRect()
    setParamItems(v => v.length ? [] : [
      { paramName: "Freq", x:getCirclePosition(1, 6, center).x, y:getCirclePosition(1, 6, center).y},
      { paramName: "Phase", x:getCirclePosition(2, 6, center).x, y:getCirclePosition(2, 6, center).y},
      { paramName: "Volume", x:getCirclePosition(3, 6, center).x, y:getCirclePosition(3, 6, center).y},
      { paramName: "Waveform", x:getCirclePosition(4, 6, center).x, y:getCirclePosition(4, 6, center).y},
      { paramName: "Detune", x:getCirclePosition(5, 6, center).x, y:getCirclePosition(5, 6, center).y},
      { paramName: "Pan", x:getCirclePosition(6, 6, center).x, y:getCirclePosition(6, 6, center).y},
    ])
  };


  return (
    <div className="flex items-center justify-center fixed inset-0 bg-gray-500" >
      <div className="container">
        {paramTransition((style, item) =>
          item ? (
            <animated.div
              style={style}
              className="circle absolute w-9 h-9 bg-white rounded-full"
            >
              <img
                className="logo object-fill absolute"
                src={item.imgSrc}
              />
            </animated.div>
          ) : (
            ""
          )
        )}
        {bubbleTransition((style, item, state, i) =>
          item ? (
            <animated.div
              className="flex items-center justify-center absolute bg-white rounded-full w-28"
              style={{...style,
              x: bubblePositions[i].x,
              y: bubblePositions[i].y}}>
            </animated.div>
            ) : ("")
        )}
      </div>
      <div className="circle absolute w-24 h-24 bg-white rounded-full" onClick={handleBubbleClick}>
      </div>
    </div>
  );
};

export default App;
