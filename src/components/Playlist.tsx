import ITrack from "../interfaces/track";
import "./Playlist.css";

function Playlist({ list }: { list: ITrack[] }) {

    return (
        <div className="playlist">
            <ul>
                {
                    list.map((track) => (
                        <li>{track.name}</li>
                    ))
                }
            </ul>
        </div>
    )

}

export default Playlist;