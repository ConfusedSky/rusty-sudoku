import React from "react";
import logo from "./logo.svg";
import "./App.css";
import Steps from "./Steps";
import {
  get_solution as getSolution,
  default_context as defaultContext,
} from "../pkg/index";
import Grid from "./Grid";

class App extends React.PureComponent {
  constructor() {
    super();

    const context = defaultContext();

    this.state = {
      grid: 0,
      hoverGrid: undefined,
      solution: getSolution(context),
    };
  }

  render() {
    const { grid, solution, hoverGrid } = this.state;

    return (
      <div className="App">
        <header className="App-header">
          <div className="App-header-item">
            <img src={logo} className="App-logo" alt="logo" />
          </div>
          <h1 className="App-header-item">Sudoku Solver</h1>
        </header>
        <div className="App-rest">
          <Grid values={solution.grids[hoverGrid || grid]} />
          <Steps
            solution={solution.steps}
            click={(index) => () => this.setState({ grid: index })}
            hover={(index) => () => this.setState({ hoverGrid: index })}
          />
        </div>
      </div>
    );
  }
}

export default App;
