import React from "react";
import "./Steps.css";

function Steps(props) {
  return (
    <div className="Steps">
      <div className="Steps-content">
        {props.solution.map((step, index) => (
          <div key={index} className="Steps-item">
            {step.message}
          </div>
        ))}
      </div>
    </div>
  );
}

export default Steps;
