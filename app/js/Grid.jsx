import React from "react";
import "./Grid.css";

function Grid(props) {
  return (
    <div className="Grid">
      {Array(9)
        .fill(0)
        .map((v, i) => (
          <div className="Grid-row" key={i}>
            {Array(9)
              .fill(0)
              .map((v2, i2) => (
                <div className="Grid-cell-container" key={i2}>
                  <div className="Grid-cell">{v2}</div>
                </div>
              ))}
          </div>
        ))}
    </div>
  );
}

export default Grid;
