import React from "react";
import "./Steps.css";
import PropTypes from "prop-types";

function Steps({ solution }) {
  return (
    <div className="Steps">
      <div className="Steps-content">
        {solution.map((step) => (
          <div key={step.message} className="Steps-item">
            {step.message}
          </div>
        ))}
      </div>
    </div>
  );
}

Steps.propTypes = {
  solution: PropTypes.arrayOf(PropTypes.shape({
    message: PropTypes.string,
  })).isRequired,
};

export default Steps;
