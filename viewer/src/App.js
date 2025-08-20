import './App.css';
import ASCIIAnimation from './components/ASCIIAnimation';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>ASCII Animation Viewer</h1>
      </header>
      <main>
        <div className="animation-container">
          <h2>Small</h2>
          <ASCIIAnimation fps={24} frameCount={120} frameFolder="small" />
        </div>
        <div className="animation-container">
          <h2>Default</h2>
          <ASCIIAnimation fps={24} frameCount={120} frameFolder="default" />
        </div>
        <div className="animation-container">
          <h2>Large</h2>
          <ASCIIAnimation fps={60} frameCount={301} frameFolder="large" />
        </div>
      </main>
    </div>
  );
}

export default App;
