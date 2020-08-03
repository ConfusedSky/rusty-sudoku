import React from "react";
import logo from "./logo.svg";
import "./App.css";
import Steps from "./Steps";
import { get_solution, get_grid } from "../pkg/index";
import Grid from "./Grid";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <div className="App-header-item">
          <img src={logo} className="App-logo" alt="logo" />
        </div>
        <h1 className="App-header-item">Sudoku Solver</h1>
      </header>
      <div className="App-rest">
        <Grid values={get_grid()}></Grid>
        <Steps solution={get_solution()}>
          <div></div>
        </Steps>
      </div>
    </div>
  );
}

export default App;
