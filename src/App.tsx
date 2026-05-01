import { Routes } from 'react-router';
import "./App.css";
import { Route } from 'react-router';
import Home from './pages/Home';
import Sender from './pages/Sender';
import Receiver from './pages/Receiver';

function App() {
  

  return (
    
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path='/sender' element={<Sender />} />
      <Route path="/receiver" element={<Receiver />} />
    </Routes>
  );
}

export default App;
