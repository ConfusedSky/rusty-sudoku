/* eslint react/no-array-index-key: 0 */

import React from "react";
import "./Grid.css";
import PropTypes from "prop-types";

function Cell({ position, value }) {
  const [x, y] = position;

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
        <div className="Grid-cell-value">{value || <>&nbsp;</>}</div>
      </div>
    </div>
  );
}

Cell.propTypes = {
  position: PropTypes.arrayOf(PropTypes.number).isRequired,
  value: PropTypes.number,
};

Cell.defaultProps = {
  value: null,
};

function Grid({ values }) {
  const cells = (v, y) => v.map((value, index) => (
    <Cell position={[index, y]} key={index} value={value} />
  ));

  const rows = values.map((value, index) => (
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

Grid.propTypes = {
  values: PropTypes.arrayOf(PropTypes.arrayOf(PropTypes.number)).isRequired,
};

export default Grid;
