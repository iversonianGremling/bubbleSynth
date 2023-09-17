import React, { useState, useEffect} from 'react';

const CursorGhost = () => {
  const [position, setPosition] = useState({ x: 0, y: 0 });

  const handleMouseMove = (e) => {
    setPosition({ x: e.clientX, y: e.clientY });
  };

  useEffect(() => {
    document.addEventListener('mousemove', handleMouseMove);

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
    };
  }, []);

  return (
    <div id="cursor" hidden={false} style={{ position: 'fixed', left: position.x, top: position.y }}>
      <div
        style={{
          width: '10px',
          height: '10px',
          borderRadius: '50%',
        }}
      ></div>
    </div>
  );
}

export default CursorGhost
