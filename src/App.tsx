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
  const [playing, setPlaying] = useState(false);

  listen('open', async () => {
    setPlayList(await get_playlist());
    setPlaying(true);
  })

  async function stop() {
    await invoke("stop", {});
    setPlaying(false);
  }

  async function pause() {
    await invoke("pause", {});
    setPlaying(false);
  }

  async function play() {
    await invoke("play", {});
    setPlaying(true);
  }

  async function get_playlist(): Promise<ITrack[]> {
    return await invoke("get_playlist", {});
  }

  return (
    <div className="container">
      <div className="controls">
        <button className="control-button" onClick={!playing ? () => play() : () => pause()}>
          <img src={!playing ? play_icon : pause_icon} />
        </button>
        <button className="control-button" onClick={() => stop()}><img src={stop_icon} /></button>
        <Volume></Volume>
      </div>

      <ProgressBar length={playList[0] ? playList[0].length : 0}></ProgressBar>

      <Playlist list={playList}></Playlist>

    </div >
  );
}

export default App;
