import React from "react";
import "./Grid.css";

function Cell(props) {
  return (
    <div className="Grid-cell-container">
      <div className="Grid-cell-border"></div>
      <div className="Grid-cell-inner">
        <div className="Grid-cell-value">{props.value || <>&nbsp;</>}</div>
      </div>
    </div>
  );
}

function Grid(props) {
  const cells = (v) =>
    v.map((value, index) => <Cell key={index} value={value}></Cell>);

  const rows = props.values.map((value, index) => (
    <div className="Grid-row" key={index}>
      {cells(value)}
    </div>
  ));

  return (
    <div className="Grid-container">
      <div className="Grid">{rows}</div>
    </div>
  );
}

export default Grid;
