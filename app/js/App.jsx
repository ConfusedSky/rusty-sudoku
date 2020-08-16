import React from "react";
import logo from "./logo.svg";
import "./App.css";
import Steps from "./Steps";
import { get_grid as getGrid, get_solution as getSolution } from "../pkg/index";
import Grid from "./Grid";

class App extends React.PureComponent {
  constructor() {
    super();

    this.state = {
      grid: getGrid(),
      solution: getSolution(),
    };
  }

  render() {
    const { grid, solution } = this.state;

    return (
      <div className="App">
        <header className="App-header">
          <div className="App-header-item">
            <img src={logo} className="App-logo" alt="logo" />
          </div>
          <h1 className="App-header-item">Sudoku Solver</h1>
        </header>
        <div className="App-rest">
          <Grid values={grid} />
          <Steps solution={solution} />
        </div>
      </div>
    );
  }
}

export default App;
