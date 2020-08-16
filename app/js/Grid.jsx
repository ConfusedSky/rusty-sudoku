import React from "react";
import "./Grid.css";

function Cell(props) {
  const [x, y] = props.position;

  const leftBorder = x === 0;
  const rightBorder = x % 3 === 2;

  const topBorder = y === 0;
  const bottomBorder = y % 3 === 2;

  let borderClass = "Grid-cell-border";

  if (leftBorder) {
    borderClass += " Grid-thick-border-L";
  }

  if (rightBorder) {
    borderClass += " Grid-thick-border-R";
  }

  if (topBorder) {
    borderClass += " Grid-thick-border-T";
  }

  if (bottomBorder) {
    borderClass += " Grid-thick-border-B";
  }

  return (
    <div className="Grid-cell-container">
      <div className={borderClass} />
      <div className="Grid-cell-inner">
        <div className="Grid-cell-value">{props.value || <>&nbsp;</>}</div>
      </div>
    </div>
  );
}

function Grid(props) {
  const cells = (v, y) => v.map((value, index) => (
    <Cell position={[index, y]} key={index} value={value} />
  ));

  const rows = props.values.map((value, index) => (
    <div className="Grid-row" key={index}>
      {cells(value, index)}
    </div>
  ));

  return (
    <div className="Grid-container">
      <div className="Grid">{rows}</div>
    </div>
  );
}

export default Grid;
