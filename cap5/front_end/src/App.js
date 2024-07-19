import { useState } from 'react';

import logo from './logo.svg';
import './App.css';

function App() {
  const [state, set_state] = useState({ message: 'To Do' });

  return (
    <div className="App">
      <p>{state.message} application</p>
    </div>
  );
}

export default App;
