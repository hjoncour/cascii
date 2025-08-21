import './styles/App.css';
import ASCIIAnimation from './components/ASCIIAnimation';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>ASCII Animation Viewer</h1>
      </header>
      <main>
        <h2>Small</h2>
        <div className="animation-container">
          <ASCIIAnimation fps={24} frameCount={120} frameFolder="small" className="small-animation" />
        </div>
        <h2>Small 2</h2>
        <div className="animation-container">
          <ASCIIAnimation fps={24} frameCount={120} frameFolder="frame_images" className="small-animation" />
        </div>
        <h2>Default</h2>
        <div className="animation-container">
          <ASCIIAnimation fps={24} frameCount={120} frameFolder="default" className="default-animation" />
        </div>
        <h2>Large</h2>
        <div className="animation-container">
          <ASCIIAnimation fps={60} frameCount={301} frameFolder="large" className="large-animation" />
        </div>
      </main>
    </div>
  );
}

export default App;
