import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";
import Integration from './components/integration/Integration'
import Sidebar from './components/sidebar/sidebar'
import MobileMenu from './components/mobileMenu/mobileMenu'

function App() {

  // useEffect(() => {
  //   auto_start();
  // }, []);

  const auto_start = async () => {
    // set auto-start  -> only required first-time -> will be vary on OS ...
    await enable();
    console.log(`registered for autostart? ${await isEnabled()}`);
    // disable();
    //end.
  };
  const [activeButton, setActiveButton] = useState(1);
  const [count, setCount] = useState(0)
  const [message, setMessage] = useState()
  const [config, setConfig] = useState()

  const callMessage = () => {
    invoke("greet", { name: "Roman" }).then((response) => setMessage(response))
  };

  const callCrash = () => {
    invoke("crash").then((response) => setMessage(response))
  };

  const callReadConfig = () => {
    invoke("read_config").then((response) => {
      setConfig(response);
    }).catch((error) => {
      console.error('Error invoking read_config:', error);
      // setConfig('Failed to read config'); // or handle error as needed
    });
  };

  return (
    <>
      <div className="flex min-h-screen"> {/* Ensure the flex container covers full height */}
        {/* <div className='fixed left-0 top-0 h-screen w-64 bg-gray-100 text-white'>
        <Sidebar />
        </div> */}
        <MobileMenu activeButton={activeButton} setActiveButton={setActiveButton} />

        <div className="flex-1 flex flex-col overflow-y-auto"> {/* Flex-1 ensures this div takes up remaining space and scrolls */}
          <Integration />
        </div>
      </div>

      {/* <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>This Is Banana
      </h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <button onClick={() => callMessage()}>
          Rust function : {message}
        </button>
        <button onClick={() => callCrash()}>
          Crash function
        </button>
        <button onClick={() => auto_start()}>
          Auto-start
        </button>
        <button onClick={() => callReadConfig()}>
          Config File : {JSON.stringify(config)}
          Read Config
        </button>
        <p>
          Edit <code>src/App.jsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p> */}
    </>
  )
}

export default App
