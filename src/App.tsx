import { useState } from "react";
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import { } from "@tauri-apps/api/window"
import play_icon from './assets/play.svg';
import stop_icon from './assets/stop.svg';
import pause_icon from './assets/pause.svg';
import "./App.css";

import Playlist from "./components/Playlist";
import ProgressBar from "./components/ProgressBar";

function App() {
  const [playing, setPlaying] = useState(false);

  async function play(path: string) {
    await invoke("play", { path });
  }

  async function stop() {
    await invoke("stop", {});
  }

  async function pause() {
    await invoke("pause", {});
  }

  async function start() {
    await invoke("start", {});
  }

  async function openFile() {
    const path = await open({
      multiple: false,
      filters: [{
        name: 'Audio  ',
        extensions: ['mp3']
      }]
    })
    console.log(path);

    if (Array.isArray(path)) {

    } else if (path === null) {
      // user cancelled the selection
    } else {
      play(path);
    }
  }

  return (
    <div className="container">
      <button onClick={() => openFile()}>Open</button>

      {playing && <p>Playing</p>}

      <Playlist list={["test"]}></Playlist>

      <ProgressBar></ProgressBar>
      <div className="controls">
        <button className="control-button" onClick={() => start()}><img src={play_icon} /></button>
        <button className="control-button" onClick={() => pause()}><img src={pause_icon} /></button>
        <button className="control-button" onClick={() => stop()}><img src={stop_icon} /></button>
      </div>

    </div>
  );
}

export default App;
