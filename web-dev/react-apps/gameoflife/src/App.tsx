import React from 'react';
import './App.css';

import Game from './Game.js';

const App = () => {
  return (
    <div className="App">
      <h1>Welcome to the Game of Life</h1>
      <Game/>
    </div>
  );
}

export default App;
