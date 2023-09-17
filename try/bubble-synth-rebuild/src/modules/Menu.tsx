type MenuProps = {
    x: number;
    y: number;
    handleClick: (item: string) => void;
  };

function Menu({y, x, handleClick}:MenuProps) {

return (
        <div
          className="menu"
          style={{ top: y, left: x }}
        >
          <ul>
            <li onClick={() => handleClick("Oscillator")}>
              <div className="item-description">Oscillator</div>
            </li>
          </ul>
        </div>
      )
}

export default Menu
