import { useState, useEffect } from 'react';
import './styles/App.css';
import ASCIIAnimation from './components/ASCIIAnimation';

function App() {
  const [activeTab, setActiveTab] = useState('tests');
  const [projects, setProjects] = useState([]);

  useEffect(() => {
    const fetchProjects = async () => {
      try {
        const response = await fetch('/projects.json');
        if (!response.ok) {
          // If projects.json doesn't exist, default to an empty array
          if (response.status === 404) {
            setProjects([]);
            return;
          }
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        setProjects(data);
      } catch (error) {
        console.error("Could not fetch projects:", error);
        setProjects([]);
      }
    };

    fetchProjects();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <h1>ASCII Animation Viewer</h1>
        <nav>
          <button onClick={() => setActiveTab('tests')} className={activeTab === 'tests' ? 'active' : ''}>Tests</button>
          <button onClick={() => setActiveTab('projects')} className={activeTab === 'projects' ? 'active' : ''}>Projects</button>
        </nav>
      </header>
      <main>
        {activeTab === 'tests' && (
          <>
            <h2>Small</h2>
            <div className="animation-container">
              <ASCIIAnimation fps={24} frameCount={120} frameFolder="small" className="small-animation" />
            </div>
            <h2>Default</h2>
            <div className="animation-container">
              <ASCIIAnimation fps={24} frameCount={120} frameFolder="default" className="default-animation" />
            </div>
            <h2>Large</h2>
            <div className="animation-container">
              <ASCIIAnimation fps={60} frameCount={301} frameFolder="large" className="large-animation" />
            </div>
          </>
        )}
        {activeTab === 'projects' && (
          <div>
            {projects.length > 0 ? (
              projects.map(project => (
                <div key={project.name}>
                  <h2>{project.name}</h2>
                  <div className="animation-container">
                    <ASCIIAnimation
                      fps={project.fps}
                      frameCount={project.frameCount}
                      frameFolder={project.name}
                      className="project-animation"
                    />
                  </div>
                </div>
              ))
            ) : (
              <p>No projects found. Use the 'casci-demo' command to add a new project.</p>
            )}
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
