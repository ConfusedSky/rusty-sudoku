import React from "react";
import "./Steps.css";
import PropTypes from "prop-types";

function Steps({ solution, click, hover }) {
  const clickFunction = click || (() => () => undefined);

  const keyFunction = (index) => (event) => {
    if (event.key === "Enter") {
      clickFunction(index)();
    }
  };

  const hoverFunction = hover || (() => () => undefined);

  return (
    <div className="Steps">
      <div className="Steps-content">
        {solution.map((step, index) => (
          <div
            role="button"
            key={step.message}
            onClick={clickFunction(index)}
            onKeyDown={keyFunction(index)}
            onMouseEnter={hoverFunction(index)}
            onMouseLeave={hoverFunction(undefined)}
            onFocus={clickFunction(index)}
            className="Steps-item"
            tabIndex="0"
          >
            {step.message}
          </div>
        ))}
      </div>
    </div>
  );
}

Steps.propTypes = {
  solution: PropTypes.arrayOf(
    PropTypes.shape({
      message: PropTypes.string,
    }),
  ).isRequired,
  click: PropTypes.func,
  hover: PropTypes.func,
};

Steps.defaultProps = {
  click: undefined,
  hover: undefined,
};

export default Steps;
