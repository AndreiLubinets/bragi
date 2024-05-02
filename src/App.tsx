import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { } from "@tauri-apps/api/window"
import play_icon from './assets/play.svg';
import stop_icon from './assets/stop.svg';
import pause_icon from './assets/pause.svg';
import previous_icon from './assets/previous.svg';
import next_icon from './assets/next.svg';

import Playlist from "./components/Playlist";
import ITrack from "./interfaces/track";
import { Event, listen } from "@tauri-apps/api/event";
import Volume from "./components/Volume";
import ProgressBar from "./components/ProgressBar";
import TrackInfo from "./components/TrackInfo";

function App() {
  const [playList, setPlayList] = useState<ITrack[]>([]);
  const [currentTrack, setCurrentTrack] = useState(Number);
  const [playing, setPlaying] = useState(false);

  listen('open', async () => {
    setPlayList(await getPlaylist());
  })

  listen('track_changed', async (event: Event<number>) => {
    setCurrentTrack(event.payload);
    setPlaying(true);
  })

  listen('playback_stopped', async () => {
    setPlaying(false);
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

  async function getPlaylist(): Promise<ITrack[]> {
    return await invoke("get_playlist", {});
  }

  async function next() {
    await invoke("next_track", {});
  }

  async function previous() {
    await invoke("previous_track", {});
  }

  return (
    <div className="container">
      <div className="container">
        <div className="controls">
          <button className="control-button" onClick={!playing ? () => play() : () => pause()}>
            <img src={!playing ? play_icon : pause_icon} />
          </button>
          <button className="control-button" onClick={() => stop()}><img src={stop_icon} /></button>
          <button className="control-button" onClick={() => previous()}><img src={previous_icon} /></button>
          <button className="control-button" onClick={() => next()}><img src={next_icon} /></button>
          <Volume></Volume>
        </div>

        <ProgressBar length={playList[currentTrack] ? playList[currentTrack].length : 0}></ProgressBar>

        <Playlist list={playList} currentTrack={currentTrack}></Playlist>

      </div >
      <TrackInfo track={playList[currentTrack]}></TrackInfo>
    </div>
  );
}

export default App;
