import "./Playlist.css";

function Playlist({ list }: { list: string[] }) {

    return (
        <div className="playlist">
            <ul>
                {
                    list.map((track) => (
                        <li>{track}</li>
                    ))
                }
            </ul>
        </div>
    )

}

export default Playlist;