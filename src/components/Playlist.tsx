import ITrack from "../interfaces/track";
import { convertLength } from "../util/time";
import "./Playlist.css";

function Playlist({ list, currentTrack }: { list: ITrack[], currentTrack: number }) {
    return (
        <table className="playlist">
            <tr>
                <th>Title</th>
                <th>Artist</th>
                <th>Album</th>
                <th>Length</th>
            </tr>
            {
                list.map((track, index) => (
                    <tr className={index === currentTrack ? "selected" : ""}>
                        <td>{track.title}</td>
                        <td>{track.artist}</td>
                        <td>{track.album}</td>
                        <td>{convertLength(track.length)}</td>
                    </tr>
                ))
            }
        </table>
    )

}

export default Playlist;
