onDrag={(e, dragElement) => {const bubbleStyle = dragElement.node.getAttribute("style")
setY(parseInt(bubbleStyle
.match(/transform: translate\((.+?)px/)[1]));
setX(parseInt(bubbleStyle
.match(/transform: translate\(-?\d*px, (.+?)px/)[1]));
{/*Ahora solo queda encontrar donde están los sockets y listo :')*/}
console.log(x)
console.log(y)}}

forwardedBubbleRefs={(bubble:HTMLInputElement) => {bubbleRefs.current.push(bubble)}}
