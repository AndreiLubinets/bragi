import ITrack from "../interfaces/track";
import "./Playlist.css";

function Playlist({ list }: { list: ITrack[] }) {

    function convertLength(length: number): string {
        let minutes = Math.floor(length / 60);
        let seconds = length % 60;
        return minutes + ":" + seconds;
    }

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
                    <tr className={index === 0 ? "selected" : ""}>
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