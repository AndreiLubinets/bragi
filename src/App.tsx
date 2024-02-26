import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { } from "@tauri-apps/api/window"
import play_icon from './assets/play.svg';
import stop_icon from './assets/stop.svg';
import pause_icon from './assets/pause.svg';

import Playlist from "./components/Playlist";
import ITrack from "./interfaces/track";
import { listen } from "@tauri-apps/api/event";
import Volume from "./components/Volume";
import ProgressBar from "./components/ProgressBar";

function App() {
  const [playList, setPlayList] = useState<ITrack[]>([]);

  listen('open', async () => {
    setPlayList(await get_playlist());
  })

  async function stop() {
    await invoke("stop", {});
  }

  async function pause() {
    await invoke("pause", {});
  }

  async function play() {
    await invoke("play", {});
  }

  async function get_playlist(): Promise<ITrack[]> {
    return await invoke("get_playlist", {});
  }

  return (
    <div className="container">
      <div className="controls">
        <button className="control-button" onClick={() => play()}><img src={play_icon} /></button>
        <button className="control-button" onClick={() => pause()}><img src={pause_icon} /></button>
        <button className="control-button" onClick={() => stop()}><img src={stop_icon} /></button>
        <Volume></Volume>
      </div>

      <ProgressBar length={playList[0] ? playList[0].length : 0}></ProgressBar>

      <Playlist list={playList}></Playlist>

    </div >
  );
}

export default App;
