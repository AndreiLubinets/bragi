import { invoke } from "@tauri-apps/api/core";
import ITrack from "../interfaces/track";
import { convertLength } from "../util/time";
import "./Playlist.css";

function Playlist({ list, currentTrack }: { list: ITrack[], currentTrack: number }) {

    async function changeTrack(index: number) {
        await invoke("change_track", { index });
    }

    return (
        <table className="playlist">
            <tbody>
                <tr>
                    <th>Title</th>
                    <th>Artist</th>
                    <th>Album</th>
                    <th>Length</th>
                </tr>
                {
                    list.map((track, index) => (
                        <tr key={index} className={index === currentTrack ? "selected" : ""} onDoubleClick={() => changeTrack(index)}>
                            <td>{track.title}</td>
                            <td>{track.artist}</td>
                            <td>{track.album}</td>
                            <td>{convertLength(track.length)}</td>
                        </tr>
                    ))
                }
            </tbody>
        </table>
    )

}

export default Playlist;
